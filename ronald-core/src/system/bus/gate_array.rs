use serde::{Deserialize, Serialize};

use crate::VideoSink;
use crate::debug::DebugSource;
use crate::debug::Debuggable;
use crate::debug::Snapshottable;
use crate::debug::event::GateArrayDebugEvent;
use crate::debug::view::GateArrayDebugView;
use crate::system::bus::crtc;
use crate::system::bus::screen;
use crate::system::clock::MasterClockTick;
use crate::system::memory::MemManage;
use crate::system::memory::MemRead;

pub trait GateArray: Default {
    fn write_byte(&mut self, memory: &mut impl MemManage, port: u16, value: u8);
    fn acknowledge_interrupt(&mut self);
    fn step(
        &mut self,
        crtc: &impl crtc::CrtController,
        memory: &mut (impl MemRead + MemManage),
        screen: &mut screen::Screen,
        video: &mut impl VideoSink,
        master_clock: MasterClockTick,
    ) -> bool;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amstrad40007 {
    current_screen_mode: u8,
    requested_screen_mode: Option<u8>,
    hsync_active: bool,
    vsync_active: bool,
    hsyncs_since_last_vsync: u16,
    interrupt_counter: u8,
    hold_interrupt: bool,
    selected_pen: usize,
    pen_colors: Vec<u8>,
    master_clock: MasterClockTick,
}

impl Default for Amstrad40007 {
    fn default() -> Self {
        Amstrad40007 {
            current_screen_mode: 0,
            requested_screen_mode: None,
            hsync_active: false,
            vsync_active: false,
            hsyncs_since_last_vsync: 0,
            interrupt_counter: 0,
            hold_interrupt: false,
            selected_pen: 0,
            pen_colors: vec![0; 17], // 16 colors + border
            master_clock: MasterClockTick::default(),
        }
    }
}

impl Amstrad40007 {
    fn update_interrupt_counter(&mut self, crtc: &impl crtc::CrtController) -> bool {
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

        if generate_interrupt {
            self.emit_debug_event(GateArrayDebugEvent::InterruptGenerated, self.master_clock);
        }

        generate_interrupt
    }

    fn update_screen_mode(&mut self, crtc: &impl crtc::CrtController) {
        if !self.hsync_active && crtc.read_horizontal_sync() {
            if let Some(requested) = self.requested_screen_mode.take() {
                let was = self.current_screen_mode;
                self.current_screen_mode = requested;
                log::trace!("New screen mode: {}", self.current_screen_mode);
                self.emit_debug_event(
                    GateArrayDebugEvent::ScreenModeChanged {
                        is: self.current_screen_mode,
                        was,
                        applied: true,
                    },
                    self.master_clock,
                );
            }
        }
    }

