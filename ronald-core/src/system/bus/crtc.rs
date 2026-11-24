use std::fmt;
use std::ops::{Index, IndexMut};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use crate::debug::event::CrtcDebugEvent;
use crate::debug::view::CrtcDebugView;
use crate::debug::{DebugSource, Debuggable, Snapshottable};
use crate::system::CrtcType;
use crate::system::clock::MasterClockTick;

pub trait CrtControllerInterface: Default {
    fn read_byte(&mut self, port: u16) -> u8;
    fn write_byte(&mut self, port: u16, value: u8);
    fn step(&mut self, master_clock: MasterClockTick);
    fn read_address(&self) -> usize;
    fn read_display_enabled(&self) -> bool;
    fn read_horizontal_sync(&self) -> bool;
    fn read_vertical_sync(&self) -> bool;
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    IntoPrimitive,
    TryFromPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(usize)]
pub enum Register {
    #[default]
    R0HorizontalTotal,
    R1HorizontalDisplayed,
    R2HorizontalSyncPosition,
    R3HorizontalAndVerticalSyncWidths,
    R4VerticalTotal,
    R5VerticalTotalAdjust,
    R6VerticalDisplayed,
    R7VerticalSyncPosition,
    R8InterlaceAndSkew,
    R9MaximumRasterAddress,
    R10CursorStartRaster,
    R11CursorEndRaster,
    R12DisplayStartAddressHigh,
    R13DisplayStartAddressLow,
    R14CursorAddressHigh,
    R15CursorAddressLow,
    R16LightPenAddressHigh,
    R17LightPenAddressLow,
    #[num_enum(alternatives = [19..31])]
    Unused,
    R31Dummy = 31,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::R0HorizontalTotal => write!(f, "R{} (Horizontal Total)", usize::from(*self)),
            Register::R1HorizontalDisplayed => {
                write!(f, "R{} (Horizontal Displayed)", usize::from(*self))
            }
            Register::R2HorizontalSyncPosition => {
                write!(f, "R{} (H. Sync Position)", usize::from(*self))
            }
            Register::R3HorizontalAndVerticalSyncWidths => {
                write!(f, "R{} (H/V Sync Widths)", usize::from(*self))
            }
            Register::R4VerticalTotal => write!(f, "R{} (Vertical Total)", usize::from(*self)),
            Register::R5VerticalTotalAdjust => {
                write!(f, "R{} (V. Total Adjust)", usize::from(*self))
            }
            Register::R6VerticalDisplayed => {
                write!(f, "R{} (Vertical Displayed)", usize::from(*self))
            }
            Register::R7VerticalSyncPosition => {
                write!(f, "R{} (V. Sync Position)", usize::from(*self))
            }
            Register::R8InterlaceAndSkew => write!(f, "R{} (Interlace/Skew)", usize::from(*self)),
            Register::R9MaximumRasterAddress => {
                write!(f, "R{} (Max Raster Address)", usize::from(*self))
            }
            Register::R10CursorStartRaster => write!(f, "R{} (Cursor Start)", usize::from(*self)),
            Register::R11CursorEndRaster => write!(f, "R{} (Cursor End)", usize::from(*self)),
            Register::R12DisplayStartAddressHigh => {
                write!(f, "R{} (Display Start High)", usize::from(*self))
            }
            Register::R13DisplayStartAddressLow => {
                write!(f, "R{} (Display Start Low)", usize::from(*self))
            }
            Register::R14CursorAddressHigh => {
                write!(f, "R{} (Cursor Address High)", usize::from(*self))
            }
            Register::R15CursorAddressLow => {
                write!(f, "R{} (Cursor Address Low)", usize::from(*self))
            }
            Register::R16LightPenAddressHigh => {
                write!(f, "R{} (Light Pen High)", usize::from(*self))
            }
            Register::R17LightPenAddressLow => write!(f, "R{} (Light Pen Low)", usize::from(*self)),
            Register::Unused => write!(f, "R18-R30 (Unused)"),
            Register::R31Dummy => write!(f, "R{} (Dummy)", usize::from(*self)),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterFile([u8; 20]);

impl RegisterFile {
    fn lookup(&self, register: Register) -> usize {
        match register {
            Register::Unused => 18,
            Register::R31Dummy => 19,
            _ => usize::from(register),
        }
    }
}

impl Index<Register> for RegisterFile {
    type Output = u8;

