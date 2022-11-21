use std::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::system::bus::keyboard::Keyboard;

const INVERSE_SQRT_2: f32 = 1.0 / std::f32::consts::SQRT_2;

const VOLUMES: [f32; 16] = [
    0.0,
    0.00999466,
    0.014450294,
    0.021057451,
    0.030701153,
    0.045548182,
    0.064499885,
    0.10736248,
    0.12658885,
    0.2049897,
    0.29221028,
    0.37283894,
    0.4925307,
    0.63532466,
    0.8055848,
    1.0,
];

pub struct SoundGenerator {
    buffer: u8,
    registers: [u8; 14],
    selected_register: usize,
    sample_queue: mpsc::Sender<f32>,
    audio_stream: Option<cpal::Stream>,
    sample_rate: Option<f32>,
    chip_clock_now: u32,
    chip_clock_next_sample: f32,
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
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let mut psg = SoundGenerator {
            buffer: 0,
            registers: [0; 14],
            selected_register: 0,
            sample_queue: tx,
            audio_stream: None,
            sample_rate: None,
            chip_clock_now: 0,
            chip_clock_next_sample: 0.0,
            sample_clock: 0.0,
            tone_frequency: [0.0; 3],
            noise_frequency: 0.0,
            tone_active: [false; 3],
            noise_active: [false; 3],
            channel_volume: [Some(0); 3],
            envelope_frequency: 0.0,
            envelope_shape_hold: false,
            envelope_shape_alternate: false,
            envelope_shape_attack: false,
            envelope_shape_continue: false,
            frames: 0,
            start: std::time::Instant::now(),
        };

        psg.init_audio_stream(rx);

        psg
    }

    pub fn perform_function(&mut self, keyboard: &Keyboard, function: u8) {
        match function {
            0 => (), // inactive
            1 => match self.selected_register {
                0x0e => {
                    self.buffer = keyboard.scan_active_line();
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
        self.chip_clock_now += 1;

        if let Some(sample_rate) = self.sample_rate {
            if self.chip_clock_now as f32 >= sample_rate {
                self.chip_clock_now = 0;
                self.chip_clock_next_sample = 0.0;
            }

            if self.chip_clock_now as f32 >= self.chip_clock_next_sample.floor() {
                self.chip_clock_next_sample += 1_000_000.0 / sample_rate;
                self.sample_clock = (self.sample_clock + 1.0) % sample_rate;
                let mut sample = 0f32;

                for channel in 0..3 {
                    if self.tone_active[channel] {
                        match self.channel_volume[channel] {
                            Some(volume_level) => {
                                let volume = INVERSE_SQRT_2.powi(15 - volume_level);
                                let volume = VOLUMES[volume_level as usize];
                                let volume = 0.33; // TODO: fix volume calculation
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
                            }
                            None => {
                                // TODO: Use volume envelope
                            }
                        }
                    }
                }

                if let Err(err) = self.sample_queue.send(sample) {
                    log::debug!("Error sending sample to audio thread: {}", err);
                }

                self.frames += 1;
                if self.start.elapsed().as_micros() >= 1_000_000 {
                    log::trace!("Generated {} audio samples per second", self.frames);
                    self.frames = 0;
                    self.start = std::time::Instant::now();
                }
            }
        }
    }

    fn init_audio_stream(&mut self, sample_queue: mpsc::Receiver<f32>) {
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
        sample_queue: mpsc::Receiver<f32>,
    ) where
        T: cpal::Sample,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut last_sample = 0.0;

        let mut start = std::time::Instant::now();
        let mut frames = 0;

        let err_fn = |err| log::error!("An error occurred on audio output stream: {}", err);

        match device.build_output_stream(
            config,
            move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in output.chunks_mut(channels) {
                    let next_sample = match sample_queue.try_recv() {
                        Ok(sample) => {
                            sample
                        }
                        Err(err) => {
                            log::trace!("Error fetching next audio sample batch: {}", err);
                            last_sample
                        }
                    };

                    let value: T = cpal::Sample::from::<f32>(&next_sample);
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }

                    last_sample = next_sample;

                    frames += 1;
                    if start.elapsed().as_micros() >= 1_000_000 {
                        log::trace!("Rendered {} audio samples per second", frames);
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
