use std::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use ronald_core::AudioSink;

pub struct CpalAudio {
    sample_queue: mpsc::Sender<f32>,
    audio_stream: Option<cpal::Stream>,
    sample_rate: Option<f32>,
}

impl CpalAudio {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let mut cpal_audio = CpalAudio {
            sample_queue: tx,
            audio_stream: None,
            sample_rate: None,
        };

        cpal_audio.init_audio_stream(rx);

        cpal_audio
    }

    fn init_audio_stream(&mut self, sample_queue: mpsc::Receiver<f32>) {
        let host = cpal::default_host();

        match host.default_output_device() {
            Some(device) => match device.default_output_config() {
                Ok(config) => {
                    log::debug!("Default audio output config: {config:?}");

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
                            log::error!("Unsupported sample format: {sample_format:?}");
                        }
                    }
                }
                Err(err) => {
                    log::error!("Error finding audio output config: {err}");
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
        T: cpal::SizedSample + cpal::FromSample<f32>,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut last_sample = 0.0;

        let mut start = std::time::Instant::now();
        let mut frames = 0;
        let mut repeated_frames = 0;

        let err_fn = |err| log::error!("An error occurred on audio output stream: {err}");

        match device.build_output_stream(
            config,
            move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in output.chunks_mut(channels) {
                    let next_sample = match sample_queue.try_recv() {
                        Ok(sample) => sample,
                        Err(err) => {
                            log::trace!("Error fetching next audio sample batch: {err}");
                            repeated_frames += 1;
                            last_sample
                        }
                    };

                    let value: T = cpal::Sample::from_sample::<f32>(next_sample);
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }

                    last_sample = next_sample;

                    frames += 1;
                }

                if start.elapsed().as_micros() >= 1_000_000 {
                    log::debug!(
                        "Rendered {frames} audio samples per second ({repeated_frames} repeated)"
                    );
                    start = std::time::Instant::now();
                    frames = 0;
                    repeated_frames = 0;
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
                log::error!("Error initializing audio stream: {err}");
            }
        }

        self.play_audio();
    }
}

impl AudioSink for CpalAudio {
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
                    log::error!("Error starting audio stream: {err}")
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
                    log::error!("Error stopping audio stream: {err}")
                }
            },
            None => {
                log::debug!("Cannot stop uninitialized audio stream");
            }
        }
    }

    fn add_sample(&self, sample: f32) {
        if let Err(err) = self.sample_queue.send(sample) {
            log::debug!("Error sending sample to audio thread: {err}");
        }
    }
}
