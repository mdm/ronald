use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::keyboard;

pub type SoundGeneratorShared = Rc<RefCell<SoundGenerator>>;

pub struct SoundGenerator {
    keyboard: keyboard::KeyboardShared,
    buffer: u8,
    selected_register: u8,
    message_channel: mpsc::Sender<SoundMessage>,
    audio_stream: Option<cpal::Stream>,
}

impl SoundGenerator {
    pub fn new_shared(keyboard: keyboard::KeyboardShared) -> SoundGeneratorShared {
        let (tx, rx) = mpsc::channel();
        let mut psg = SoundGenerator {
            keyboard,
            buffer: 0,
            selected_register: 0,
            message_channel: tx,
            audio_stream: None,
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
                ); // TODO: process data
            }
            3 => {
                self.selected_register = self.buffer;
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

    fn init_audio_stream(&mut self, message_channel: mpsc::Receiver<SoundMessage>) {
        let host = cpal::default_host();

        match host.default_output_device() {
            Some(device) => match device.default_output_config() {
                Ok(config) => {
                    log::debug!("Default audio output config: {:?}", config);

                    match config.sample_format() {
                        cpal::SampleFormat::F32 => {
                            self.run_audio_stream::<f32>(&device, &config.into(), message_channel)
                        }
                        cpal::SampleFormat::I16 => {
                            self.run_audio_stream::<i16>(&device, &config.into(), message_channel)
                        }
                        cpal::SampleFormat::U16 => {
                            self.run_audio_stream::<u16>(&device, &config.into(), message_channel)
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

    fn run_audio_stream<T>(&mut self, device: &cpal::Device, config: &cpal::StreamConfig, message_channel: mpsc::Receiver<SoundMessage>)
    where
        T: cpal::Sample,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut tone_frequency = [0f32; 3];
        let mut noise_frequency = 0f32;
        let mut tone_active = [false; 3];
        let mut noise_active = [false; 3];
        let mut channel_volume = [Some(0f32); 3];
        let mut envelope_frequency = 0f32;
        let mut envelope_shape_hold = false;
        let mut envelope_shape_alternate = false;
        let mut envelope_shape_attack = false;
        let mut envelope_shape_continue = false;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;
        let mut next_sample = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            let mut sample = 0f32;

            for channel in 0..3 {
                if tone_active[channel] {
                    match channel_volume[channel] {
                        Some(volume) => {
                            sample += volume * (sample_clock * tone_frequency[channel] * 2.0 * std::f32::consts::PI / sample_rate).sin();
                        }
                        None => {
                            // TODO: Use volume envelope
                        }
                    }
                }
            }
            
            sample
        };

        let err_fn = |err| log::error!("An error occurred on audio output stream: {}", err);

        match device.build_output_stream(
            config,
            move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                if let Ok(message) = message_channel.try_recv() {
                }

                for frame in output.chunks_mut(channels) {
                    let value: T = cpal::Sample::from::<f32>(&next_sample());
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
        ) {
            Ok(audio_stream) => {
                self.audio_stream = Some(audio_stream);
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

enum SoundMessage {
    ToneFrequency(usize, f32),
    NoiseFrequency(f32),
    MixerControl(u8),
    ChannelVolume(usize, Option<f32>),
    EnvelopeFrequency(f32),
    EnvelopeShape(u8),
}
