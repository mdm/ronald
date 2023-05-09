use crate::system::bus::keyboard::Keyboard;
use crate::AudioSink;

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
    psg: Option<psg::PSG>,
}

impl SoundGenerator {
    pub fn new() -> Self {
        SoundGenerator {
            buffer: 0,
            registers: [0; 14],
            selected_register: 0,
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
            psg: None,
        }
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
                                psg.set_envelope_enabled(0, true)
                            } else {
                                psg.set_envelope_enabled(0, true)
                            }                            
                        }
                        0x09 => {
                            let amplitude = self.registers[0x09] & 0x1f;
                            if amplitude < 0x10 {
                                psg.set_amplitude(1, amplitude);
                                psg.set_envelope_enabled(1, true)
                            } else {
                                psg.set_envelope_enabled(1, true)
                            }                            
                        }
                        0x0a => {
                            let amplitude = self.registers[0x0a] & 0x1f;
                            if amplitude < 0x10 {
                                psg.set_amplitude(2, amplitude);
                                psg.set_envelope_enabled(2, true)
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
        if let Some(sample_rate) = audio.get_sample_rate() {
            if self.psg.is_none() {
                self.psg = Some(psg::PSG::new(1_000_000_f64, sample_rate as u32).unwrap());
            }

            if let Some(psg) = &mut self.psg {
                let (_left, right) = psg.render();
                audio.add_sample(right as f32);
            }
        }
    }
}
