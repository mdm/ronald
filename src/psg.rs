use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::keyboard;

pub type SoundGeneratorShared = Rc<RefCell<SoundGenerator>>;

const INVERSE_SQRT_2: f32 = 1.0 / std::f32::consts::SQRT_2;

pub struct SoundGenerator {
    keyboard: keyboard::KeyboardShared,
    buffer: u8,
    registers: [u8; 14],
    selected_register: usize,
    sample_batch: Vec<f32>,
    sample_queue: mpsc::Sender<Vec<f32>>,
    audio_stream: Option<cpal::Stream>,
    sample_rate: Option<f32>,
    chip_clock: u32,
    sample_clock: f32,
    tone_frequency: [f32; 3],
    noise_frequency: f32,
    tone_active: [bool; 3],
    noise_active: [bool; 3],
    channel_volume: [Option<i32>; 3],
    envelope_frequency: f32,
    envelope_shape_hold: bool,
    envelope_shape_alternate: bool,
    envelope_shape_attack: bool,
    envelope_shape_continue: bool,
    frames: u32,
    start: std::time::Instant,
}

impl SoundGenerator {
    pub fn new_shared(keyboard: keyboard::KeyboardShared) -> SoundGeneratorShared {
        let (tx, rx) = mpsc::channel();
        let mut psg = SoundGenerator {
            keyboard,
            buffer: 0,
            registers: [0; 14],
            selected_register: 0,
            sample_batch: Vec::new(),
            sample_queue: tx,
            audio_stream: None,
            sample_rate: None,
            chip_clock: 0,
            sample_clock: 0f32,
            tone_frequency: [0f32; 3],
            noise_frequency: 0f32,
            tone_active: [false; 3],
            noise_active: [false; 3],
            channel_volume: [Some(0i32); 3],
            envelope_frequency: 0f32,
            envelope_shape_hold: false,
            envelope_shape_alternate: false,
            envelope_shape_attack: false,
            envelope_shape_continue: false,
            frames: 0,
            start: std::time::Instant::now(),
        };

        psg.init_audio_stream(rx);

        Rc::new(RefCell::new(psg))
    }