    fn index(&self, index: Register) -> &Self::Output {
        &self.0[self.lookup(index)]
    }
}

impl IndexMut<Register> for RegisterFile {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.0[self.lookup(index)]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Function {
    Ignore,
    Select,
    Write,
    Status,
    Read,
}

impl From<u16> for Function {
    fn from(port: u16) -> Self {
        if port & 0x4000 != 0 {
            return Function::Ignore;
        }

        match (port >> 8) & 0x03 {
            0 => Function::Select,
            1 => Function::Write,
            2 => Function::Status,
            3 => Function::Read,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CrtController<I>
where
    I: CrtControllerImpl,
{
    registers: RegisterFile,
    selected_register: u8,
    c0_horizontal_counter: u8,
    c3l_horizontal_sync_width_counter: u8,
    c4_character_row_counter: u8,
    c9_scan_line_counter: u8,
    video_memory_address: u16,
    video_memory_address_initial: u16,
    master_clock: MasterClockTick,
    frame: usize,
    previous_hsync: bool,
    previous_vsync: bool,
    active_vsync_scanlines: u8,
    previous_display_enabled: bool,
    previous_address: usize,
    border_r1_condition: bool,
    border_r6_condition: bool,
    impl_: I,
}

impl<I> Snapshottable for CrtController<I>
where
    I: CrtControllerImpl,
{
    type View = CrtcDebugView;

    fn debug_view(&self) -> Self::View {
        CrtcDebugView {
            registers: self.registers.clone(),
            selected_register_read: I::resolve_selected_register_read(self)
                .expect("masked register index is always valid"),
            selected_register_write: I::resolve_selected_register_write(self)
                .expect("masked register index is always valid"),
            horizontal_counter: self.c0_horizontal_counter,
            character_row_counter: self.c4_character_row_counter,
            scan_line_counter: self.c9_scan_line_counter,
            video_memory_address: self.video_memory_address,
            video_memory_address_initial: self.video_memory_address_initial,
            hsync_active: self.read_horizontal_sync(),
            vsync_active: self.read_vertical_sync(),
            display_enabled: self.read_display_enabled(),
            current_address: self.read_address(),
        }
    }
}

impl<I> Debuggable for CrtController<I>
where
    I: CrtControllerImpl,
{
    const SOURCE: DebugSource = DebugSource::Crtc;
    type Event = CrtcDebugEvent;
}

impl<I> CrtControllerInterface for CrtController<I>
where
    I: CrtControllerImpl,
{
    fn read_byte(&mut self, port: u16) -> u8 {
        match port.into() {
            Function::Read => I::read_register(self),
            Function::Status => I::read_status(self),
            _ => 0xff, // TODO: properly emulate floating bus
        }
    }

    fn write_byte(&mut self, port: u16, value: u8) {
        match port.into() {
            Function::Select => I::select_register(self, value),
            Function::Write => I::write_register(self, value),
            _ => (),
        }
    }

    fn step(&mut self, master_clock: MasterClockTick) {
        // TODO: make sure this is stepped in increments of 0.5 usecs (i.e. 8 MasterClockTicks not 16),
        // so we can correctly emulate the 2-phase clock of the CRTC. This will allow us to cancel
        // the border_r6_condition at the right time.
        self.master_clock = master_clock;

        let horizontal_counter_was = self.c0_horizontal_counter;
        let scan_line_was = self.c9_scan_line_counter;
        let character_row_was = self.c4_character_row_counter;

        self.c0_horizontal_counter += 1;

        if self.c0_horizontal_counter > self.registers[Register::R0HorizontalTotal] {
            self.c9_scan_line_counter += 1;
            self.c0_horizontal_counter = 0;
        }

        if self.c9_scan_line_counter > self.registers[Register::R9MaximumRasterAddress] {
            self.c4_character_row_counter += 1;
            self.c9_scan_line_counter = 0;
        }

        if self.c4_character_row_counter > self.registers[Register::R4VerticalTotal] {
            // TODO: take VerticalTotalAdjust into account
            self.c4_character_row_counter = 0;
        }

        if self.c0_horizontal_counter == 0 {
            if self.active_vsync_scanlines > 0 {
                self.active_vsync_scanlines += 1;
                if self.active_vsync_scanlines > I::vsync_length(self) {
                    self.active_vsync_scanlines = 0;
                }
            } else if self.c9_scan_line_counter == 0
                && self.c4_character_row_counter == self.registers[Register::R7VerticalSyncPosition]
            {
                self.active_vsync_scanlines = 1;
            }
        }

        self.emit_debug_event(
            CrtcDebugEvent::CountersChanged {
                character_row_is: self.c4_character_row_counter,
                character_row_was,
                scan_line_is: self.c9_scan_line_counter,
                scan_line_was,
                horizontal_counter_is: self.c0_horizontal_counter,
                horizontal_counter_was,
            },
            master_clock,
        );

        I::update_video_memory_address(self);

        // TODO: I::update_border_r1_condition(self);
        I::update_border_r6_condition(self);

        if self.c0_horizontal_counter == 0
            && self.c4_character_row_counter == 0
            && self.c9_scan_line_counter == 0
        {
            self.frame += 1;
        }

        I::update_status(self);

        // Check for sync state changes
        let new_hsync = self.read_horizontal_sync();
        let new_vsync = self.read_vertical_sync();
        let new_display_enabled = self.read_display_enabled();
        let new_address = self.read_address();

        if new_hsync != self.previous_hsync {
            self.emit_debug_event(
                CrtcDebugEvent::HorizontalSync { enabled: new_hsync },
                master_clock,
            );
            self.previous_hsync = new_hsync;
        }

        if new_vsync != self.previous_vsync {
            self.emit_debug_event(
                CrtcDebugEvent::VerticalSync { enabled: new_vsync },
                master_clock,
            );
            self.previous_vsync = new_vsync;
        }

        if new_display_enabled != self.previous_display_enabled {
            self.emit_debug_event(
                CrtcDebugEvent::DisplayEnableChanged {
                    enabled: new_display_enabled,
                },
                master_clock,
            );
            self.previous_display_enabled = new_display_enabled;
        }

        if new_address != self.previous_address {
            self.emit_debug_event(
                CrtcDebugEvent::AddressChanged {
                    is: new_address,
                    was: self.previous_address,
                },
                master_clock,
            );
            self.previous_address = new_address;
        }
    }

    fn read_address(&self) -> usize {
        let bits_14_and_15 = (self.video_memory_address & (0b11 << 12)) << 2;
        let bits_11_to_13 = ((self.c9_scan_line_counter & 0b111) as u16) << 11;
        let bits_0_to_10 = (self.video_memory_address & 0b11_1111_1111) << 1;

        (bits_14_and_15 | bits_11_to_13 | bits_0_to_10) as usize
    }

    fn read_display_enabled(&self) -> bool {
        // TODO: self.border_r6_condition || self.border_r1_condition
        self.c0_horizontal_counter < self.registers[Register::R1HorizontalDisplayed]
            && self.c4_character_row_counter < self.registers[Register::R6VerticalDisplayed]
    }

    fn read_horizontal_sync(&self) -> bool {
        // TODO: what happens before registers are initialized?
        let sync_start = self.registers[Register::R2HorizontalSyncPosition];
        let sync_end = self.registers[Register::R2HorizontalSyncPosition] + I::hsync_width(self);
        self.c0_horizontal_counter >= sync_start && self.c0_horizontal_counter < sync_end
        // this results in NO sync if the horizontal sync width is 0
    }

    fn read_vertical_sync(&self) -> bool {
        // TODO: what happens before registers are initialized?
        self.active_vsync_scanlines != 0
    }
}

macro_rules! dispatch {
    ($any_crtc:expr, $inner_crtc:ident => $body:expr) => {
        match &$any_crtc.inner {
            AnyCrtControllerInner::Type0($inner_crtc) => $body,
            AnyCrtControllerInner::Type1($inner_crtc) => $body,
            AnyCrtControllerInner::Type2($inner_crtc) => $body,
            AnyCrtControllerInner::Type4($inner_crtc) => $body,
        }
    };
}

macro_rules! dispatch_mut {
    ($any_crtc:expr, $inner_crtc:ident => $body:expr) => {
        match &mut $any_crtc.inner {
            AnyCrtControllerInner::Type0($inner_crtc) => $body,
            AnyCrtControllerInner::Type1($inner_crtc) => $body,
            AnyCrtControllerInner::Type2($inner_crtc) => $body,
            AnyCrtControllerInner::Type4($inner_crtc) => $body,
        }
    };
}

#[derive(Serialize, Deserialize)]
enum AnyCrtControllerInner {
    Type0(CrtController<Type0>),
    Type1(CrtController<Type1>),
    Type2(CrtController<Type2>),
    Type4(CrtController<Type4>),
}

#[derive(Serialize, Deserialize)]
pub struct AnyCrtController {
    inner: AnyCrtControllerInner,
}

impl AnyCrtController {
    pub fn new(type_: CrtcType) -> Self {
        let inner = match type_ {
            CrtcType::Type0 => AnyCrtControllerInner::Type0(CrtController::<Type0>::default()),
            CrtcType::Type1 => AnyCrtControllerInner::Type1(CrtController::<Type1>::default()),
            CrtcType::Type2 => AnyCrtControllerInner::Type2(CrtController::<Type2>::default()),
            CrtcType::Type4 => AnyCrtControllerInner::Type4(CrtController::<Type4>::default()),
        };

        Self { inner }
    }
}

impl Default for AnyCrtController {
    fn default() -> Self {
        Self::new(CrtcType::Type0)
    }
}

impl Snapshottable for AnyCrtController {
    type View = CrtcDebugView;

    fn debug_view(&self) -> Self::View {
        dispatch!(self, crtc => crtc.debug_view())
    }
}

impl CrtControllerInterface for AnyCrtController {
    fn read_byte(&mut self, port: u16) -> u8 {
        dispatch_mut!(self, crtc => crtc.read_byte(port))
    }

    fn write_byte(&mut self, port: u16, value: u8) {
        dispatch_mut!(self, crtc => crtc.write_byte(port, value))
    }

    fn step(&mut self, master_clock: MasterClockTick) {
        dispatch_mut!(self, crtc => crtc.step(master_clock))
    }

    fn read_address(&self) -> usize {
        dispatch!(self, crtc => crtc.read_address())
    }

    fn read_display_enabled(&self) -> bool {
        dispatch!(self, crtc => crtc.read_display_enabled())
    }

    fn read_horizontal_sync(&self) -> bool {
        dispatch!(self, crtc => crtc.read_horizontal_sync())
    }

    fn read_vertical_sync(&self) -> bool {
        dispatch!(self, crtc => crtc.read_vertical_sync())
    }
}

trait CrtControllerImpl: Default {
    const READ_MASKS: [u8; 18] = common::generate_read_masks();

    fn select_register(crtc: &mut CrtController<Self>, register: u8) {
        common::select_register(crtc, register)
    }

    fn resolve_selected_register_read(
        crtc: &CrtController<Self>,
    ) -> Result<Register, num_enum::TryFromPrimitiveError<Register>> {
        common::resolve_selected_register_read(crtc)
    }

    fn read_register(crtc: &CrtController<Self>) -> u8 {
        common::read_register(crtc)
    }

    fn read_status(crtc: &CrtController<Self>) -> u8 {
        common::read_status(crtc)
    }

    fn update_status(_crtc: &mut CrtController<Self>) {
        // Default implementation does nothing
    }

    fn update_border_r6_condition(crtc: &mut CrtController<Self>);

    fn resolve_selected_register_write(
        crtc: &CrtController<Self>,
    ) -> Result<Register, num_enum::TryFromPrimitiveError<Register>> {
        common::resolve_selected_register_write(crtc)
    }

    fn write_register(crtc: &mut CrtController<Self>, value: u8) {
        common::write_register(crtc, value);
    }

    fn hsync_width(crtc: &CrtController<Self>) -> u8;

    fn vsync_length(crtc: &CrtController<Self>) -> u8;

    fn update_video_memory_address(crtc: &mut CrtController<Self>);
}

mod common {
    use super::*;

    pub(super) const fn generate_read_masks() -> [u8; 18] {
        let mut masks = [0x00; 18];
        masks[14] = 0x3f; // Register::CursorAddressHigh
        masks[15] = 0xff; // Register::CursorAddressLow

        masks[16] = 0x3f; // Register::LightPenAddressHigh
        masks[17] = 0xff; // Register::LightPenAddressLow

        masks
    }

    pub(super) fn select_register<I>(crtc: &mut CrtController<I>, register: u8)
    where
        I: CrtControllerImpl,
    {
        crtc.selected_register = register;
        crtc.emit_debug_event(
            CrtcDebugEvent::RegisterSelected { register },
            crtc.master_clock,
        );
    }

    pub(super) fn resolve_selected_register_read<I>(
        crtc: &CrtController<I>,
    ) -> Result<Register, num_enum::TryFromPrimitiveError<Register>>
    where
        I: CrtControllerImpl,
    {
        Register::try_from(crtc.selected_register as usize & 0x1f)
    }

    pub(super) fn read_register<I>(crtc: &CrtController<I>) -> u8
    where
        I: CrtControllerImpl,
    {
        let Ok(register) = I::resolve_selected_register_read(crtc) else {
            return 0xff; // TODO: properly emulate floating bus
        };

        if matches!(register, Register::Unused | Register::R31Dummy) {
            return 0;
        }

        crtc.registers[register] & I::READ_MASKS[usize::from(register)]
    }

    pub(super) fn resolve_selected_register_write<I>(
        crtc: &CrtController<I>,
    ) -> Result<Register, num_enum::TryFromPrimitiveError<Register>>
    where
        I: CrtControllerImpl,
    {
        Register::try_from(crtc.selected_register as usize & 0x1f)
    }

    pub(super) fn write_register<I>(crtc: &mut CrtController<I>, value: u8)
    where
        I: CrtControllerImpl,
    {
        let Ok(register) = I::resolve_selected_register_write(crtc) else {
            return;
        };

        if matches!(
            register,
            Register::R16LightPenAddressHigh | Register::R17LightPenAddressLow
        ) {
            return;
        }

        let was = crtc.registers[register];

        let truncated = match register {
            Register::R0HorizontalTotal => value,
            Register::R1HorizontalDisplayed => value,
            Register::R2HorizontalSyncPosition => value,
            Register::R3HorizontalAndVerticalSyncWidths => value,
            Register::R4VerticalTotal => value & 0x7f,
            Register::R5VerticalTotalAdjust => value & 0x1f,
            Register::R6VerticalDisplayed => value & 0x7f,
            Register::R7VerticalSyncPosition => value & 0x7f,
            Register::R8InterlaceAndSkew => value,
            Register::R9MaximumRasterAddress => value & 0x1f,
            Register::R10CursorStartRaster => value,
            Register::R11CursorEndRaster => value,
            Register::R12DisplayStartAddressHigh => value,
            Register::R13DisplayStartAddressLow => value,
            Register::R14CursorAddressHigh => value,
            Register::R15CursorAddressLow => value,
            Register::R16LightPenAddressHigh => value,
            Register::R17LightPenAddressLow => value,
            Register::Unused => value,
            Register::R31Dummy => value,
        };

        crtc.registers[register] = truncated;

        crtc.emit_debug_event(
            CrtcDebugEvent::RegisterWritten {
                register,
                is: truncated,
                was,
            },
            crtc.master_clock,
        );
    }

    pub(super) fn read_status<I>(_crtc: &CrtController<I>) -> u8
    where
        I: CrtControllerImpl,
    {
        0xff
    }

    pub(super) fn update_border_r6_condition_type_02<I>(crtc: &mut CrtController<I>)
    where
        I: CrtControllerImpl,
    {
        if crtc.c0_horizontal_counter == crtc.c4_character_row_counter
            && crtc.c4_character_row_counter == crtc.c9_scan_line_counter
            && crtc.c9_scan_line_counter == 0
        {
            crtc.border_r6_condition = false;
        }

        if crtc.border_r6_condition {
            return;
        }

        if crtc.c4_character_row_counter == crtc.registers[Register::R6VerticalDisplayed] {
            if crtc.c4_character_row_counter != 0
                || crtc.c9_scan_line_counter != 0
                || crtc.c0_horizontal_counter == crtc.registers[Register::R1HorizontalDisplayed]
            {
                crtc.border_r6_condition = true;
            } else if !crtc.border_r1_condition {
                crtc.border_r6_condition = false; // TODO: toggle on after 8 MasterClockTicks (0.5 µs) when we emulate the 2-phase clock of the CRTC
            }
        }
    }

    pub(super) fn update_video_memory_address_type_04<I>(crtc: &mut CrtController<I>)
    where
        I: CrtControllerImpl,
    {
        crtc.video_memory_address += 1;

        if crtc.c0_horizontal_counter == crtc.registers[Register::R1HorizontalDisplayed]
            && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress]
        {
            crtc.video_memory_address_initial = crtc.video_memory_address;
        }

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            let display_start_address =
                ((crtc.registers[Register::R12DisplayStartAddressHigh] as u16) << 8)
                    + crtc.registers[Register::R13DisplayStartAddressLow] as u16;

            crtc.video_memory_address_initial = display_start_address;
        }

        if crtc.c0_horizontal_counter == 0 {
            crtc.video_memory_address = crtc.video_memory_address_initial;
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Type0 {}

impl Type0 {
    const fn generate_read_masks() -> [u8; 18] {
        let mut masks = common::generate_read_masks();
        masks[12] = 0x3f; // Register::DisplayStartAddressHigh
        masks[13] = 0xff; // Register::DisplayStartAddressLow

        masks
    }
}

impl CrtControllerImpl for Type0 {
    const READ_MASKS: [u8; 18] = Type0::generate_read_masks();

    fn update_border_r6_condition(crtc: &mut CrtController<Self>) {
        common::update_border_r6_condition_type_02(crtc);
    }

    fn hsync_width(crtc: &CrtController<Self>) -> u8 {
        crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] & 0x0f
    }

    fn vsync_length(crtc: &CrtController<Self>) -> u8 {
        if (crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] >> 4) == 0 {
            16
        } else {
            crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] >> 4
        }
    }

    fn update_video_memory_address(crtc: &mut CrtController<Self>) {
        common::update_video_memory_address_type_04(crtc);
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Type1 {
    border_cancellable: bool,
    border_r6_status: bool,
    light_pen_status: bool,
}

impl CrtControllerImpl for Type1 {
    fn read_register(crtc: &CrtController<Self>) -> u8 {
        if let Ok(Register::R31Dummy) = Self::resolve_selected_register_read(crtc) {
            return 0xff;
        };

        common::read_register(crtc)
    }

    fn read_status(crtc: &CrtController<Self>) -> u8 {
        let mut status = 0;

        if crtc.impl_.light_pen_status {
            status |= 0x40;
        }

        if crtc.impl_.border_r6_status {
            status |= 0x20;
        }

        status
    }

    fn update_status(crtc: &mut CrtController<Self>) {
        if crtc.c0_horizontal_counter == crtc.registers[Register::R0HorizontalTotal]
            && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress]
        {
            if crtc.c4_character_row_counter == crtc.registers[Register::R6VerticalDisplayed] {
                crtc.impl_.border_r6_status = true;
            }

            if crtc.c4_character_row_counter == crtc.registers[Register::R4VerticalTotal] {
                crtc.impl_.border_r6_status = false;
            }
        }
    }

    fn update_border_r6_condition(crtc: &mut CrtController<Self>) {
        if crtc.c0_horizontal_counter == crtc.c4_character_row_counter
            && crtc.c4_character_row_counter == crtc.c9_scan_line_counter
            && crtc.c9_scan_line_counter == 0
        {
            crtc.border_r6_condition = false;
            crtc.impl_.border_cancellable = false;
        }

        if crtc.border_r6_condition && !crtc.impl_.border_cancellable {
            return;
        }

        if crtc.c4_character_row_counter == crtc.registers[Register::R6VerticalDisplayed]
            || crtc.registers[Register::R6VerticalDisplayed] == 0
        {
            crtc.border_r6_condition = true;
            let definite = crtc.c4_character_row_counter
                == crtc.registers[Register::R6VerticalDisplayed]
                || crtc.registers[Register::R6VerticalDisplayed] == 0;
            let exception = crtc.c0_horizontal_counter
                == crtc.registers[Register::R0HorizontalTotal]
                && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress]
                && crtc.c4_character_row_counter == crtc.registers[Register::R4VerticalTotal];
            crtc.impl_.border_cancellable = !definite || exception;
        } else if crtc.impl_.border_cancellable {
            crtc.border_r6_condition = false;
        }
    }

    fn hsync_width(crtc: &CrtController<Self>) -> u8 {
        crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] & 0x0f
    }

