use std::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use web_sys::{CanvasRenderingContext2d, ImageData};

use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};
use ronald_core::{AudioSink, VideoSink};

pub struct CanvasVideo {
    ctx: CanvasRenderingContext2d,
}

impl CanvasVideo {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx }
    }
}

impl VideoSink for CanvasVideo {
    fn draw_frame(&mut self, buffer: &Vec<(u8, u8, u8)>) {
        let mut frame = vec![0u8; 4 * SCREEN_BUFFER_WIDTH * SCREEN_BUFFER_HEIGHT];
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = buffer[i].0; // R
            pixel[1] = buffer[i].1; // G
            pixel[2] = buffer[i].2; // B
            pixel[3] = 255; // A
        }

        let image_data = match ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(frame.as_mut_slice()),
            SCREEN_BUFFER_WIDTH as u32,
            SCREEN_BUFFER_HEIGHT as u32,
        ) {
            Ok(image_data) => image_data,
            Err(_) => return,
        };
        self.ctx.put_image_data(&image_data, 0.0, 0.0);
    }
}

pub struct WebAudio {
    sample_queue: mpsc::Sender<f32>,
    audio_stream: Option<cpal::Stream>,
    sample_rate: Option<f32>,
}

impl WebAudio {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let mut web_audio = WebAudio {
            sample_queue: tx,
            audio_stream: None,
            sample_rate: None,
        };

        web_audio.init_audio_stream(rx);

        web_audio
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
                        sample_format => {
                            log::error!("Unsupported sample format: {:?}", sample_format);
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
        T: cpal::SizedSample + cpal::FromSample<f32>
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut last_sample = 0.0;

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

                    let value: T = cpal::Sample::from_sample::<f32>(next_sample);
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }

                    last_sample = next_sample;
                }
            },
            err_fn,
            None,
        ) {
            Ok(audio_stream) => {
                self.audio_stream = Some(audio_stream);
                self.sample_rate = Some(sample_rate);
            }
            Err(err) => {
                log::error!("Error initializing audio stream: {}", err);
            }
        }
    }
}

impl AudioSink for WebAudio {
    fn get_sample_rate(&self) -> Option<f32> {
        self.sample_rate
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

    fn add_sample(&self, sample: f32) {
        if let Err(err) = self.sample_queue.send(sample) {
            log::debug!("Error sending sample to audio thread: {}", err);
        }
    }
}