    pub fn perform_function(&mut self, function: u8) {
        match function {
            0 => (), // inactive
            1 => match self.selected_register {
                0x0e => {
                    self.buffer = self.keyboard.borrow().scan_active_line();
                }
                _ => unimplemented!(),
            },
            2 => {
                log::trace!(
                    "Writing to PSG register {:#04x}: {:#04x}",
                    self.selected_register,
                    self.buffer
                );

                self.registers[self.selected_register] = self.buffer;

                match self.selected_register {
                    0x00 | 0x01 => {
                        let period = (((self.registers[0x01] as u16) & 0x0f) << 8)
                            + self.registers[0x00] as u16;
                        // TODO: adjust period if 0
                        self.tone_frequency[0] = 1_000_000.0 * 0.0625 / (period as f32);
                    }
                    0x02 | 0x03 => {
                        let period = (((self.registers[0x03] as u16) & 0x0f) << 8)
                            + self.registers[0x02] as u16;
                        // TODO: adjust period if 0
                        self.tone_frequency[1] = 1_000_000.0 * 0.0625 / (period as f32);
                    }
                    0x04 | 0x05 => {
                        let period = (((self.registers[0x05] as u16) & 0x0f) << 8)
                            + self.registers[0x04] as u16;
                        // TODO: adjust period if 0
                        self.tone_frequency[2] = 1_000_000.0 * 0.0625 / (period as f32);
                    }
                    0x06 => {
                        let period = self.registers[0x06] & 0x1f;
                        // TODO: adjust period if 0
                        self.noise_frequency = 1_000_000.0 * 0.0625 / (period as f32);
                    }
                    0x07 => {
                        // TODO: pause/resume audio stream as appropriate
                        for channel in 0..3 {
                            self.tone_active[channel] = (self.registers[0x07] & (1 << channel)) == 0;
                            self.noise_active[channel] = (self.registers[0x07] & (8 << channel)) == 0;
                        }
                        dbg!(&self.tone_active);
                    }
                    0x08 => {
                        if (self.registers[0x08] & 0x1f) < 0x10 {
                            self.channel_volume[0] = Some((self.registers[0x08] & 0x1f) as i32);
                        } else {
                            self.channel_volume[0] = None;
                        }
                    }
                    0x09 => {
                        if (self.registers[0x09] & 0x1f) < 0x10 {
                            self.channel_volume[1] = Some((self.registers[0x08] & 0x1f) as i32);
                        } else {
                            self.channel_volume[1] = None;
                        }
                    }
                    0x0a => {
                        if (self.registers[0x0a] & 0x1f) < 0x10 {
                            self.channel_volume[2] = Some((self.registers[0x08] & 0x1f) as i32);
                        } else {
                            self.channel_volume[2] = None;
                        }
                    }
                    0x0b | 0x0c => {
                        let period =
                            ((self.registers[0x0c] as u16) << 8) + self.registers[0x0b] as u16;
                        // TODO: adjust period if 0
                        self.envelope_frequency = 1_000_000.0 * 0.0625 / (period as f32);
                    }
                    0x0d => {
                        self.envelope_shape_hold = (self.registers[0x0d] & 1) != 0;
                        self.envelope_shape_alternate = (self.registers[0x0d] & 2) != 0;
                        self.envelope_shape_attack = (self.registers[0x0d] & 4) != 0;
                        self.envelope_shape_continue = (self.registers[0x0d] & 8) != 0;
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            3 => {
                self.selected_register = self.buffer as usize; // TODO: check bounds
            }
            _ => unimplemented!(),
        }
    }

    pub fn read_byte(&self) -> u8 {
        self.buffer
    }

    pub fn write_byte(&mut self, value: u8) {
        self.buffer = value;
    }

    pub fn step(&mut self) {
        self.chip_clock += 1;

        if let Some(sample_rate) = self.sample_rate {
            if self.chip_clock as f32 >= (1_000_000.0 / sample_rate).floor() {
                self.sample_clock = (self.sample_clock + 1.0) % sample_rate;
                let mut sample = 0f32;

                for channel in 0..3 {
                    if self.tone_active[channel] {
                        match self.channel_volume[channel] {
                            Some(volume_level) => {
                                let volume = INVERSE_SQRT_2.powi(15 - volume_level);
                                let square_wave = 0.5
                                    + (2.0 / std::f32::consts::PI)
                                        * (0..10)
                                            .into_iter()
                                            .map(|k| {
                                                (self.sample_clock
                                                    * self.tone_frequency[channel]
                                                    * (2.0 * k as f32 + 1.0)
                                                    * 2.0
                                                    * std::f32::consts::PI
                                                    / sample_rate)
                                                    .sin()
                                                    / (2.0 * k as f32 + 1.0)
                                            })
                                            .sum::<f32>();
                                sample += volume * square_wave;
                                sample = (self.sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin();
                            }
                            None => {
                                // TODO: Use volume envelope
                            }
                        }
                    }
                }

                self.sample_batch.push(sample);
                // self.sample_batch.push((self.sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin());

                if self.sample_batch.len() as f32 > sample_rate / 10.0 {
                    if let Err(err) = self.sample_queue.send(std::mem::take(&mut self.sample_batch)) {
                        log::debug!("Error sending sample to audio thread: {}", err);
                    }
                }

                self.frames += 1;
                if self.start.elapsed().as_micros() >= 1_000_000 {
                    log::trace!("Generated {} audio samples per second", self.frames);
                    self.frames = 0;
                    self.start = std::time::Instant::now();
                }

                self.chip_clock = 0;
            }
        }
    }

    fn init_audio_stream(&mut self, sample_queue: mpsc::Receiver<Vec<f32>>) {
        let host = cpal::default_host();

        match host.default_output_device() {
            Some(device) => match device.default_output_config() {
                Ok(config) => {
                    log::debug!("Default audio output config: {:?}", config);

                    match config.sample_format() {
                        cpal::SampleFormat::F32 => {
                            self.run_audio_stream::<f32>(&device, &config.into(), sample_queue)
                        }
                        cpal::SampleFormat::I16 => {
                            self.run_audio_stream::<i16>(&device, &config.into(), sample_queue)
                        }
                        cpal::SampleFormat::U16 => {
                            self.run_audio_stream::<u16>(&device, &config.into(), sample_queue)
                        }
                    }
                }
                Err(err) => {
                    log::error!("Error finding audio output config: {}", err);
                }
            },
            None => {
                log::error!("Failed to find audio output device");
            }
        }
    }

    fn run_audio_stream<T>(
        &mut self,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        sample_queue: mpsc::Receiver<Vec<f32>>,
    ) where
        T: cpal::Sample,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut sample_batch = Vec::new();
        let mut sample_index = 0;

        let mut count = 10;
        let mut start = std::time::Instant::now();
        let mut frames = 0;

        let mut sample_clock = 0.0;

        let err_fn = |err| log::error!("An error occurred on audio output stream: {}", err);

        match device.build_output_stream(
            config,
            move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                // for frame in output.chunks_mut(channels) {
                //     sample_clock = (sample_clock + 1.0) % sample_rate;
                //     let next_sample = (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin();
    
                //     let value: T = cpal::Sample::from::<f32>(&next_sample);
                //     for sample in frame.iter_mut() {
                //         *sample = value;
                //     }
                // }

                
                for frame in output.chunks_mut(channels) {
                    if sample_index >= sample_batch.len() {
                        match sample_queue.recv() {
                            Ok(next_batch) => {
                                sample_batch = next_batch;
                                sample_index = 0;
                                log::trace!("New batch with {} items", sample_batch.len());
                            }
                            Err(err) => {
                                log::trace!("Error fetching next audio sample batch: {}", err);
                            }
                        }
                    }

                    let value: T = cpal::Sample::from::<f32>(&sample_batch[sample_index]);
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }

                    sample_index += 1;

                    frames += 1;
                    if start.elapsed().as_micros() >= 1_000_000 {
                        log::trace!("Rendered {} audio samples per second", frames);
                        log::trace!("Sample rate: {}", sample_rate);
                        frames = 0;
                        start = std::time::Instant::now();
                    }
                }
            },
            err_fn,
        ) {
            Ok(audio_stream) => {
                self.audio_stream = Some(audio_stream);
                self.sample_rate = Some(sample_rate);
            }
            Err(err) => {
                log::error!("Error initializing audio stream: {}", err);
            }
        }

        self.play_audio();
    }

    fn play_audio(&self) {
        match &self.audio_stream {
            Some(audio_stream) => match audio_stream.play() {
                Ok(_) => {
                    log::trace!("Playing audio");
                }
                Err(err) => {
                    log::error!("Error starting audio stream: {}", err)
                }
            },
            None => {
                log::debug!("Cannot start uninitialized audio stream");
            }
        }
    }

    fn pause_audio(&self) {
        match &self.audio_stream {
            Some(audio_stream) => match audio_stream.pause() {
                Ok(_) => {
                    log::trace!("Pausing audio");
                }
                Err(err) => {
                    log::error!("Error stopping audio stream: {}", err)
                }
            },
            None => {
                log::debug!("Cannot stop uninitialized audio stream");
            }
        }
    }
}