    fn vsync_length(_crtc: &CrtController<Self>) -> u8 {
        16
    }

    fn update_video_memory_address(crtc: &mut CrtController<Self>) {
        crtc.video_memory_address += 1;

        if crtc.c0_horizontal_counter == crtc.registers[Register::R1HorizontalDisplayed]
            && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress]
        {
            crtc.video_memory_address_initial = crtc.video_memory_address;
        }

        if crtc.c0_horizontal_counter == 0 {
            crtc.video_memory_address = if crtc.c4_character_row_counter == 0 {
                ((crtc.registers[Register::R12DisplayStartAddressHigh] as u16) << 8)
                    + crtc.registers[Register::R13DisplayStartAddressLow] as u16
            } else {
                crtc.video_memory_address_initial
            };
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Type2 {}

impl CrtControllerImpl for Type2 {
    fn update_border_r6_condition(crtc: &mut CrtController<Self>) {
        common::update_border_r6_condition_type_02(crtc);
    }

    fn hsync_width(crtc: &CrtController<Self>) -> u8 {
        if (crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] & 0x0f) == 0 {
            16
        } else {
            crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] & 0x0f
        }
    }

    fn vsync_length(_crtc: &CrtController<Self>) -> u8 {
        16
    }

    fn update_video_memory_address(crtc: &mut CrtController<Self>) {
        crtc.video_memory_address += 1;

        if crtc.c0_horizontal_counter == crtc.registers[Register::R1HorizontalDisplayed]
            && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress]
        {
            crtc.video_memory_address_initial =
                if crtc.c4_character_row_counter == crtc.registers[Register::R4VerticalTotal] {
                    let display_start_address =
                        ((crtc.registers[Register::R12DisplayStartAddressHigh] as u16) << 8)
                            + crtc.registers[Register::R13DisplayStartAddressLow] as u16;

                    let video_memory_address_initial =
                        crtc.video_memory_address_initial & display_start_address;

                    if crtc.registers[Register::R1HorizontalDisplayed] == 0 {
                        video_memory_address_initial
                    } else {
                        video_memory_address_initial | display_start_address
                    }
                } else {
                    crtc.video_memory_address
                };
        }