    fn write_to_screen(
        &self,
        crtc: &impl crtc::CrtController,
        memory: &mut (impl MemRead + MemManage),
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

        memory.force_ram_read(true);
        for offset in 0..2 {
            let address = crtc.read_address() + offset;
            let packed = memory.read_byte(address);
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
        memory.force_ram_read(false);
    }
}

impl GateArray for Amstrad40007 {
    fn write_byte(&mut self, memory: &mut impl MemManage, port: u16, value: u8) {
        // TODO: remove port parameter?
        let function = (value >> 6) & 0x03;

        match function {
            0 => {
                if value & 0x10 == 0 {
                    self.selected_pen = (value & 0x0f) as usize; // TODO: what if pen number exceeds max. number of pens for mode?
                } else {
                    self.selected_pen = 0x10; // select border
                }
                self.emit_debug_event(
                    GateArrayDebugEvent::PenSelected {
                        pen: self.selected_pen,
                    },
                    self.master_clock,
                );
            }
            1 => {
                log::trace!(
                    "Gate Array color select (pen {}): {:#04x} ({:#04x})",
                    self.selected_pen,
                    value,
                    value & 0x1f
                );
                let was = self.pen_colors[self.selected_pen];
                self.pen_colors[self.selected_pen] = value & 0x1f;
                self.emit_debug_event(
                    GateArrayDebugEvent::PenColorChanged {
                        pen: self.selected_pen,
                        is: value & 0x1f,
                        was,
                    },
                    self.master_clock,
                );
            }
            2 => {
                let new_mode = value & 0x03;
                let was = self
                    .requested_screen_mode
                    .unwrap_or(self.current_screen_mode);
                self.requested_screen_mode = Some(new_mode);

                // Emit requested event
                self.emit_debug_event(
                    GateArrayDebugEvent::ScreenModeChanged {
                        is: new_mode,
                        was,
                        applied: false,
                    },
                    self.master_clock,
                );

                let lower_rom_enabled = value & 0x04 == 0;
                let upper_rom_enabled = value & 0x08 == 0;

                memory.enable_lower_rom(lower_rom_enabled);
                memory.enable_upper_rom(upper_rom_enabled);

                self.emit_debug_event(
                    GateArrayDebugEvent::RomConfigChanged {
                        lower_rom_enabled,
                        upper_rom_enabled,
                    },
                    self.master_clock,
                );

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

    fn acknowledge_interrupt(&mut self) {
        self.interrupt_counter &= 0x1f;
    }

    fn step(
        &mut self,
        crtc: &impl crtc::CrtController,
        memory: &mut (impl MemRead + MemManage),
        screen: &mut screen::Screen,
        video: &mut impl VideoSink,
        master_clock: MasterClockTick,
    ) -> bool {
        self.master_clock = master_clock;

        let generate_interrupt = self.update_interrupt_counter(crtc);
        self.update_screen_mode(crtc);
        self.write_to_screen(crtc, memory, screen, video);

        self.hsync_active = crtc.read_horizontal_sync();
        self.vsync_active = crtc.read_vertical_sync();

        generate_interrupt
    }
}

impl Snapshottable for Amstrad40007 {
    type View = GateArrayDebugView;

    fn debug_view(&self) -> Self::View {
        GateArrayDebugView {
            current_screen_mode: self.current_screen_mode,
            requested_screen_mode: self.requested_screen_mode,
            hsync_active: self.hsync_active,
            vsync_active: self.vsync_active,
            hsyncs_since_last_vsync: self.hsyncs_since_last_vsync,
            interrupt_counter: self.interrupt_counter,
            hold_interrupt: self.hold_interrupt,
            selected_pen: self.selected_pen,
            pen_colors: self.pen_colors.clone(),
        }
    }
}

impl Debuggable for Amstrad40007 {
    const SOURCE: DebugSource = DebugSource::GateArray;
    type Event = GateArrayDebugEvent;
}

#[derive(Serialize, Deserialize)]
pub enum AnyGateArray {
    Amstrad40007(Amstrad40007),
}

impl Default for AnyGateArray {
    fn default() -> Self {
        AnyGateArray::Amstrad40007(Amstrad40007::default())
    }
}

impl GateArray for AnyGateArray {
    fn write_byte(&mut self, memory: &mut impl MemManage, port: u16, value: u8) {
        match self {
            AnyGateArray::Amstrad40007(gate_array) => gate_array.write_byte(memory, port, value),
        }
    }

    fn acknowledge_interrupt(&mut self) {
        match self {
            AnyGateArray::Amstrad40007(gate_array) => gate_array.acknowledge_interrupt(),
        }
    }

    fn step(
        &mut self,
        crtc: &impl crtc::CrtController,
        memory: &mut (impl MemRead + MemManage),
        screen: &mut screen::Screen,
        video: &mut impl VideoSink,
        master_clock: MasterClockTick,
    ) -> bool {
        match self {
            AnyGateArray::Amstrad40007(gate_array) => {
                gate_array.step(crtc, memory, screen, video, master_clock)
            }
        }
    }
}

impl Snapshottable for AnyGateArray {
    type View = GateArrayDebugView;

    fn debug_view(&self) -> Self::View {
        match self {
            AnyGateArray::Amstrad40007(gate_array) => gate_array.debug_view(),
        }
    }
}
