use serde::{Deserialize, Serialize};

use crate::system::bus::keyboard::Keyboard;
use crate::AudioSink;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundGenerator {
    buffer: u8,
    registers: [u8; 14],
    selected_register: usize,
    chip_clock_now: u32,
    chip_clock_next_sample: f32,
    #[serde(skip)] // TODO: restore state as best as possible when deserializing
    psg: Option<psg::PSG>,
}

impl SoundGenerator {
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
                    "Writing to PSG register {:#04X}: {:#04X}",
                    self.selected_register,
                    self.buffer
                );

                self.registers[self.selected_register] = self.buffer;

                if let Some(psg) = &mut self.psg {
                    match self.selected_register {
                        0x00 | 0x01 => {
                            let period = (((self.registers[0x01] as u16) & 0x0f) << 8)
                                + self.registers[0x00] as u16;
                            psg.set_tone_period(0, period)
                        }
                        0x02 | 0x03 => {
                            let period = (((self.registers[0x03] as u16) & 0x0f) << 8)
                                + self.registers[0x02] as u16;
                            psg.set_tone_period(1, period)
                        }
                        0x04 | 0x05 => {
                            let period = (((self.registers[0x05] as u16) & 0x0f) << 8)
                                + self.registers[0x04] as u16;
                            psg.set_tone_period(2, period)
                        }
                        0x06 => {
                            let period = self.registers[0x06] & 0x1f;
                            psg.set_noise_period(period)
                        }
                        0x07 => {
                            // TODO: pause/resume audio stream as appropriate
                            for channel in 0..3 {
                                let tone_disabled = (self.registers[0x07] & (1 << channel)) != 0;
                                psg.set_tone_disabled(channel, tone_disabled);
                                let noise_disabled = (self.registers[0x07] & (8 << channel)) != 0;
                                psg.set_noise_disabled(channel, noise_disabled);
                            }
                        }
                        0x08 => {
                            let amplitude = self.registers[0x08] & 0x1f;
                            if amplitude < 0x10 {
                                psg.set_amplitude(0, amplitude);
                                psg.set_envelope_enabled(0, false)
                            } else {
                                psg.set_envelope_enabled(0, true)
                            }
                        }
                        0x09 => {
                            let amplitude = self.registers[0x09] & 0x1f;
                            if amplitude < 0x10 {
                                psg.set_amplitude(1, amplitude);
                                psg.set_envelope_enabled(1, false)
                            } else {
                                psg.set_envelope_enabled(1, true)
                            }
                        }
                        0x0a => {
                            let amplitude = self.registers[0x0a] & 0x1f;
                            if amplitude < 0x10 {
                                psg.set_amplitude(2, amplitude);
                                psg.set_envelope_enabled(2, false)
                            } else {
                                psg.set_envelope_enabled(2, true)
                            }
                        }
                        0x0b | 0x0c => {
                            let period =
                                ((self.registers[0x0c] as u16) << 8) + self.registers[0x0b] as u16;
                            psg.set_envelope_period(period);
                        }
                        0x0d => {
                            psg.set_envelope_shape(self.registers[0x0d]);
                        }
                        _ => {
                            unreachable!()
                        }
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

    pub fn step(&mut self, audio: &mut impl AudioSink) {
        self.chip_clock_now += 1;

        if let Some(sample_rate) = audio.get_sample_rate() {
            if self.psg.is_none() {
                self.psg = Some(psg::PSG::new(1_000_000_f64, sample_rate as u32).unwrap());
            }

            if self.chip_clock_now as f32 >= sample_rate {
                self.chip_clock_now = 0;
                self.chip_clock_next_sample = 0.0;
            }

            if self.chip_clock_now as f32 >= self.chip_clock_next_sample.floor() {
                self.chip_clock_next_sample += 1_000_000.0 / sample_rate;

                if let Some(psg) = &mut self.psg {
                    let (_left, right) = psg.render();
                    audio.add_sample(right as f32);
                }
            }
        }
    }
}