        if crtc.c0_horizontal_counter == 0 {
            crtc.video_memory_address = crtc.video_memory_address_initial;
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Type4 {}

impl Type4 {
    const fn generate_read_masks() -> [u8; 18] {
        let mut masks = common::generate_read_masks();
        masks[10] = 0xff; // Register::CursorStartRaster
        masks[11] = 0xff; // Register::CursorEndRaster
        masks[12] = 0x3f; // Register::DisplayStartAddressHigh
        masks[13] = 0xff; // Register::DisplayStartAddressLow

        masks
    }
}

impl CrtControllerImpl for Type4 {
    const READ_MASKS: [u8; 18] = Type4::generate_read_masks();

    fn resolve_selected_register_read(
        crtc: &CrtController<Self>,
    ) -> Result<Register, num_enum::TryFromPrimitiveError<Register>> {
        let selected_register = 10 + ((crtc.selected_register & 0x07) as i8 - 2).rem_euclid(8);

        Register::try_from(selected_register as usize)
    }

    fn read_status(crtc: &CrtController<Self>) -> u8 {
        Self::read_register(crtc)
    }

    fn update_status(crtc: &mut CrtController<Self>) {
        let mut status1 = 0;

        if crtc.c0_horizontal_counter == crtc.registers[Register::R0HorizontalTotal] {
            status1 |= 0x01;
        }

        if !(crtc.c0_horizontal_counter == crtc.registers[Register::R0HorizontalTotal] / 2) {
            status1 |= 0x01 << 1;
        }

        if !(crtc.c0_horizontal_counter
            == crtc.registers[Register::R1HorizontalDisplayed].saturating_sub(1)
            && crtc.registers[Register::R0HorizontalTotal]
                >= crtc.registers[Register::R1HorizontalDisplayed])
        {
            status1 |= 0x01 << 2;
        }

        if !(crtc.c0_horizontal_counter == crtc.registers[Register::R2HorizontalSyncPosition]) {
            status1 |= 0x01 << 3;
        }

        if !(crtc.c0_horizontal_counter
            == crtc.registers[Register::R2HorizontalSyncPosition] + Self::hsync_width(crtc))
        {
            status1 |= 0x01 << 4;
        }

        if (crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] >> 4) > 0 {
            if !(crtc.active_vsync_scanlines == Self::vsync_length(crtc)) {
                status1 |= 0x01 << 5;
            }
        } else {
            if crtc.active_vsync_scanlines > 15
                || (crtc.active_vsync_scanlines == 0
                    && crtc.c4_character_row_counter
                        >= crtc.registers[Register::R7VerticalSyncPosition])
            {
                status1 |= 0x01 << 5;
            }
        }

        status1 |= 0x01 << 6;

        if crtc.c0_horizontal_counter < crtc.registers[Register::R0HorizontalTotal] {
            if !(crtc.video_memory_address & 0x00ff == 0xff) {
                status1 |= 0x01 << 7;
            }
        } else {
            if !(crtc.video_memory_address_initial & 0x00ff == 0x00) {
                status1 |= 0x01 << 7;
            }
        }

        crtc.registers[Register::R10CursorStartRaster] = status1;

        let mut status2 = crtc.registers[Register::R11CursorEndRaster] & (0x01 << 3);

        let last_char_common = crtc.c0_horizontal_counter
            == crtc.registers[Register::R0HorizontalTotal]
            && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress];

        if !(last_char_common
            && crtc.c4_character_row_counter == crtc.registers[Register::R4VerticalTotal])
        {
            status2 |= 0x01;
        }

        if !(last_char_common
            && crtc.c4_character_row_counter
                == crtc.registers[Register::R6VerticalDisplayed].saturating_sub(1))
        {
            // TODO: Verify behavior when R6 == 0; is saturating or wrapping correct?
            status2 |= 0x01 << 1;
        }

        if !(last_char_common
            && crtc.c4_character_row_counter
                == crtc.registers[Register::R7VerticalSyncPosition].saturating_sub(1))
        {
            // TODO: Verify behavior when R6 == 0; is saturating or wrapping correct?
            status2 |= 0x01 << 2;
        }

        if crtc.frame > 0
            && crtc.frame.is_multiple_of(16)
            && crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            status2 ^= 0x01 << 3;
        }

