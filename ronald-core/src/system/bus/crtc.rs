use std::fmt;

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
    HorizontalTotal,
    HorizontalDisplayed,
    HorizontalSyncPosition,
    HorizontalAndVerticalSyncWidths,
    VerticalTotal,
    VerticalTotalAdjust,
    VerticalDisplayed,
    VerticalSyncPosition,
    InterlaceAndSkew,
    MaximumRasterAddress,
    CursorStartRaster,
    CursorEndRaster,
    DisplayStartAddressHigh,
    DisplayStartAddressLow,
    CursorAddressHigh,
    CursorAddressLow,
    LightPenAddressHigh,
    LightPenAddressLow,
    #[num_enum(alternatives = [19..31])]
    Unused,
    Dummy = 31,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::HorizontalTotal => write!(f, "R{} (Horizontal Total)", usize::from(*self)),
            Register::HorizontalDisplayed => {
                write!(f, "R{} (Horizontal Displayed)", usize::from(*self))
            }
            Register::HorizontalSyncPosition => {
                write!(f, "R{} (H. Sync Position)", usize::from(*self))
            }
            Register::HorizontalAndVerticalSyncWidths => {
                write!(f, "R{} (H/V Sync Widths)", usize::from(*self))
            }
            Register::VerticalTotal => write!(f, "R{} (Vertical Total)", usize::from(*self)),
            Register::VerticalTotalAdjust => write!(f, "R{} (V. Total Adjust)", usize::from(*self)),
            Register::VerticalDisplayed => {
                write!(f, "R{} (Vertical Displayed)", usize::from(*self))
            }
            Register::VerticalSyncPosition => {
                write!(f, "R{} (V. Sync Position)", usize::from(*self))
            }
            Register::InterlaceAndSkew => write!(f, "R{} (Interlace/Skew)", usize::from(*self)),
            Register::MaximumRasterAddress => {
                write!(f, "R{} (Max Raster Address)", usize::from(*self))
            }
            Register::CursorStartRaster => write!(f, "R{} (Cursor Start)", usize::from(*self)),
            Register::CursorEndRaster => write!(f, "R{} (Cursor End)", usize::from(*self)),
            Register::DisplayStartAddressHigh => {
                write!(f, "R{} (Display Start High)", usize::from(*self))
            }
            Register::DisplayStartAddressLow => {
                write!(f, "R{} (Display Start Low)", usize::from(*self))
            }
            Register::CursorAddressHigh => {
                write!(f, "R{} (Cursor Address High)", usize::from(*self))
            }
            Register::CursorAddressLow => write!(f, "R{} (Cursor Address Low)", usize::from(*self)),
            Register::LightPenAddressHigh => write!(f, "R{} (Light Pen High)", usize::from(*self)),
            Register::LightPenAddressLow => write!(f, "R{} (Light Pen Low)", usize::from(*self)),
            Register::Unused => write!(f, "R18-R30 (Unused)"),
            Register::Dummy => write!(f, "R{} (Dummy)", usize::from(*self)),
        }
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
    registers: [u8; 18],
    selected_register: u8,
    horizontal_counter: u8,
    horizontal_sync_width_counter: u8,
    character_row_counter: u8,
    scan_line_counter: u8,
    display_start_address: u16,
    master_clock: MasterClockTick,
    previous_hsync: bool,
    previous_vsync: bool,
    previous_display_enabled: bool,
    previous_address: usize,
    impl_: I,
}

