use serde::{Deserialize, Serialize};

use crate::system::bus::crtc;
use crate::system::bus::screen;
use crate::system::memory;
use crate::system::memory::Mmu;
use crate::VideoSink;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GateArray {
    current_screen_mode: u8,
    requested_screen_mode: u8,
    hsync_active: bool,
    vsync_active: bool,
    hsyncs_since_last_vsync: u8,
    interrupt_counter: u8,
    hold_interrupt: bool,
    selected_pen: usize,
    pen_colors: Vec<u8>,
}

impl GateArray {
    pub fn new() -> Self {
        GateArray {
            current_screen_mode: 0,
            requested_screen_mode: 0,
            hsync_active: false,
            vsync_active: false,
            hsyncs_since_last_vsync: 0,
            interrupt_counter: 0,
            hold_interrupt: false,
            selected_pen: 0,
            pen_colors: vec![0; 17], // 16 pens + border
        }
    }

    pub fn write_byte(&mut self, memory: &mut impl Mmu, port: u16, value: u8) {
        // TODO: remove port parameter?
        let function = (value >> 6) & 0x03;

        match function {
            0 => {
                if value & 0x10 == 0 {
                    self.selected_pen = (value & 0x0f) as usize; // TODO: what if pen number exceeds max. number of pens for mode?
                } else {
                    self.selected_pen = 0x10; // select border
                }
            }
            1 => {
                log::trace!(
                    "Gate Array color select (pen {}): {:#04x} ({:#04x})",
                    self.selected_pen,
                    value,
                    value & 0x1f
                );
                self.pen_colors[self.selected_pen] = value & 0x1f;
            }
            2 => {
                self.requested_screen_mode = value & 0x03;

                memory.enable_lower_rom(value & 0x04 == 0);
                memory.enable_upper_rom(value & 0x08 == 0);

                if value & 0x10 != 0 {
                    self.interrupt_counter = 0;
                }
            }
            3 => {
                // ROM banking (only available in CPC 6128)
                // TODO: show error message to user
                log::error!("Gate Array ROM banking not supported: {value:#010b}");
                unimplemented!();
            }
            _ => {
                unreachable!();
            }
        }
    }

    pub fn acknowledge_interrupt(&mut self) {
        self.interrupt_counter &= 0x1f;
    }

    pub fn step(
        &mut self,
        crtc: &dyn crtc::CrtController,
        memory: &memory::Memory,
        screen: &mut screen::Screen,
        video: &mut impl VideoSink,
    ) -> bool {
        let generate_interrupt = self.update_interrupt_counter(crtc);
        self.update_screen_mode(crtc);
        self.write_to_screen(crtc, memory, screen, video);

        self.hsync_active = crtc.read_horizontal_sync();
        self.vsync_active = crtc.read_vertical_sync();

        generate_interrupt
    }

    fn update_interrupt_counter(&mut self, crtc: &dyn crtc::CrtController) -> bool {
        let mut generate_interrupt = false;
        if self.hsync_active && !crtc.read_horizontal_sync() {
            self.interrupt_counter += 1;
            self.hsyncs_since_last_vsync += 1;

            if self.interrupt_counter == 52 {
                self.interrupt_counter = 0;
                generate_interrupt = true;
            }
        }

        if !self.vsync_active && crtc.read_vertical_sync() {
            self.hsyncs_since_last_vsync = 0;
            self.hold_interrupt = true;
        }

        if self.hold_interrupt && self.hsyncs_since_last_vsync == 2 {
            self.interrupt_counter = 0;
            self.hold_interrupt = false;
            generate_interrupt = self.interrupt_counter & 0x20 == 0;
        }

        generate_interrupt
    }

    fn update_screen_mode(&mut self, crtc: &dyn crtc::CrtController) {
        if !self.hsync_active && crtc.read_horizontal_sync() {
            self.current_screen_mode = self.requested_screen_mode;
            log::trace!("New screen mode: {}", self.current_screen_mode);
        }
    }

    fn write_to_screen(
        &self,
        crtc: &dyn crtc::CrtController,
        memory: &memory::Memory,
        screen: &mut screen::Screen,
        video: &mut impl VideoSink,
    ) {
        if !self.vsync_active && crtc.read_vertical_sync() {
            screen.trigger_vsync(video);
        }

        if crtc.read_horizontal_sync() || crtc.read_vertical_sync() {
            // TODO: use modified hsync/vsync durations (see http://www.cpcwiki.eu/index.php?title=CRTC#HSYNC_and_VSYNC)
            for _ in 0..16 {
                // screen.write(20); // black
                screen.write(self.pen_colors[0x10] as usize); // border // TODO: remove this workaround for the lengthened vsync problem
            }
            return;
        }

        if !crtc.read_display_enabled() {
            for _ in 0..16 {
                screen.write(self.pen_colors[0x10] as usize); // border
            }
            return;
        }

        for offset in 0..2 {
            let address = crtc.read_address() + offset;
            let packed = memory.read_byte_from_ram(address);
            match self.current_screen_mode {
                0 => {
                    let pixels = [
                        ((packed & 0x80) >> 7)
                            | ((packed & 0x08) >> 2)
                            | ((packed & 0x20) >> 3)
                            | ((packed & 0x02) << 2),
                        ((packed & 0x40) >> 6)
                            | ((packed & 0x04) >> 1)
                            | ((packed & 0x10) >> 2)
                            | ((packed & 0x01) << 3),
                    ];

                    for pixel in pixels {
                        for _ in 0..4 {
                            screen.write(self.pen_colors[pixel as usize] as usize);
                        }
                    }
                }
                1 => {
                    let pixels = [
                        ((packed & 0x80) >> 7) | ((packed & 0x08) >> 2),
                        ((packed & 0x40) >> 6) | ((packed & 0x04) >> 1),
                        ((packed & 0x20) >> 5) | (packed & 0x02),
                        ((packed & 0x10) >> 4) | ((packed & 0x01) << 1),
                    ];

                    for pixel in pixels {
                        for _ in 0..2 {
                            screen.write(self.pen_colors[pixel as usize] as usize);
                        }
                    }
                }
                2 => {
                    for bit in 0..8 {
                        let pixel = (packed >> (7 - bit)) & 1;
                        screen.write(self.pen_colors[pixel as usize] as usize);
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}