        status2 |= 0x01 << 4;

        if !(crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress]) {
            status2 |= 0x01 << 5;
        }

        // status2, bit 6 is always 0

        if (crtc.c0_horizontal_counter == crtc.registers[Register::R0HorizontalTotal]
            && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress])
            || (crtc.c0_horizontal_counter < crtc.registers[Register::R0HorizontalTotal]
                && crtc.c9_scan_line_counter == 0)
        {
            status2 |= 0x01 << 7;
        }

        crtc.registers[Register::R11CursorEndRaster] = status2;
    }

    fn update_border_r6_condition(crtc: &mut CrtController<Self>) {
        if crtc.c0_horizontal_counter == crtc.c4_character_row_counter
            && crtc.c4_character_row_counter == crtc.c9_scan_line_counter
            && crtc.c9_scan_line_counter == 0
        {
            crtc.border_r6_condition = false;
        }

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == crtc.registers[Register::R6VerticalDisplayed]
        {
            crtc.border_r6_condition = true;
        }
    }

    fn hsync_width(crtc: &CrtController<Self>) -> u8 {
        if (crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] & 0x0f) == 0 {
            16
        } else {
            crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] & 0x0f
        }
    }

    fn vsync_length(crtc: &CrtController<Self>) -> u8 {
        if (crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] >> 4) == 0 {
            16
        } else {
            crtc.registers[Register::R3HorizontalAndVerticalSyncWidths] >> 4
        }
    }

    fn update_video_memory_address(crtc: &mut CrtController<Self>) {
        common::update_video_memory_address_type_04(crtc);
    }
}

#[cfg(test)]
mod tests;