impl<I> Snapshottable for CrtController<I>
where
    I: CrtControllerImpl,
{
    type View = CrtcDebugView;

    fn debug_view(&self) -> Self::View {
        CrtcDebugView {
            registers: self.registers,
            selected_register: self.selected_register,
            horizontal_counter: self.horizontal_counter,
            character_row_counter: self.character_row_counter,
            scan_line_counter: self.scan_line_counter,
            display_start_address: self.display_start_address,
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
        self.master_clock = master_clock;

        let horizontal_counter_was = self.horizontal_counter;
        let scan_line_was = self.scan_line_counter;
        let character_row_was = self.character_row_counter;

        self.horizontal_counter += 1;

        if self.horizontal_counter > self.registers[Register::HorizontalTotal as usize] {
            self.scan_line_counter += 1;
            self.horizontal_counter = 0;
        }

        if self.scan_line_counter > self.registers[Register::MaximumRasterAddress as usize] {
            self.character_row_counter += 1;
            self.scan_line_counter = 0;
        }

        if self.character_row_counter > self.registers[Register::VerticalTotal as usize] {
            // TODO: take VerticalTotalAdjust into account
            self.character_row_counter = 0;
        }

        self.emit_debug_event(
            CrtcDebugEvent::CountersChanged {
                character_row_is: self.character_row_counter,
                character_row_was,
                scan_line_is: self.scan_line_counter,
                scan_line_was,
                horizontal_counter_is: self.horizontal_counter,
                horizontal_counter_was,
            },
            master_clock,
        );

        if self.horizontal_counter == 0 && self.character_row_counter == 0 {
            self.display_start_address =
                ((self.registers[Register::DisplayStartAddressHigh as usize] as u16) << 8)
                    + self.registers[Register::DisplayStartAddressLow as usize] as u16;
        }

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
        let refresh_memory_address = self.display_start_address
            + self.registers[Register::HorizontalDisplayed as usize] as u16
                * self.character_row_counter as u16
            + self.horizontal_counter as u16;

        let bits_14_and_15 = (refresh_memory_address & (0b11 << 12)) << 2;
        let bits_11_to_13 = ((self.scan_line_counter & 0b111) as u16) << 11;
        let bits_0_to_10 = (refresh_memory_address & 0b11_1111_1111) << 1;

        (bits_14_and_15 | bits_11_to_13 | bits_0_to_10) as usize
    }

    fn read_display_enabled(&self) -> bool {
        self.horizontal_counter < self.registers[Register::HorizontalDisplayed as usize]
            && self.character_row_counter < self.registers[Register::VerticalDisplayed as usize]
    }

    fn read_horizontal_sync(&self) -> bool {
        // TODO: what happens before registers are initialized?
        let sync_start = self.registers[Register::HorizontalSyncPosition as usize];
        let sync_end = self.registers[Register::HorizontalSyncPosition as usize]
            + (self.registers[Register::HorizontalAndVerticalSyncWidths as usize] & 0b1111);
        self.horizontal_counter >= sync_start && self.horizontal_counter < sync_end
        // this results in NO sync if the horizontal sync width is 0
    }

    fn read_vertical_sync(&self) -> bool {
        // TODO: what happens before registers are initialized?
        let sync_start = self.registers[Register::VerticalSyncPosition as usize] as i32;
        let character_rows_since_start = self.character_row_counter as i32 - sync_start;
        let scan_lines_since_start =
            (self.registers[Register::MaximumRasterAddress as usize] as i32 + 1)
                * character_rows_since_start
                + self.scan_line_counter as i32;
        (0..16).contains(&scan_lines_since_start)
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

    fn resolve_selected_register_write(
        crtc: &CrtController<Self>,
    ) -> Result<Register, num_enum::TryFromPrimitiveError<Register>> {
        common::resolve_selected_register_write(crtc)
    }

    fn write_register(crtc: &mut CrtController<Self>, value: u8) {
        common::write_register(crtc, value);
    }
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

        if matches!(register, Register::Unused | Register::Dummy) {
            return 0;
        }

        crtc.registers[usize::from(register)] & I::READ_MASKS[usize::from(register)]
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
            Register::LightPenAddressHigh | Register::LightPenAddressLow
        ) {
            return;
        }

        let was = crtc.registers[usize::from(register)];

        let truncated = match register {
            Register::HorizontalTotal => value,
            Register::HorizontalDisplayed => value,
            Register::HorizontalSyncPosition => value,
            Register::HorizontalAndVerticalSyncWidths => value,
            Register::VerticalTotal => value & 0x7f,
            Register::VerticalTotalAdjust => value & 0x1f,
            Register::VerticalDisplayed => value & 0x7f,
            Register::VerticalSyncPosition => value & 0x7f,
            Register::InterlaceAndSkew => value,
            Register::MaximumRasterAddress => value & 0x1f,
            Register::CursorStartRaster => value,
            Register::CursorEndRaster => value,
            Register::DisplayStartAddressHigh => value,
            Register::DisplayStartAddressLow => value,
            Register::CursorAddressHigh => value,
            Register::CursorAddressLow => value,
            Register::LightPenAddressHigh => value,
            Register::LightPenAddressLow => value,
            Register::Unused => value,
            Register::Dummy => value,
        };

        crtc.registers[usize::from(register)] = truncated;

        crtc.emit_debug_event(
            CrtcDebugEvent::RegisterWritten {
                register,
                is: truncated,
                was,
            },
            crtc.master_clock,
        );
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
}

#[derive(Default, Serialize, Deserialize)]
struct Type1 {}

impl CrtControllerImpl for Type1 {}

#[derive(Default, Serialize, Deserialize)]
struct Type2 {}

impl CrtControllerImpl for Type2 {}

#[derive(Default, Serialize, Deserialize)]
struct Type4 {}

impl Type4 {
    const fn generate_read_masks() -> [u8; 18] {
        let mut masks = common::generate_read_masks();
        masks[12] = 0x3f; // Register::DisplayStartAddressHigh
        masks[13] = 0xff; // Register::DisplayStartAddressLow

        masks
    }
}

impl CrtControllerImpl for Type4 {
    const READ_MASKS: [u8; 18] = Type0::generate_read_masks();
}

// Test suite derived from "The Amstrad CPC CRTC Compendium" (ACCC) v1.11 by Longshot / Logon System.
// Section numbers in the comments refer to the compendium. One `step` is one CRTC character (1 µs).
//
// Test names start with `test_type_<types>_`, listing the CRTC types the behaviour applies to:
//   type 0: Hitachi HD6845S / UMC UM6845
//   type 1: UMC UM6845R
//   type 2: Motorola MC6845
//   type 4: Amstrad pre-ASIC 40226
// `all` means types 0, 1, 2 and 4. Type 3 (CPC+ ASIC) is out of scope.
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    macro_rules! crtcs {
        (All) => {
            [
                AnyCrtController::new(CrtcType::Type0),
                AnyCrtController::new(CrtcType::Type1),
                AnyCrtController::new(CrtcType::Type2),
                AnyCrtController::new(CrtcType::Type4),
            ]
        };
        ($($variant:ident),+) => {
            [$(
                AnyCrtController::new(CrtcType::$variant)
            ),+]
        };
    }

    mod register_access {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn test_type_all_register_function_is_decoded_from_port_bits_8_and_9() {
            // ACCC 4.3, 4.4.1: &BC00 selects, &BD00 writes, &BE00 reads status, &BF00 reads a register.
            // The low byte of the port address must not influence the function.

            for port in 0xbc00..=0xbcff {
                assert_eq!(Function::from(port), Function::Select);
            }

            for port in 0xbd00..=0xbdff {
                assert_eq!(Function::from(port), Function::Write);
            }

            for port in 0xbe00..=0xbeff {
                assert_eq!(Function::from(port), Function::Status);
            }

            for port in 0xbf00..=0xbfff {
                assert_eq!(Function::from(port), Function::Read);
            }

            assert_eq!(Function::from(0x4000), Function::Ignore);
        }

        #[test]
        fn test_type_all_register_select_for_writes_ignores_upper_three_bits() {
            // ACCC 5.1: only the 5 low bits of the register number count, so selecting &29 is the same as selecting R9.

            for crtc in &mut crtcs!(All) {
                for register in 0..=255 {
                    if register & 0x1f >= 16 {
                        continue; // Only R0-R15 are writable
                    }

                    crtc.write_byte(0xbc00, register);
                    crtc.write_byte(0xbd00, 0xff);

                    let value = dispatch!(crtc, crtc => crtc.registers[register as usize & 0x1f]);

                    assert_ne!(value, 0x00);

                    crtc.write_byte(0xbd00, 0x00);
                }
            }
        }

        #[test]
        fn test_type_all_r9_write_is_truncated_to_five_bits() {
            // ACCC 5.1, 10.1: writing &27 to R9 stores 7.

            for crtc in &mut crtcs!(All) {
                crtc.write_byte(0xbc00, 9);
                crtc.write_byte(0xbd00, 0x27);

                let r9_value = dispatch!(crtc, crtc => crtc.registers[9]);

                assert_eq!(r9_value, 7);
            }
        }

        // TODO: test for other register truncations according to ACCC 4.3

        #[test]
        fn test_type_all_r5_write_is_truncated_to_five_bits() {
            // ACCC 11.1: R5 holds a number of lines on 5 bits (0 to 31).

            for crtc in &mut crtcs!(All) {
                crtc.write_byte(0xbc00, 5);
                crtc.write_byte(0xbd00, 0x27);

                let r5_value = dispatch!(crtc, crtc => crtc.registers[5]);

                assert_eq!(r5_value, 7);
            }
        }

        #[test]
        fn test_type_all_character_row_registers_are_truncated_to_seven_bits() {
            // ACCC 12.1: C4 counts up to 127, so R4, R6 and R7 are 7-bit registers.

            for crtc in &mut crtcs!(All) {
                for register in [4, 6, 7] {
                    crtc.write_byte(0xbc00, register);
                    crtc.write_byte(0xbd00, 0xff);

                    let value = dispatch!(crtc, crtc => crtc.registers[register as usize]);

                    assert_eq!(value, 0x7f);
                }
            }
        }

        #[test]
        fn test_type_all_writes_to_read_ports_are_ignored() {
            // ACCC 4.3: only &BC00 and &BD00 are writable; writing to &BE00/&BF00 changes nothing.

            for crtc in &mut crtcs!(All) {
                let before = dispatch!(crtc, crtc => crtc.selected_register);
                crtc.write_byte(0xbc00, 0x42);
                let after = dispatch!(crtc, crtc => crtc.selected_register);

                assert_ne!(before, after);

                let before = dispatch!(crtc, crtc => crtc.registers);
                crtc.write_byte(0xbd00, 0x42);
                let after = dispatch!(crtc, crtc => crtc.registers);

                assert_ne!(before, after);

                for port in [0xbe00, 0xbf00] {
                    let before = dispatch!(crtc, crtc => (crtc.selected_register, crtc.registers));
                    crtc.write_byte(port, 0x42);
                    let after = dispatch!(crtc, crtc => (crtc.selected_register, crtc.registers));

                    assert_eq!(before, after);
                }
            }
        }

        #[test]
        fn test_type_all_cursor_registers_store_and_read_back_values() {
            // ACCC 21.2.1-21.2.3: the cursor is unused on CPC, but R14/R15 still store values that can be read back.

            for crtc in &mut crtcs!(All) {
                for register in [14, 15] {
                    crtc.write_byte(0xbc00, register);
                    crtc.write_byte(0xbd00, 0x3f);

                    let value = crtc.read_byte(0xbf00);

                    assert_eq!(value, 0x3f);
                }
            }
        }

        #[test]
        fn test_type_all_light_pen_registers_are_read_only() {
            // ACCC 4.3: R16/R17 can be read but writes to them are ignored.

            for crtc in &mut crtcs!(All) {
                for register in [16, 17] {
                    crtc.write_byte(0xbc00, register);
                    crtc.write_byte(0xbd00, 0x42);

                    let value = crtc.read_byte(0xbf00);

                    assert_eq!(value, 0x00);
                }
            }
        }

        #[test]
        fn test_type_all_cursor_and_light_pen_high_registers_read_bits_6_and_7_as_zero() {
            // ACCC 21.2.1-21.2.3: R14 and R16 only have 6 bits; bits 6 and 7 read as 0.

            for crtc in &mut crtcs!(All) {
                for register in [14, 16] {
                    dispatch_mut!(crtc, crtc => crtc.registers[register as usize] = 0xff);

                    crtc.write_byte(0xbc00, register);
                    let value = crtc.read_byte(0xbf00);

                    assert_eq!(value & 0b1100_0000, 0);
                }
            }
        }

        #[test]
        fn test_type_012_register_select_for_reads_ignores_upper_three_bits() {
            // ACCC 21.2.1, 21.2.2, 28.1.9: selecting register 108 and reading &BF00 is the same as reading R12.

            for crtc in &mut crtcs!(Type0, Type1, Type2) {
                dispatch_mut!(crtc, crtc => crtc.registers = std::array::from_fn(|i| i as u8));

                for register in 0..=255 {
                    crtc.write_byte(0xbc00, register);
                    let value = crtc.read_byte(0xbf00);

                    crtc.write_byte(0xbc00, register & 0x1f);
                    let expected = crtc.read_byte(0xbf00);

                    assert_eq!(value, expected);
                }
            }
        }

        #[test]
        fn test_type_04_display_start_address_is_readable() {
            // ACCC 4.3, 21.2.1, 21.2.3: R12 and R13 can be read back.

            for crtc in &mut crtcs!(Type0, Type4) {
                for register in [0x0c, 0x0d] {
                    crtc.write_byte(0xbc00, register);
                    crtc.write_byte(0xbd00, 0xff);

                    let value = crtc.read_byte(0xbf00);

                    assert_ne!(value, 0);
                }
            }
        }

        #[test]
        fn test_type_04_display_start_address_high_reads_bits_6_and_7_as_zero() {
            // ACCC 21.2.1, 21.2.3: R12 only has 6 bits.

            for crtc in &mut crtcs!(Type0, Type4) {
                crtc.write_byte(0xbc00, 0x0c);
                crtc.write_byte(0xbd00, 0xff);

                let value = crtc.read_byte(0xbf00);

                assert_eq!(value, 0x3f);
            }
        }

        #[test]
        fn test_type_12_display_start_address_reads_as_zero() {
            // ACCC 4.3, 21.2.2, 28.1.9: R12/R13 are write-only on types 1 and 2, so reading them returns 0.

            for crtc in &mut crtcs!(Type1, Type2) {
                crtc.write_byte(0xbc00, 0x0c);
                crtc.write_byte(0xbd00, 0xff);

                let value = crtc.read_byte(0xbf00);

                assert_eq!(value, 0x00);
            }
        }

        #[test]
        fn test_type_012_reading_write_only_registers_returns_zero() {
            // ACCC 21.2.1, 21.2.2: reading R0-R11 returns 0.

            for crtc in &mut crtcs!(Type0) {
                for register in 0..=0x0b {
                    crtc.write_byte(0xbc00, register);
                    crtc.write_byte(0xbd00, 0xff);

                    let value = crtc.read_byte(0xbf00);

                    assert_eq!(value, 0x00);
                }
            }

            for crtc in &mut crtcs!(Type1, Type2) {
                for register in 0..=0x0d {
                    crtc.write_byte(0xbc00, register);
                    crtc.write_byte(0xbd00, 0xff);

                    let value = crtc.read_byte(0xbf00);

                    assert_eq!(value, 0x00);
                }
            }
        }

        #[test]
        #[ignore]
        fn test_type_02_reading_undefined_registers_returns_zero() {
            // ACCC 21.2.1, 21.2.2, 21.4: reading R18-R31 returns 0; the "dummy" R31 does not exist.

            for crtc in &mut crtcs!(Type0, Type2) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_1_reading_register_31_returns_non_zero() {
            // ACCC 21.2.2, 21.4, 28.1.9: R31 (and any number whose bits 0-4 are all 1) reads as a non-zero value (127 or 255 observed).

            for crtc in &mut crtcs!(Type1) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_0_status_port_does_not_return_register_contents() {
            // ACCC 21.3.2: type 0 has no status register; the bus floats (255 or 127 observed).

            for crtc in &mut crtcs!(Type0) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_2_status_port_reads_255() {
            // ACCC 21.3.2: type 2 has no status register; &BE00 always read 255 on the test machine.

            for crtc in &mut crtcs!(Type2) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_1_status_register_unused_bits_read_as_zero() {
            // ACCC 21.3.3: bits 0-4 and 7 of the &BE00 status register read 0.

            for crtc in &mut crtcs!(Type1) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_1_status_bit_5_reflects_r6_border_state_updated_at_c0_equal_r0() {
            // ACCC 21.3.3: bit 5 becomes 1 at C0=R0 of the line before C4=R6, C9=0 and becomes 0 at C0=R0 of the line before C4=C9=0.

            for crtc in &mut crtcs!(Type1) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_1_status_bit_5_ignores_border_from_r6_zero() {
            // ACCC 21.3.3: setting R6=0 while C4>0 shows border but does not set bit 5.

            for crtc in &mut crtcs!(Type1) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_register_reads_use_only_three_bits_of_register_number() {
            // ACCC 21.2.3, 28.1.9: reads map numbers 0-7 to R16, R17, R10, R11, R12, R13, R14, R15, so reading R4 or R20 returns R12.

            for crtc in &mut crtcs!(Type4) {
                todo!("display this correctly in the debug window")
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_port_mirrors_read_port() {
            // ACCC 21.2.3, 21.3.1, 28.1.8: &BE00 behaves exactly like &BF00.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_1_bit_0_is_set_when_c0_equals_r0() {
            // ACCC 21.3.4.1: reading R10 returns status 1; bit 0 is 1 only while C0=R0.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_1_bit_1_is_cleared_when_c0_equals_half_r0() {
            // ACCC 21.3.4.1: bit 1 is 0 only while C0=R0/2.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_1_bit_2_is_cleared_when_c0_equals_r1_minus_one() {
            // ACCC 21.3.4.1: bit 2 is 0 while C0=R1-1 (if R0>=R1).

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_1_bits_3_and_4_are_cleared_at_hsync_start_and_end() {
            // ACCC 21.3.4.1: bit 3 is 0 while C0=R2; bit 4 is 0 while C0=R2+R3.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_1_bit_5_tracks_last_vsync_line() {
            // ACCC 21.3.4.1: bit 5 is 0 on line R3h of the VSYNC (R3h>0), or 1 over 15 lines from the VSYNC start when R3h=0.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_1_bit_6_is_always_set() {
            // ACCC 21.3.4.1.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_1_bit_7_is_cleared_before_vma_low_byte_resets() {
            // ACCC 21.3.4.1: bit 7 is 0 when VMA's low byte is &FF (C0<R0) or when VMA' low byte is &00 at C0=R0.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_2_bits_0_to_2_flag_last_char_of_screen_display_and_before_vsync() {
            // ACCC 21.3.4.2: bit 0 is 0 at C4=R4,C9=R9,C0=R0; bit 1 at C4=R6-1,C9=R9,C0=R0; bit 2 at C4=R7-1,C9=R9,C0=R0.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_2_bit_3_toggles_every_16_frames() {
            // ACCC 21.3.4.2: with line-to-line rupture it toggles every 16 lines.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_2_constant_bits() {
            // ACCC 21.3.4.2: bit 4 is always 1 and bit 6 is always 0.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_2_bit_5_is_cleared_on_last_raster_of_character() {
            // ACCC 21.3.4.2: bit 5 is 0 on every C0 of a line with C9=R9.

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }

        #[test]
        #[ignore]
        fn test_type_4_status_2_bit_7_is_set_on_character_boundaries() {
            // ACCC 21.3.4.2: bit 7 is 1 when (C9=R9 and C0=R0) or (C9=0 and C0<R0).

            for crtc in &mut crtcs!(Type4) {
                todo!()
            }
        }
    }

    mod frame_geometry {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_all_horizontal_counter_counts_from_zero_to_r0_inclusive() {
            // ACCC 13.1: C0 counts from 0 up to and including R0, so a line lasts R0+1 µs.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_raster_counter_increments_when_horizontal_counter_wraps() {
            // ACCC 6.1.1: when C0 reaches R0 it goes to 0 and C9 is incremented.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_character_row_counter_increments_when_raster_counter_wraps() {
            // ACCC 6.1.1: when C9 reaches R9 it goes to 0 and C4 is incremented.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_counters_restart_frame_after_last_line_when_r5_is_zero() {
            // ACCC 6.1.4: on C0=R0, C9=R9 and C4=R4 with R5=0, C4 and C9 both return to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_standard_european_frame_lasts_19968_us() {
            // ACCC 4.1: R0=63, R4=38, R9=7, R5=0 gives 312 lines of 64 µs;
            // total lines = (R4+1) x (R9+1) + R5.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_standard_us_frame_has_262_lines() {
            // ACCC 4.1: the US ROM table uses R4=31, R9=7, R5=6: 32 x 8 + 6 = 262 lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r4_below_c4_makes_c4_count_to_127_before_wrapping() {
            // ACCC 12.1, 12.3, 12.5: if R4 is set below C4 (outside vertical adjustment), C4 counts up to 127 and wraps.
            todo!()
        }
    }

    mod raster_counter {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_all_r9_update_is_considered_up_to_c0_equal_r0() {
            // ACCC 10.2: setting R9 to 0 up to C0=R0 of a C9=0 line still ends the character on that line.
            // (The type 3/4 diagrams are shifted by one µs, which matches the ASIC's later I/O cycle, ACCC 4.4.3.)
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_setting_r9_to_current_c9_ends_character_on_next_line() {
            // ACCC 10.3.1.1, 10.3.2.1, 10.3.4.1: C9=3 and R9 changed from 7 to 3 gives C9=0 on the next line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_setting_r9_above_c9_increments_c9_without_changing_c4() {
            // ACCC 10.3.1.1, 10.3.2.1, 10.3 tables: C9=0 and R9 set to 7 gives C9=1 on the next line, C4 unchanged.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_setting_r9_below_c9_overflows_c9_up_to_31() {
            // ACCC 10.3, 10.3.1.1, 10.3.2.1, 10.3.3: C9=3 and R9 set to 1 gives C9=4; C9 counts to 31 before looping to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_c9_overflow_keeps_c4_unchanged_until_c9_wraps() {
            // ACCC 10.3.2.1: while C9 overflows, C4 does not change; the offset is only reloaded if C4=C9=C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_setting_r9_at_or_below_c9_resets_c9_on_next_line() {
            // ACCC 10.3.4.1, 12.5, 13.5: if C9 >= R9 the next C9 is 0 and C4 increments (or goes to 0 if C4=R4); C9 cannot overflow.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r9_changed_at_c0_equal_r0_on_character_end_increments_c4_and_c9() {
            // ACCC 10.3.1.2: if R9 changes exactly at C0=R0 while C9 was R9, C4 is still incremented,
            // so C4 and C9 can both increment at once.
            todo!()
        }
    }

    mod last_line {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_0_last_line_state_is_evaluated_only_while_c0_is_below_two() {
            // ACCC 12.2: C4==R4 && C9==R9 is only tested on C0=0 and C0=1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r4_or_r9_updated_at_c0_below_two_establishes_last_line() {
            // ACCC 12.2: R4/R9 can be changed on the current line while C0<2 to make it the last line,
            // so C4 and C9 reset on the next line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r4_or_r9_updated_after_c0_one_does_not_cancel_last_line() {
            // ACCC 10.3.1.2, 12.2: after C0>1 the last line state stays true and C4/C9 still reset to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r4_or_r9_updated_at_c0_zero_can_cancel_last_line() {
            // ACCC 12.2: an update at C0=0 that makes C4<>R4 (or C9<>R9) overrides the last line state.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r4_or_r9_updated_at_c0_one_on_last_line_starts_vertical_adjustment() {
            // ACCC 10.3.1.2, 12.2: an update at C0=1 on a last line activates vertical adjustment
            // even with R5=0; the current line becomes the first adjustment line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r4_and_r9_zeroed_on_first_line_after_c0_one_do_not_make_it_last_line() {
            // ACCC 12.2.1: with R4, R9 > 0, zeroing both on the first line (C0>1) is too late;
            // the next line gets C4=1, C9=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_line_to_line_rupture_keeps_c4_and_c9_at_zero() {
            // ACCC 12.2.1 (R.L.A.L.): with R4=R9=0, every line is a last line and starts a new frame with C4=C9=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_line_to_line_rupture_ends_when_r4_or_r9_updated_after_c0_one() {
            // ACCC 12.2.1: raising R4 and/or R9 after C0>1 still resets C4/C9 once, then counting resumes.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_line_to_line_rupture_reloads_r12_r13_on_every_line() {
            // ACCC 12.2.1, 12.3, 12.4.2, 12.5, 20.3: because every line starts with C4=C9=0, R12/R13 updates apply on every line
            // (on type 2 they must be written before C0=R1).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_14_setting_r4_and_r9_to_zero_on_first_line_starts_line_to_line_rupture() {
            // ACCC 12.3, 12.5: with C4=C9=0, setting R4=R9=0 is enough; there is no last line state to arm.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_14_setting_r4_to_c4_resets_c4_when_c9_wraps() {
            // ACCC 12.3, 12.5: if C9<R9, C9 keeps counting and C4 goes to 0 when C9 wraps; if C9=R9, C4=C9=0 on the next line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_setting_r4_to_zero_on_last_line_overflows_c4() {
            // ACCC 12.3: unlike type 0, R4=0 on the last line of the frame is handled as the general case and C4 overflows to 127.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_last_line_is_evaluated_at_c0_zero_with_new_r4_but_old_r9() {
            // ACCC 12.4.1: at C0=0 the comparison uses R4 updated on C0=0 but R9 as it was before C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_last_line_state_cannot_be_cancelled_once_set() {
            // ACCC 12.4.1, 13.4: once armed, C4 and C9 go to 0 on the next line whatever R4/R9 become.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_last_line_is_not_set_if_previous_line_was_last_line() {
            // ACCC 12.4.1: the "previous last line" state (sampled on the last HSYNC character) prevents two consecutive detections.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_last_line_is_not_set_if_hsync_starts_at_c0_zero() {
            // ACCC 12.4.1, 15.6: an HSYNC starting on C0=0 cancels the last line, so C4 increments instead of going to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r4_or_r9_update_outside_hsync_re_evaluates_last_line() {
            // ACCC 12.4.1: while "last line management" is active, updating R4/R9 can arm the last line state mid-line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r4_or_r9_update_during_hsync_does_not_set_last_line() {
            // ACCC 15.6: updates to R4/R9 that would make the line a last line are ignored during HSYNC.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_last_line_management_is_inactive_on_first_line_of_frame() {
            // ACCC 12.4.1: on C4=C9=0 (with R4 or R9 non-zero), updating R4/R9 cannot arm the last line state.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_previous_last_line_is_sampled_on_last_hsync_character() {
            // ACCC 12.4.1: at C0=R2+R3-1, C4==R4 && C9==R9 sets "previous last line"; otherwise it clears it and re-enables management.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_missing_hsync_freezes_last_line_management() {
            // ACCC 12.4.1: with R2>R0, "previous last line" and "last line management" are never updated.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_line_to_line_rupture_requires_toggling_r9_during_hsync() {
            // ACCC 12.4.2: with R4=R9=0, R9 must differ from C9 on the last HSYNC character and be set back to 0 after the HSYNC
            // for C4=C9=0 to hold on every line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r9_raised_on_last_line_still_resets_counters() {
            // ACCC 12.4.2: when the last line state is armed, raising R9 has no effect and C4/C9 go to 0 on the next line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_counting_after_two_consecutive_last_lines_uses_current_r9() {
            // ACCC 12.4.2: if C9<>R9, C9 increments; if C9==R9, C9 goes to 0 and C4 increments unconditionally.
            todo!()
        }
    }

    mod vertical_adjustment {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_all_r5_adds_extra_raster_lines_at_end_of_frame() {
            // ACCC 11.1: with R5>0, R5 additional lines follow the (R4+1) x (R9+1) lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_c4_and_c9_return_to_zero_after_vertical_adjustment() {
            // ACCC 11.2.1, 13.2.4: once the adjustment ends, the next line is C4=C9=0 regardless of R4.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r5_set_to_next_line_number_ends_adjustment() {
            // ACCC 11.3, 11.3.1-11.3.3: R5 set to C9+1 (types 0, 4) or C5+1 (types 1, 2) stops the adjustment; C4=C9=0 on the next line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_c4_is_incremented_exactly_once_during_vertical_adjustment() {
            // ACCC 11.2.1, 13.2.4, 28.1.1: with R4=10, R9=3, R5=16, all adjustment lines have C4=11.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_c9_counts_past_r9_up_to_r5_minus_one_during_vertical_adjustment() {
            // ACCC 11.1, 11.2.1, 11.2.2, 11.2.6: types 0 and 4 have no C5 counter; with R9=3, R5=16, C9 goes 0..15.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_c4_increments_each_time_c9_wraps_during_vertical_adjustment() {
            // ACCC 11.1, 11.2.1, 11.2.3: with R4=10, R9=3, R5=16, C9 cycles 0..3, C5 counts 0..15 and C4 goes 11, 12, 13, 14.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_c4_stays_at_r4_during_vertical_adjustment() {
            // ACCC 11.1, 11.2.6, 12.5: C4 is not incremented; the last character absorbs the adjustment lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_video_pointer_is_latched_when_c9_equals_r9_during_vertical_adjustment() {
            // ACCC 11.2.1: with R1=40, the address advances by 40 after the C9=R9 line of the adjustment and then stays there.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_video_pointer_advances_with_each_adjustment_character() {
            // ACCC 11.2.1, 11.2.3: with R1=40, R9=3, the address goes 0, 40, 80, 120 over the adjustment characters.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_video_pointer_is_not_updated_during_vertical_adjustment() {
            // ACCC 11.2.1, 11.2.6: all adjustment lines use the same row address (0 in the example).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r9_update_during_vertical_adjustment_relatches_video_pointer() {
            // ACCC 11.2.2: setting R9=10 at C9=4 makes the pointer advance again (40 -> 80) after C9=10.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_r9_update_during_vertical_adjustment_changes_character_height() {
            // ACCC 11.2.3: setting R9=10 at C9=0 of C4=12 makes C9 count 0..10 before C4 becomes 13.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r5_set_before_c0_three_of_last_line_enables_vertical_adjustment() {
            // ACCC 11.2.2, 13.2.1: R5>0 written on C0=0, 1 or 2 of the last line adds R5 lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r5_set_after_c0_two_of_last_line_is_ignored() {
            // ACCC 11.4.2, 13.2.1: R5>0 written after C0>2 on the last line adds no lines; next line is C4=C9=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_124_r5_update_is_considered_at_any_c0_of_last_line() {
            // ACCC 11.4.1: R5 is evaluated on every C0 position.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r4_modified_on_last_line_switches_c9_comparison_to_r5() {
            // ACCC 11.2.2 example 2: R4 changed between C0=2 and C0=R0 on the last line makes C9 count until R5.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r9_modified_on_last_line_continues_c9_before_adjustment() {
            // ACCC 11.2.2 example 3: R4=38, R9 changed between C0=2 and C0=R0-1 on C9=7 gives C4=38, C9=8.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r9_modified_at_c0_equal_r0_on_last_line_increments_c4_and_c9() {
            // ACCC 11.2.2: changing R9 exactly at C0=R0 on the last line (C4=R4=38) gives C4=39, C9=8.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r4_or_r9_modified_on_last_line_postpones_end_of_frame() {
            // ACCC 11.2.4: before C0=R0, the new R4/R9 values cancel the adjustment and the frame continues.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r4_or_r9_modified_at_c0_equal_r0_on_last_line_keeps_adjustment() {
            // ACCC 11.2.4: at C0=R0 the adjustment stays active and the new R4/R9 values are used to count the extra lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r4_or_r9_modified_on_last_line_counts_within_adjustment() {
            // ACCC 11.2.5, 12.4.1: the new values are used immediately for C9/C4 and the resulting lines are adjustment lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_vma_loaded_from_r12_r13_during_adjustment_after_c4_zero() {
            // ACCC 11.2.4, 17.4.2: with R4=0 and R5>0, VMA is reloaded from R12/R13 on every line while C4=1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_r5_set_below_next_line_number_overflows_counter() {
            // ACCC 11.3.1, 11.3.2: C9 (type 0) or C5 (types 1, 2) wraps through 0 and the adjustment ends when it reaches the new R5.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_r5_set_below_c9_plus_one_ends_adjustment() {
            // ACCC 11.3.3: the current line becomes the last; C9 cannot overflow.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r5_set_to_zero_during_adjustment_prevents_c4_reset() {
            // ACCC 11.3.2: the adjustment state stays on, C5 loops and C4 does not return to 0 until C5+1 reaches a non-zero R5.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r5_update_after_c0_two_of_final_adjustment_line_is_ignored() {
            // ACCC 11.3.1: on the last adjustment line (C9+1=R5), R5 updates after C0>2 are not considered.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_vsync_can_be_triggered_by_c4_value_reached_during_adjustment() {
            // ACCC 11.5.1, 15.4.1: R7 set to a C4 value above R4 triggers a VSYNC during the adjustment lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_vsync_during_adjustment_requires_c9_and_c0_zero() {
            // ACCC 11.5.4: the VSYNC only starts when C4=R7 on C0=C9=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r7_set_to_c4_at_c0_below_two_during_adjustment_blocks_vsync() {
            // ACCC 11.5.2: on type 0 the VSYNC is blocked when R7 is set to C4 while C0<2.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_r6_can_match_c4_value_reached_during_adjustment() {
            // ACCC 11.7, 18.2.2, 18.2.3: R6 set to a C4 value above R4 turns on the border during the adjustment lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_r6_equal_to_r4_also_covers_adjustment_lines() {
            // ACCC 18.2.4: C4 stays at R4 during adjustment, so the last character and the R5 lines are both border.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_vsync_never_occurs_when_r7_exceeds_r4_plus_one() {
            // ACCC 28.1.1: with R4=36, R9=7, R5=16, C4 reaches at most 37, so R7>37 gives no VSYNC.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_vsync_never_occurs_when_r7_exceeds_highest_adjustment_c4() {
            // ACCC 28.1.1: with R4=36, R9=7, R5=16, C4 reaches 38, so R7>38 gives no VSYNC.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_vsync_never_occurs_when_r7_exceeds_r4() {
            // ACCC 28.1.1: C4 never exceeds R4, so R7>R4 gives no VSYNC.
            todo!()
        }
    }

    mod rupture_for_dummies {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_1_r5_changed_from_zero_at_c0_equal_r0_triggers_rfd() {
            // ACCC 11.6: writing R5>0 exactly at C0=R0 while R5 was 0 makes VMA reload from R12/R13 at line start whatever C4.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r5_changed_from_non_zero_does_not_trigger_rfd() {
            // ACCC 11.6: R5>0 -> 0 and R5>0 -> another R5>0 do not trigger the bug.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_rfd_without_adjustment_when_r5_reset_to_zero() {
            // ACCC 11.6: OUT R5,1 then OUT R5,0 triggers the RFD without adding adjustment lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_rfd_reload_state_ends_when_c9_equals_r9_at_c0_equal_r1() {
            // ACCC 11.6: the R12/R13 reload state stays on until C0=R1 on a line where C9=R9 (parity-aware).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_rfd_makes_c9_equal_r9_test_parity_aware() {
            // ACCC 11.6, 11.6.1: the C9=R9 test at C0=R1 includes frame parity, so on one frame out of two VMA' is never latched
            // and character rows repeat.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_rfd_does_not_change_c4_counting() {
            // ACCC 11.6, 11.6.1: C4 still increments whenever C9 reaches R9.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_ivm_on_off_fixes_frame_parity_for_rfd() {
            // ACCC 11.6.2: OUT R8,3 then OUT R8,0 on an even C9 with odd R9 fixes the parity; done before the RFD,
            // every line of every frame can take a new offset.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_enlarging_r0_at_c0_equal_r0_of_last_line_and_cancelling_last_line_triggers_rfd()
         {
            // ACCC 13.6.2, 13.7.1.2: R0 enlarged at C0=R0 of the last line (R5=0), then R4/R9 changed during the longer line, gives an RFD.
            todo!()
        }
    }

    mod horizontal_total {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_all_r0_increase_before_c0_reaches_old_r0_lengthens_current_line() {
            // ACCC 13.6.1-13.6.3: an R0 update that lands before C0=R0 is taken into account on the current line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r0_increase_at_c0_equal_old_r0_applies_from_next_line() {
            // ACCC 13.6.1-13.6.3: an update that lands at C0=R0 is too late; C0 still wraps to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r0_zero_keeps_c0_at_zero() {
            // ACCC 13.2.6, 13.6.3: with R0=0, C0 stays 0 (1 µs lines) and never overflows.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_124_r0_zero_keeps_vertical_counters_running() {
            // ACCC 13.3, 13.4, 13.5: R0 accepts any value; with R0=0, C9 and C4 are still managed on every 1 µs line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_zero_advances_c9_once_then_freezes_it() {
            // ACCC 13.2.4: on the first C0=0 with R0=0, C9 is computed once (e.g. C9=4, R9=7 -> 5) then frozen.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_zero_with_c9_not_equal_r9_freezes_all_vertical_counters() {
            // ACCC 13.2.1: all counters stay frozen while R0=0; 64 x 8 µs of R0=0 "forgets" 8 lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_zero_with_c9_equal_r9_increments_c4_once_then_freezes() {
            // ACCC 13.2.1, 13.2.6: C4 increments once on the second C0=0 without C9 returning to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_zero_on_last_line_starts_adjustment_that_persists_after_r0_restored() {
            // ACCC 13.2.6 examples: C0=R0=C4=R4=C9=R9=R5=0 gives C4=1; once R0>2 again, C9 counts until C9+1=R5.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r4_r5_and_r9_updates_are_ignored_while_r0_is_zero() {
            // ACCC 13.2.1, 13.2.3: C9 management is inhibited, so R4, R5 and R9 have no effect while R0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_zero_freezes_c4_and_c9_on_adjustment_line() {
            // ACCC 13.2.3: R0=0 on C0=0 of an adjustment line (C4=R4+1, C9=0) keeps them until C0 reaches 1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_one_on_last_line_adds_one_two_us_adjustment_line() {
            // ACCC 13.2.5: with R0=1, C0 never reaches 2, so adjustment is never disarmed;
            // after the last line comes a 2 µs line with C4=R4+1, C9=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_one_with_r4_and_r9_zero_alternates_c4_between_zero_and_one() {
            // ACCC 13.2.5, 20.3.1: 2 µs "frames" alternate C4=0 and C4=1, so R12/R13 are reloaded every 4 µs.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_one_with_r9_above_zero_reaches_one_less_c9_value() {
            // ACCC 13.2.7: with R0=1 and R9>0, the extra C4=1 line after C9=R9 costs one C9 increment.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_increasing_r0_at_c0_one_with_c9_equal_r9_overflows_c4() {
            // ACCC 13.7.2, 13.7.2.1: R0 raised from 1 at C0=1 while C9=R9 and C4<>R4 increments C4
            // without resetting C9.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_increasing_r0_at_c0_one_on_last_line_leaves_adjustment_active() {
            // ACCC 13.7.2.2: on a last line, C9 then counts up to R5 (to 31 when R5=0) before C4 returns to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_increasing_r0_at_c0_zero_does_not_increment_c4() {
            // ACCC 13.7.2: raising R0 from 1 at C0=0 avoids the C4 increment.
            todo!()
        }
    }

    mod horizontal_sync {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_012_hsync_output_starts_when_c0_reaches_r2() {
            // ACCC 6.1.2, 14.7.1, 15.1: HSYNC starts on C0=R2.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_hsync_output_starts_one_character_after_c0_reaches_r2() {
            // ACCC 13.1, 15.1, 27.6.5: the ASIC delays HSYNC to match the displayed character (C0=R2+1 of the CRTC).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_hsync_lasts_r3_low_nibble_characters() {
            // ACCC 14.1: HSYNC lasts R3l µs; the high nibble does not change the HSYNC width.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_01_hsync_width_zero_produces_no_hsync() {
            // ACCC 14.1, 14.6, 27.6.3, 28.1.5: with R3l=0 no HSYNC is generated.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_24_hsync_width_zero_produces_sixteen_character_hsync() {
            // ACCC 14.6, 27.6.4, 27.6.5, 28.1.5: R3l=0 gives a 16 µs HSYNC.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_hsync_can_continue_past_end_of_line() {
            // ACCC 15.4.2, 15.4.3: with R0=63, R2=50, R3=15, the HSYNC continues after C0 wraps to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r3_increase_during_hsync_extends_hsync() {
            // ACCC 14.5: changing R3l while C3l counts changes the HSYNC length.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_r3_set_to_current_c3_during_hsync_ends_hsync() {
            // ACCC 14.5, 14.5.1, 14.5.2, 14.5.4 (R3.JIT): R3l set to C3l ends the HSYNC right away.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_r3_set_to_current_c3_during_hsync_makes_c3_wrap() {
            // ACCC 14.5.3, 14.5.4.4: there is no R3.JIT on type 4; C3l goes past the new R3l, wraps at 16 and ends on its next match.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r3_set_below_c3_during_hsync_makes_c3_wrap_through_sixteen() {
            // ACCC 14.5, 14.5.1-14.5.3: C3l counts up to 15, wraps to 0 and ends when it reaches the new R3l.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_024_r3_set_to_zero_during_hsync_makes_c3_wrap_to_zero() {
            // ACCC 14.5.1, 14.5.2, 14.5.3: R3=0 is treated as a value to reach; the HSYNC lasts until C3l wraps to 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r3_set_to_zero_during_hsync_cancels_hsync() {
            // ACCC 14.5, 14.5.2: R3=0 means "no HSYNC" even during an HSYNC, so the current one stops.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r2_update_during_hsync_does_not_restart_hsync() {
            // ACCC 15.1, 15.3.1: C0=R2 is ignored while an HSYNC is in progress.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_hsync_is_not_retriggered_when_it_ends_on_c0_equal_r2() {
            // ACCC 15.3.1: two HSYNCs cannot be contiguous on type 0 if R3l was not modified on that position.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_hsync_restarts_without_c3_reset_when_r3_modified_where_it_ends_on_c0_equal_r2()
         {
            // ACCC 15.3.3: if R3l is changed on the position C0=R2+R3l where C0 equals R2 again,
            // a new HSYNC starts without C3l being reset.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_124_c3_overflows_when_c0_equals_r2_at_end_of_hsync() {
            // ACCC 15.3.1, 15.3.4, 15.3.5: if C0=R2 on C0=R2+R3l, C3l keeps counting to 15, wraps and the HSYNC lasts until it
            // reaches R3l again.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_overflowed_hsync_briefly_drops_before_continuing() {
            // ACCC 15.3.4: type 1 ends the HSYNC for an instant and immediately raises it again; type 2 keeps it high.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_one_us_lines_with_r2_zero_trigger_hsync_every_third_us() {
            // ACCC 15.3.2: with R0=0, R2=0, R3=1, type 0 skips the HSYNC on the 2nd C0=0 and starts the next on the 3rd.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_124_one_us_lines_with_r2_zero_give_infinite_hsync() {
            // ACCC 15.3.2: with R0=0, R2=0, R3=1, the HSYNC never ends; C3l overflows again and again.
            todo!()
        }
    }

    mod vertical_sync {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_all_vsync_starts_when_c4_reaches_r7() {
            // ACCC 6.1.2, 7.2, 16.1: VSYNC starts when C4 becomes R7.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vsync_starts_at_c0_zero_when_c4_becomes_r7() {
            // ACCC 16.4.1, 16.4.4: when C4 becomes R7 on C0=0, the VSYNC starts on C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vsync_length_is_counted_on_c0_zero() {
            // ACCC 16.1: VSYNC lines are counted each time C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_vsync_lasts_r3_high_nibble_lines() {
            // ACCC 14.1, 14.2: R3h sets the VSYNC length in lines (ROM default R3h=8).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_vsync_width_zero_means_sixteen_lines() {
            // ACCC 14.1, 14.2: R3h=0 gives a 16-line VSYNC.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_vsync_always_lasts_sixteen_lines() {
            // ACCC 14.1, 14.2, 28.1.4: R3h is ignored.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_r3_high_lowered_to_current_line_count_ends_vsync() {
            // ACCC 14.2: with R3h=9, writing 8 on the 8th VSYNC line ends the VSYNC on that line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_r3_high_lowered_below_current_line_count_makes_counter_wrap() {
            // ACCC 14.2: with R3h=9, writing 8 on the 9th line makes the counter wrap at 16, then run 8 more lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_014_vsync_is_generated_when_it_starts_during_hsync() {
            // ACCC 15.4.1-15.4.3: VSYNC during HSYNC is not a problem on these types (no ghost VSYNC).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r7_update_up_to_last_us_before_c4_reaches_r7_is_honoured() {
            // ACCC 16.4: setting R7 to the next C4 on C4-1, C9=R9, C0=R0 still starts the VSYNC when C4=R7.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r7_change_during_vsync_neither_restarts_nor_stops_it() {
            // ACCC 16.3: C4 vs R7 is ignored during VSYNC; a new R7 does not start a new VSYNC or end the current one.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vsync_retriggers_when_c4_leaves_and_returns_to_r7_during_vsync() {
            // ACCC 16.3: with R7=0, R4=1, R9=7, R3h=0, C4 returns to 0 on line 17 and a new VSYNC starts right away.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_r7_set_to_current_c4_mid_line_triggers_vsync_immediately() {
            // ACCC 16.4.1.1, 16.4.2, 16.4.3: R7=C4 written mid-line starts the VSYNC during the line, on any C9.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_mid_line_vsync_starts_line_count_on_next_c0_zero() {
            // ACCC 16.4.1.1, 28.1.3: the VSYNC line counter starts at 0 and first counts at the next C0=0,
            // so the VSYNC is R0-C0 µs longer.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_mid_line_vsync_counts_current_line_as_first_line() {
            // ACCC 16.4.2, 16.4.3, 28.1.3: the line counts as if the VSYNC started at C0=0, so the VSYNC is C0+1 µs shorter.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_r7_set_to_current_c4_mid_frame_does_not_trigger_vsync() {
            // ACCC 15.4.1, 16.4.4, 28.1.3: VSYNC only starts when C4=R7 with C9=C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r7_set_to_current_c4_at_c0_below_two_blocks_vsync() {
            // ACCC 13.2.2, 16.4.1.1: an R7=C4 update at C0<2 gives a blocked VSYNC; no VSYNC for this C4.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_blocked_vsync_is_released_when_c4_or_r7_changes() {
            // ACCC 16.3, 16.4.1: the block is lifted once the C4==R7 comparison changes.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_vsync_is_not_retriggered_while_c4_stays_equal_to_r7() {
            // ACCC 16.3: with R7=0 and R4=0, C4 stays 0 after the VSYNC ends and no new VSYNC starts.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_vsync_retriggers_immediately_while_c4_stays_equal_to_r7() {
            // ACCC 16.3, 16.4.4: there is no second protection; with R7=0 and R4=0 a new VSYNC starts as soon as one ends.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_below_two_on_line_before_vsync_blocks_vsync() {
            // ACCC 13.2.2, 16.4.1.2: C0 must reach 2 on the line before C4=R7, or the VSYNC is blocked.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_zero_on_first_vsync_line_freezes_vsync_line_counter() {
            // ACCC 16.4.1.2: the VSYNC starts on C0=0 but with R3h=1 it does not end, because C3h is frozen.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_r0_one_on_first_vsync_line_lets_vsync_line_counter_advance() {
            // ACCC 16.4.1.2: with R3h=1, the VSYNC ends after 2 µs.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_vsync_condition_during_hsync_starts_ghost_vsync() {
            // ACCC 7.3, 13.4, 15.4.4, 16.4.3: if C4=R7 is evaluated between C0=R2 and C0=R2+R3, lines are counted as a VSYNC
            // but the VSYNC output stays low.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r7_set_to_current_c4_during_hsync_starts_ghost_vsync() {
            // ACCC 15.4.4, 15.6, 16.4.3.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_ghost_vsync_blocks_new_vsync_until_it_ends() {
            // ACCC 15.4.4, 16.4.3: no real VSYNC can start while the ghost VSYNC is counting.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r2_zero_avoids_ghost_vsync() {
            // ACCC 15.4.4, 15.6: with R2=0 the VSYNC condition is handled before the HSYNC starts.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_hsync_crossing_into_vsync_line_prevents_vsync() {
            // ACCC 15.6, 28.1.2: with R0=63, R2=50, R3=14, the HSYNC covers C0=0 of the C4=R7 line and there is no VSYNC.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_shortening_r3_before_vsync_evaluation_avoids_ghost_vsync() {
            // ACCC 15.4.4: reducing R3 so the HSYNC is over when C4 reaches R7 gives a normal VSYNC.
            todo!()
        }
    }

    mod display_enable {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_all_display_is_enabled_from_c0_zero_until_c0_reaches_r1() {
            // ACCC 6.1.3, 17.1: characters are displayed while 0 <= C0 < R1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_display_is_disabled_once_c4_reaches_r6() {
            // ACCC 6.1.3, 18.1: the border is shown from the first line of character row R6, whatever C9.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r6_border_lasts_until_next_frame() {
            // ACCC 18.1, 18.2.1: once the R6 border is active, display only resumes at C4=C9=C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r6_border_takes_priority_over_r1() {
            // ACCC 18.2.1-18.2.3: once R6 applies, R1 no longer turns the display on.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r1_zero_disables_all_characters() {
            // ACCC 17.1: with R1=0, no characters are displayed.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r1_update_is_considered_immediately_within_line() {
            // ACCC 17.3: C0=R1 is evaluated immediately and can happen several times per line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r1_equal_r0_shows_one_border_character_at_end_of_line() {
            // ACCC 17.6.1: with R1=R0, the last character (C0=R0) is border.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_r6_set_to_current_c4_enables_border_immediately() {
            // ACCC 18.2.2, 18.2.3: C4=R6 is taken into account at once, on any C0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_r6_is_only_checked_at_start_of_line() {
            // ACCC 18.2.4, 18.3.4: R6 changes mid-line have no effect until the next C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_02_r1_above_r0_generates_border_at_c0_equal_r0() {
            // ACCC 13.4, 17.6.2, 19.2.4, 28.1.6: without C0=R1, border is generated half a character after C0=R0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_14_r1_above_r0_generates_no_border_between_lines() {
            // ACCC 17.6.2, 19.2.4: no border byte appears when C0 wraps.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_c0_wrapping_after_overflow_does_not_enable_display() {
            // ACCC 17.1: if C0 wraps to 0 after reaching 255 (overflow), the display is not turned on.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_02_r6_zero_on_first_line_alternates_border_and_characters() {
            // ACCC 18.3.2, 28.1.6: with C4=C9=0 and R6=0, each character is half display, half border.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_02_r6_zero_conflict_becomes_definitive_border_at_c0_equal_r1() {
            // ACCC 18.3.2: if R6 is still 0 when C0=R1 on the first line, the border stays on until next frame.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_02_r6_zero_conflict_cancelled_by_raising_r6_on_first_line() {
            // ACCC 18.3.2: setting R6>0 on the first line (C0=R1 not reached) lifts the border on the next line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r6_zero_shows_border_while_it_stays_zero() {
            // ACCC 18.2.3, 18.3.3: with C4>0, R6=0 shows border and setting R6 back to a value other than C4 removes it.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r6_zero_while_c4_is_zero_makes_border_definitive() {
            // ACCC 18.2.3, 18.3.3: the border then lasts until C4=C9=C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r6_zero_on_last_character_of_frame_is_not_definitive() {
            // ACCC 18.2.3: if C4=R6=0 was already true at C0=R0 of the previous frame's last line, raising R6 cancels the border.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_r6_zero_mid_line_on_first_row_has_no_effect_until_next_frame() {
            // ACCC 18.2.4, 18.3.4: R6=0 set when C4=C9=0 but C0>0 is not special-cased.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_c0_zero_during_hsync_does_not_disable_border() {
            // ACCC 13.4, 15.5.2, 15.6: the border from the previous line stays on when C0 wraps during an HSYNC.
            todo!()
        }
    }

    mod video_pointer {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_all_video_address_bit_0_is_always_zero() {
            // ACCC 4.1, 20.2: each CRTC character is a 16-bit word, so address bit 0 is 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_video_address_bits_1_to_10_come_from_vma_bits_0_to_9() {
            // ACCC 20.2.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_video_address_bits_11_to_13_come_from_c9_bits_0_to_2() {
            // ACCC 10.1, 20.2: C9 bits 3 and 4 are not part of the address (C9=8 is shown as line 0).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_video_address_bits_14_and_15_come_from_vma_bits_12_and_13() {
            // ACCC 20.2, 20.5: R12 bits 4-5 select the 16 KB page.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vma_is_loaded_from_r12_r13_at_start_of_frame() {
            // ACCC 17.4.1, 17.4.2, 20.3: the first line of a frame starts at the R12/R13 address.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vma_increments_every_character_including_border() {
            // ACCC 17.1, 17.3, 17.4.2, 18.1: VMA counts on every character, displayed or not.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vma_is_latched_at_c0_equal_r1_on_last_raster_of_character_row() {
            // ACCC 17.1, 17.2.1: when C0=R1 and C9=R9, VMA is copied to VMA'.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vma_is_reloaded_from_latch_at_start_of_each_line() {
            // ACCC 17.1, 17.2.1: at C0=0, VMA=VMA', so every raster line of a character row starts at the same address.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r1_above_r0_repeats_same_character_row() {
            // ACCC 17.2.2, 17.4: C0 never equals R1, VMA' is never updated and character rows repeat.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_r1_reprogrammed_in_border_latches_vma_without_displaying() {
            // ACCC 17.3: a second C0=R1 on C9=R9 during border updates VMA'.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vma_wraps_within_16k_page_unless_r12_bits_2_and_3_are_set() {
            // ACCC 20.5: VMA is a 14-bit counter; carries go into the page bits only when R12 bits 2 and 3 are both 1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_vma_carry_changes_page_when_r12_bits_2_and_3_are_set() {
            // ACCC 17.4.2, 20.5 (overscan bits): with R12 bits 2-3 set, a carry past the 10 address bits changes
            // bits 14-15, allowing screens larger than 16 KB.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_024_r12_r13_update_mid_frame_takes_effect_on_next_frame() {
            // ACCC 20.3.1, 20.3.3, 20.3.4: the pointer is only reloaded when C4=C9=C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_014_r12_r13_update_just_before_frame_start_is_used_for_new_frame() {
            // ACCC 20.3.1, 20.3.2, 20.3.4: R12/R13 changes are taken into account immediately; a write before C0=0 of the new frame applies.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_014_r1_above_r0_still_starts_frame_at_r12_r13() {
            // ACCC 17.4.1, 17.4.2: the first line of the frame starts at R12/R13 whatever R1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_vma_is_loaded_from_r12_r13_on_every_line_while_c4_is_zero() {
            // ACCC 17.4.2, 20.3.2: VMA (not VMA') is reloaded from R12/R13 at each C0=0 of character row 0, whatever C9.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r12_r13_can_change_on_every_one_us_line_while_c4_is_zero() {
            // ACCC 20.3.2: with R0=0, the offset can change on each 1 µs line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r1_above_r0_repeats_last_latched_row_after_first_row() {
            // ACCC 17.4.2: row 0 comes from R12/R13, all later rows repeat the frozen VMA'.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_r1_raised_above_r0_exactly_at_c0_equal_r1_skips_latch() {
            // ACCC 17.4.2: an R1 update landing exactly on C0=R1 cancels that C0=R1 event; one character later it still latches.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_vma_prime_is_loaded_from_r12_r13_at_c0_equal_r1_on_last_line() {
            // ACCC 12.1, 13.4, 17.4.3, 20.3.3: R12/R13 go to VMA' at C0=R1 of the last line; VMA=VMA' at frame start.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r12_r13_written_after_c0_equal_r1_on_last_line_miss_next_frame() {
            // ACCC 13.4, 20.3.3: the offset change is only picked up if it is written before C0 reaches R1 on the last line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r1_above_r0_never_loads_r12_r13() {
            // ACCC 13.4, 17.4.3: without C0=R1 no pointer is ever loaded from R12/R13 and all lines are identical.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r1_raised_above_r0_exactly_at_c0_equal_r1_is_too_late() {
            // ACCC 17.4.3: VMA' has already been latched with the old R1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r1_zero_on_last_line_keeps_offset_for_that_line() {
            // ACCC 17.4.3, 20.3.3: C0=R1=0 on the last line happens before the last line state is set, so VMA'=VMA at its start.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r12_r13_written_after_c0_zero_of_last_line_with_r1_zero_are_anded_into_vma_prime()
         {
            // ACCC 17.4.3: only the AND half of the R12/R13 load happens: VMA' = VMA' & (R12 x 256 + R13).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_r1_raised_from_zero_on_last_line_repeats_last_line_on_new_frame() {
            // ACCC 17.4.3: VMA' is loaded from VMA instead of R12/R13, so the first line of the new frame repeats the last line.
            todo!()
        }
    }

    mod skew {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_04_r8_border_on_function_disables_display_immediately() {
            // ACCC 19.1, 19.2.1: R8=%001100xx (non-output) turns DISPEN off at once.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_r8_border_on_function_keeps_video_pointer_counting() {
            // ACCC 19.2.1: VMA keeps incrementing and is still latched at C0=R1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_r8_border_on_function_does_not_affect_r6_border_state() {
            // ACCC 19.2.1: it is independent of C4=R6.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_r8_border_off_restores_normal_display() {
            // ACCC 19.2.2: R8=%000000xx stops all skew functions.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_r8_one_character_skew_delays_display_enable_by_one_character() {
            // ACCC 19.2.3, 19.2.4: R8=%000100xx delays the R1 border start/end by one character.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_r8_two_character_skew_delays_display_enable_by_two_characters() {
            // ACCC 19.2.3, 19.2.4: R8=%001000xx delays the R1 border start/end by two characters.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_r8_skew_bits_are_ignored() {
            // ACCC 19.1, 28.1.6: only bits 0-1 of R8 are used on these types.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_skew_counts_from_c0_transitions_when_r1_above_r0() {
            // ACCC 19.2.4, 19.2.5: with R1>R0, C0=R0 replaces C0=R1 as border trigger, and the skew delay applies to it.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_skew_reprogrammed_on_next_line_can_cancel_border_between_lines() {
            // ACCC 19.2.5: with R0=R1=63, changing skew across the line boundary can remove the border character.
            todo!()
        }
    }

    mod interlace {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        #[ignore]
        fn test_type_all_r8_interlace_values_0_and_2_disable_interlace() {
            // ACCC 19.1: R8 bits 0-1: 00 and 10 = no interlace, 01 = sync, 11 = sync and video.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_frame_parity_is_tracked_regardless_of_r8() {
            // ACCC 19.5.2-19.5.5: the parity state keeps switching even with interlace off.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_interlace_extra_line_follows_r5_adjustment_lines() {
            // ACCC 11.1, 11.9, 19.6: the interlace line is added after the R5 lines.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_interlace_extra_line_depends_on_r8_value_on_last_line() {
            // ACCC 11.9: the condition is evaluated on the last line at C0=R0, which can be an R5 adjustment line.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_interlace_delays_vsync_to_half_line_on_even_frame() {
            // ACCC 16.5, 19.3.1, 19.7: with R8=1 or 3 on an even frame, VSYNC starts at C0=R0/2 (31 for R0=63).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_all_interlace_vsync_is_not_delayed_on_odd_frame() {
            // ACCC 19.7: MID-VSYNC only happens when ParityFrame is even (except type 4 with R7=0).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_02_frame_parity_toggles_when_c4_reaches_r6() {
            // ACCC 19.5.2, 19.5.4: ParityR6 = ParityFrame xor 1 when C4 reaches R6; ParityFrame = ParityR6 at C4=C9=C0=0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_02_frame_parity_freezes_when_r6_is_above_r4() {
            // ACCC 19.5.2, 19.5.4, 19.6.1, 19.6.3: if C4 never reaches R6, ParityR6 keeps its last value.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_02_interlace_adds_one_line_at_end_of_frame_when_parity_r6_is_odd() {
            // ACCC 19.6.1, 19.6.3: with R8=1 or 3, an extra line is added when ParityR6 is odd (i.e. after an even frame).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_02_interlace_extra_line_every_frame_when_parity_frozen_odd() {
            // ACCC 19.6.1, 19.6.3: with R6>R4 and ParityR6 odd, an extra line is added to every frame.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_14_frame_parity_toggles_at_every_frame_start() {
            // ACCC 19.5.3, 19.5.5: ParityFrame switches when C4=C9=C0=0, whatever R6 and R8.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_14_interlace_adds_one_line_at_end_of_even_frame() {
            // ACCC 19.6.2, 19.6.4: the extra line depends on ParityFrame only, not on C4=R6.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_c4_is_incremented_once_for_adjustment_and_interlace_lines() {
            // ACCC 19.6.1: C4=R4+1 for all R5 and interlace lines together.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_c4_is_incremented_again_for_interlace_line() {
            // ACCC 11.2.3, 19.6.2, 19.6.3: the interlace line is counted like one more R5 line, so it can bump C4 once more.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_interlace_line_keeps_c4_at_r4_and_c9_at_zero() {
            // ACCC 11.2.6, 19.6.4: C4 is not incremented and the extra line always has C9=0, whatever the parity.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_interlace_line_starts_at_vma_latched_on_last_raster() {
            // ACCC 19.6.4: the first extra line starts at the VMA latched at C9=R9, C0=R1 of C4=R4.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_012_mid_vsync_with_r7_zero_uses_new_frame_parity() {
            // ACCC 19.7.2: parity switches before the VSYNC check, so with R7=0 an even new frame still gets a MID-VSYNC.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_mid_vsync_with_r7_zero_uses_previous_frame_parity() {
            // ACCC 19.7.3: VSYNC is checked before the parity switch, so with R7=0 a MID-VSYNC follows an even previous frame.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_interlace_video_mode_character_has_r9_plus_two_lines() {
            // ACCC 19.3.3, 19.4.1, 19.4.4: in IVM (R8=3), R9=N-2 gives N-line characters (R9=6 -> 8 lines, 4 per frame).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_interlace_character_has_r9_plus_one_lines() {
            // ACCC 19.4.2, 19.4.3: in both interlace modes R9=N-1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_interlace_video_mode_uses_c9_shifted_left_with_parity_in_address() {
            // ACCC 19.8.1: in IVM, C9 keeps counting by 1 but the address uses (C9 x 2) | ParityC9.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_interlace_video_mode_ends_character_when_doubled_c9_matches_r9() {
            // ACCC 19.8.1: end of character when ((C9 x 2) | ParityFrame) == (R9 + ParityFrame).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_0_interlace_video_mode_applies_from_line_after_r8_write() {
            // ACCC 19.8.1: on the line R8 becomes 3, the plain C9 is compared, but parity applies at once.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_interlace_video_mode_counts_c9_in_steps_of_two() {
            // ACCC 19.8.2: with R9 odd, C9 advances by 2 and the C9/R9 test ignores bit 0; C9 restarts at ParityC9.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_interlace_video_mode_with_even_r9_alternates_parity_per_c4() {
            // ACCC 19.5.3, 19.8.2: an even R9 means an odd line count, so ParityC9 flips on each new C4.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_1_interlace_video_mode_does_not_delay_vsync_for_odd_line_characters() {
            // ACCC 16.5.2, 19.5.3: with R9 even, VSYNC on an odd C4 is one line off between frames.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_interlace_video_mode_counts_c9_in_steps_of_two_with_parity() {
            // ACCC 19.8.4: with R8=3, C9 = (C9 + 2) | ParityC9 until C9 >= R9, then C9 restarts at ParityC9.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_4_enabling_interlace_sets_c9_parity_from_current_c9() {
            // ACCC 19.5.5: when R8 changes to 1 or 3, ParityC9 = C9 bit 0.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_interlace_video_mode_doubles_character_row_rate() {
            // ACCC 19.3.3, 28.1.7: with R9=6 and R7 unchanged, VSYNC comes twice as early (4-line characters).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_interlace_video_mode_with_odd_r9_alternates_line_parity_per_c4() {
            // ACCC 19.5.2, 19.5.5: with R9 odd, ParityC9 = C4.0 xor ParityFrame
            // (R9=7, even frame, C4=0: 5 even lines then 4 odd lines on type 0).
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_04_interlace_video_mode_with_odd_r9_delays_vsync_by_one_line_on_odd_frame_and_odd_c4()
         {
            // ACCC 16.5.1, 16.5.4, 19.7.1.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_interlace_video_mode_does_not_change_c4_counting() {
            // ACCC 19.4.3, 19.8.3, 28.1.7: R4, R5, R6 and R7 do not need to change; C9 is compared with R9 as usual.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_interlace_video_mode_uses_separate_display_counter() {
            // ACCC 19.4.3, 19.8.3: C9.IVM drives the address; it restarts at C9=0 and when C9=R9/2, so VMA' can be latched twice per C4.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_interlace_video_mode_latches_vma_when_c9_reaches_half_r9() {
            // ACCC 19.8.3: with R8=3, VMA'=VMA at C0=R1 on C9=R9/2 without incrementing C4.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_interlace_video_mode_parity_applies_immediately() {
            // ACCC 19.5.4: turning R8 to 3 or 0 changes the displayed C9 parity from the next character.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_enabling_interlace_video_mode_on_first_line_of_odd_frame_adds_line() {
            // ACCC 19.5.4, 19.6.3: that line becomes an extra line and line 0 is shown again, so the frame grows by R0+1 µs.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_2_disabling_interlace_video_mode_during_extra_line_skips_c4_reset() {
            // ACCC 19.6.3: C4 is not reset on the next line; C9 counts to R9 and C4 keeps incrementing.
            todo!()
        }

        #[test]
        #[ignore]
        fn test_type_12_interlace_line_counts_as_additional_r5_line() {
            // ACCC 11.2.3: with R4=37, R9=7, R5=7 the interlace line belongs to C4=38; with R5=8 it gets C4=39.
            todo!()
        }
    }
}
