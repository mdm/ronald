use crate::system::clock::MasterClock;

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

            let value = dispatch!(crtc, crtc => crtc.registers.0[usize::from(register) & 0x1f]);

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

        let r9_value = dispatch!(crtc, crtc => crtc.registers.0[9]);

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

        let r5_value = dispatch!(crtc, crtc => crtc.registers.0[5]);

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

            let value = dispatch!(crtc, crtc => crtc.registers.0[usize::from(register)]);

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

        let before = dispatch!(crtc, crtc => crtc.registers.0);
        crtc.write_byte(0xbd00, 0x42);
        let after = dispatch!(crtc, crtc => crtc.registers.0);

        assert_ne!(before, after);

        for port in [0xbe00, 0xbf00] {
            let before = dispatch!(crtc, crtc => (crtc.selected_register, crtc.registers.0));
            crtc.write_byte(port, 0x42);
            let after = dispatch!(crtc, crtc => (crtc.selected_register, crtc.registers.0));

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
            dispatch_mut!(crtc, crtc => crtc.registers.0[usize::from(register)] = 0xff);

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
        dispatch_mut!(crtc, crtc => crtc.registers.0 = std::array::from_fn(|i| i as u8));

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
fn test_type_02_reading_undefined_registers_returns_zero() {
    // ACCC 21.2.1, 21.2.2, 21.4: reading R18-R31 returns 0; the "dummy" R31 does not exist.

    for crtc in &mut crtcs!(Type0, Type2) {
        for register in 0x12..=0x1f {
            dispatch_mut!(crtc, crtc => crtc.registers.0 = [0xff; 20]);

            crtc.write_byte(0xbc00, register);

            let value = crtc.read_byte(0xbf00);

            assert_eq!(value, 0x00);
        }
    }
}

#[test]
fn test_type_1_reading_register_31_returns_non_zero() {
    // ACCC 21.2.2, 21.4, 28.1.9: R31 (and any number whose bits 0-4 are all 1) reads as a non-zero value (127 or 255 observed).

    let mut crtc = CrtController::<Type1>::default();
    for register in 0..=0xff {
        if register & 0x1f != 0x1f {
            continue;
        }

        crtc.write_byte(0xbc00, register);

        let value = crtc.read_byte(0xbf00);

        assert_eq!(value, 0xff);
    }
}

#[test]
fn test_type_0_status_port_does_not_return_register_contents() {
    // ACCC 21.3.2: type 0 has no status register; the bus floats (255 or 127 observed).

    let mut crtc = CrtController::<Type0>::default();

    let status = crtc.read_byte(0xbe00);

    assert_eq!(status, 0xff); // TODO: properly emulate floating bus
}

#[test]
fn test_type_2_status_port_reads_255() {
    // ACCC 21.3.2: type 2 has no status register; &BE00 always read 255 on the test machine.

    let mut crtc = CrtController::<Type2>::default();

    let status = crtc.read_byte(0xbe00);

    assert_eq!(status, 0xff);
}

#[test]
fn test_type_1_status_register_unused_bits_read_as_zero() {
    // ACCC 21.3.3: bits 0-4 and 7 of the &BE00 status register read 0.

    let mut crtc = CrtController::<Type1>::default();
    crtc.impl_.light_pen_status = true;
    crtc.border_r6_condition = true;

    let status = crtc.read_byte(0xbe00);

    assert_eq!(status & !0x60, 0);
}

#[test]
fn test_type_1_status_bit_5_reflects_r6_border_state_updated_at_c0_equal_r0() {
    // ACCC 21.3.3: bit 5 becomes 1 at C0=R0 of the line before C4=R6, C9=0 and becomes 0 at C0=R0 of the line before C4=C9=0.

    let mut crtc = CrtController::<Type1>::default();
    assert!(
        !crtc.impl_.border_r6_status,
        "BORDER R6 status should be false initially"
    );
    let mut clock = MasterClock::default();

    crtc.registers[Register::R0HorizontalTotal] = 0x3f;
    crtc.registers[Register::R4VerticalTotal] = 0x26;
    crtc.registers[Register::R6VerticalDisplayed] = 0x19;
    crtc.registers[Register::R9MaximumRasterAddress] = 0x07;

    let mut intermediate_states = Vec::new();
    while crtc.c0_horizontal_counter != crtc.registers[Register::R0HorizontalTotal]
        || crtc.c4_character_row_counter != crtc.registers[Register::R6VerticalDisplayed]
        || crtc.c9_scan_line_counter != crtc.registers[Register::R9MaximumRasterAddress]
    {
        intermediate_states.push(crtc.impl_.border_r6_status);
        crtc.step(clock.current());
        clock.step(16);
    }

    assert!(
        intermediate_states.iter().all(|&status| !status),
        "BORDER R6 status should be false until the last line before C4=R6, C9=0"
    );
    assert!(
        crtc.impl_.border_r6_status,
        "BORDER R6 status should be true"
    );

    let mut intermediate_states = Vec::new();
    while crtc.c0_horizontal_counter != crtc.registers[Register::R0HorizontalTotal]
        || crtc.c4_character_row_counter != crtc.registers[Register::R4VerticalTotal]
        || crtc.c9_scan_line_counter != crtc.registers[Register::R9MaximumRasterAddress]
    {
        intermediate_states.push(crtc.impl_.border_r6_status);
        crtc.step(clock.current());
        clock.step(16);
    }

    assert!(
        intermediate_states.iter().all(|&status| status),
        "BORDER R6 status should be true until the last line before C4=C9=0"
    );
    assert!(
        !crtc.impl_.border_r6_status,
        "BORDER R6 status should be false"
    );
}

#[test]
fn test_type_1_status_bit_5_ignores_border_from_r6_zero() {
    // ACCC 21.3.3: setting R6=0 while C4>0 shows border but does not set bit 5.

    let mut crtc = CrtController::<Type1>::default();
    let mut clock = MasterClock::default();

    crtc.registers[Register::R0HorizontalTotal] = 0x3f;
    crtc.registers[Register::R4VerticalTotal] = 0x26;
    crtc.registers[Register::R6VerticalDisplayed] = 0x19;
    crtc.registers[Register::R9MaximumRasterAddress] = 0x07;

    crtc.step(clock.current());
    clock.step(16);

    assert!(
        !crtc.border_r6_condition,
        "BORDER R6 condition should be false initially"
    );

    crtc.registers[Register::R6VerticalDisplayed] = 0x00;
    crtc.step(clock.current());
    clock.step(16);

    assert!(
        crtc.border_r6_condition,
        "BORDER R6 condition should be true"
    );
    assert!(
        !crtc.impl_.border_r6_status,
        "BORDER R6 status should be false"
    );
}

#[test]
fn test_type_4_register_reads_use_only_three_bits_of_register_number() {
    // ACCC 21.2.3, 28.1.9: reads map numbers 0-7 to R16, R17, R10, R11, R12, R13, R14, R15, so reading R4 or R20 returns R12.

    let mut crtc = CrtController::<Type4> {
        registers: RegisterFile(std::array::from_fn(|i| i as u8)),
        ..Default::default()
    };
    for register in 0..=255 {
        crtc.write_byte(0xbc00, register);

        match register & 0x07 {
            0 => assert_eq!(crtc.read_byte(0xbf00), 16),
            1 => assert_eq!(crtc.read_byte(0xbf00), 17),
            2 => assert_eq!(crtc.read_byte(0xbf00), 10),
            3 => assert_eq!(crtc.read_byte(0xbf00), 11),
            4 => assert_eq!(crtc.read_byte(0xbf00), 12),
            5 => assert_eq!(crtc.read_byte(0xbf00), 13),
            6 => assert_eq!(crtc.read_byte(0xbf00), 14),
            7 => assert_eq!(crtc.read_byte(0xbf00), 15),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_type_4_status_port_mirrors_read_port() {
    // ACCC 21.2.3, 21.3.1, 28.1.8: &BE00 behaves exactly like &BF00.

    let mut crtc = CrtController::<Type4> {
        registers: RegisterFile(std::array::from_fn(|i| i as u8)),
        ..Default::default()
    };
    for register in 0..=255 {
        crtc.write_byte(0xbc00, register);

        assert_eq!(crtc.read_byte(0xbe00), crtc.read_byte(0xbf00));
    }
}

#[test]
fn test_type_4_status_1_bit_0_is_set_when_c0_equals_r0() {
    // ACCC 21.3.4.1: reading R10 returns status 1; bit 0 is 1 only while C0=R0.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        crtc.write_byte(0xbc00, 10);
        let status1 = crtc.read_byte(0xbf00);

        let condition = crtc.c0_horizontal_counter == crtc.registers[Register::R0HorizontalTotal];

        samples.push((status1, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(
        samples
            .iter()
            .all(|(status1, condition)| (status1 & 0x01 != 0) == *condition)
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _)| status1 & 0x01 != 0)
            .count(),
        (crtc.registers[Register::R4VerticalTotal] as usize + 1)
            * (crtc.registers[Register::R9MaximumRasterAddress] as usize + 1)
    );
}

#[test]
fn test_type_4_status_1_bit_1_is_cleared_when_c0_equals_half_r0() {
    // ACCC 21.3.4.1: bit 1 is 0 only while C0=R0/2.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 10);
        let status1 = crtc.read_byte(0xbf00);

        let condition =
            crtc.c0_horizontal_counter == crtc.registers[Register::R0HorizontalTotal] / 2;

        samples.push((status1, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(
        samples
            .iter()
            .all(|(status1, condition)| (status1 & (0x01 << 1) == 0) == *condition)
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _)| status1 & (0x01 << 1) == 0)
            .count(),
        (crtc.registers[Register::R4VerticalTotal] as usize + 1)
            * (crtc.registers[Register::R9MaximumRasterAddress] as usize + 1)
    );
}

#[test]
fn test_type_4_status_1_bit_2_is_cleared_when_c0_equals_r1_minus_one() {
    // ACCC 21.3.4.1: bit 2 is 0 while C0=R1-1 (if R0>=R1).

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 10);
        let status1 = crtc.read_byte(0xbf00);

        let condition = crtc.c0_horizontal_counter
            == crtc.registers[Register::R1HorizontalDisplayed].saturating_sub(1)
            && crtc.registers[Register::R0HorizontalTotal]
                >= crtc.registers[Register::R1HorizontalDisplayed];

        samples.push((status1, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(
        samples
            .iter()
            .all(|(status1, condition)| (status1 & (0x01 << 2) == 0) == *condition)
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _)| status1 & (0x01 << 2) == 0)
            .count(),
        (crtc.registers[Register::R4VerticalTotal] as usize + 1)
            * (crtc.registers[Register::R9MaximumRasterAddress] as usize + 1)
    );
}

#[test]
fn test_type_4_status_1_bits_3_and_4_are_cleared_at_hsync_start_and_end() {
    // ACCC 21.3.4.1: bit 3 is 0 while C0=R2; bit 4 is 0 while C0=R2+R3.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 10);
        let status1 = crtc.read_byte(0xbf00);

        let condition = crtc.read_horizontal_sync();

        samples.push((status1, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(samples.windows(2).all(|window| {
        let (_, before) = window[0];
        let (status1, condition) = window[1];

        match (before, condition) {
            (false, true) => status1 & (0x01 << 3) == 0 && status1 & (0x01 << 4) != 0,
            (true, false) => status1 & (0x01 << 3) != 0 && status1 & (0x01 << 4) == 0,
            _ => status1 & (0x01 << 3) != 0 && status1 & (0x01 << 4) != 0,
        }
    }));

    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _)| status1 & (0x01 << 3) == 0)
            .count(),
        (crtc.registers[Register::R4VerticalTotal] as usize + 1)
            * (crtc.registers[Register::R9MaximumRasterAddress] as usize + 1)
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _)| status1 & (0x01 << 4) == 0)
            .count(),
        (crtc.registers[Register::R4VerticalTotal] as usize + 1)
            * (crtc.registers[Register::R9MaximumRasterAddress] as usize + 1)
    );
}

#[test]
fn test_type_4_status_1_bit_5_tracks_last_vsync_line() {
    // ACCC 21.3.4.1: bit 5 is 0 on line R3h of the VSYNC (R3h>0), or 1 over 15 lines from the VSYNC start when R3h=0.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 10);
        let status1 = crtc.read_byte(0xbf00);

        let condition = crtc.active_vsync_scanlines == Type4::vsync_length(&crtc);

        samples.push((status1, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(
        samples
            .iter()
            .all(|(status1, condition)| (status1 & (0x01 << 5) == 0) == *condition)
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _)| status1 & (0x01 << 5) == 0)
            .count(),
        (crtc.registers[Register::R0HorizontalTotal] as usize + 1)
    );

    // special case VSYNC length == 16
    crtc.write_byte(0xbc00, 3);
    crtc.write_byte(0xbd00, 14);

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 10);
        let status1 = crtc.read_byte(0xbf00);

        let condition = crtc.active_vsync_scanlines >= Type4::vsync_length(&crtc)
            || (crtc.active_vsync_scanlines == 0
                && crtc.c4_character_row_counter
                    >= crtc.registers[Register::R7VerticalSyncPosition]);

        samples.push((status1, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(
        samples
            .iter()
            .all(|(status1, condition)| (status1 & (0x01 << 5) != 0) == *condition)
    );

    let chars_per_line = crtc.registers[Register::R0HorizontalTotal] as usize + 1;
    let rows_per_vsync = crtc.registers[Register::R4VerticalTotal] as usize
        - crtc.registers[Register::R7VerticalSyncPosition] as usize
        + 1;
    let lines_per_row = crtc.registers[Register::R9MaximumRasterAddress] as usize + 1;
    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _)| status1 & (0x01 << 5) != 0)
            .count(),
        chars_per_line * (rows_per_vsync * lines_per_row - 15)
    );
}

#[test]
fn test_type_4_status_1_bit_6_is_always_set() {
    // ACCC 21.3.4.1.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 10);
        let status1 = crtc.read_byte(0xbf00);

        assert!(status1 & (0x01 << 6) != 0);

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }
}

#[test]
fn test_type_4_status_1_bit_7_is_cleared_before_vma_low_byte_resets() {
    // ACCC 21.3.4.1: bit 7 is 0 when VMA's low byte is &FF (C0<R0) or when VMA' low byte is &00 at C0=R0.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 10);
        let status1 = crtc.read_byte(0xbf00);

        let condition = if crtc.c0_horizontal_counter < crtc.registers[Register::R0HorizontalTotal]
        {
            crtc.video_memory_address & 0x00ff == 0xff
        } else {
            crtc.video_memory_address_initial & 0x00ff == 0x00
        };

        samples.push((status1, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(
        samples
            .iter()
            .all(|(status1, condition)| (status1 & (0x01 << 7) == 0) == *condition)
    );
}

#[test]
fn test_type_4_status_2_bits_0_to_2_flag_last_char_of_screen_display_and_before_vsync() {
    // ACCC 21.3.4.2: bit 0 is 0 at C4=R4,C9=R9,C0=R0; bit 1 at C4=R6-1,C9=R9,C0=R0; bit 2 at C4=R7-1,C9=R9,C0=R0.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 11);
        let status2 = crtc.read_byte(0xbf00);

        let condition_common = crtc.c0_horizontal_counter
            == crtc.registers[Register::R0HorizontalTotal]
            && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress];

        let condition_bit0 = condition_common
            && crtc.c4_character_row_counter == crtc.registers[Register::R4VerticalTotal];

        let condition_bit1 = condition_common
            && crtc.c4_character_row_counter == crtc.registers[Register::R6VerticalDisplayed] - 1;

        let condition_bit2 = condition_common
            && crtc.c4_character_row_counter
                == crtc.registers[Register::R7VerticalSyncPosition] - 1;

        samples.push((status2, condition_bit0, condition_bit1, condition_bit2));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(samples.iter().all(
        |(status2, condition_bit0, condition_bit1, condition_bit2)| (status2 & (0x01 << 0) == 0)
            == *condition_bit0
            && (status2 & (0x01 << 1) == 0) == *condition_bit1
            && (status2 & (0x01 << 2) == 0) == *condition_bit2
    ));

    assert_eq!(
        samples
            .iter()
            .filter(|(status2, _, _, _)| status2 & (0x01 << 0) == 0)
            .count(),
        1
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _, _, _)| status1 & (0x01 << 1) == 0)
            .count(),
        1
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status1, _, _, _)| status1 & (0x01 << 2) == 0)
            .count(),
        1
    );
}

#[test]
fn test_type_4_status_2_bit_3_toggles_every_16_frames() {
    // ACCC 21.3.4.2: with line-to-line rupture it toggles every 16 lines.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    for frame in 0usize..64 {
        loop {
            crtc.write_byte(0xbc00, 11);
            let status2 = crtc.read_byte(0xbf00);

            let condition = (frame / 16).is_multiple_of(2);

            dbg!(
                crtc.c0_horizontal_counter,
                crtc.c4_character_row_counter,
                crtc.c9_scan_line_counter
            );

            assert!(
                (status2 & (0x01 << 3) == 0) == condition,
                "{}: {} != {}",
                frame,
                status2 & (0x01 << 3),
                condition
            );

            crtc.step(clock.current());
            clock.step(16);

            if crtc.c0_horizontal_counter == 0
                && crtc.c4_character_row_counter == 0
                && crtc.c9_scan_line_counter == 0
            {
                break;
            }
        }
    }
}

#[test]
fn test_type_4_status_2_constant_bits() {
    // ACCC 21.3.4.2: bit 4 is always 1 and bit 6 is always 0.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 11);
        let status2 = crtc.read_byte(0xbf00);

        assert!(status2 & (0x01 << 4) != 0);
        assert!(status2 & (0x01 << 6) == 0);

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }
}

#[test]
fn test_type_4_status_2_bit_5_is_cleared_on_last_raster_of_character() {
    // ACCC 21.3.4.2: bit 5 is 0 on every C0 of a line with C9=R9.

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 11);
        let status2 = crtc.read_byte(0xbf00);

        let condition =
            crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress];

        samples.push((status2, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(
        samples
            .iter()
            .all(|(status2, condition)| (status2 & (0x01 << 5) == 0) == *condition)
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status2, _)| status2 & (0x01 << 5) == 0)
            .count(),
        (crtc.registers[Register::R0HorizontalTotal] as usize + 1)
            * (crtc.registers[Register::R4VerticalTotal] as usize + 1)
    );
}

#[test]
fn test_type_4_status_2_bit_7_is_set_on_character_boundaries() {
    // ACCC 21.3.4.2: bit 7 is 1 when (C9=R9 and C0=R0) or (C9=0 and C0<R0).

    let mut crtc = CrtController::<Type4>::default();
    let mut clock = MasterClock::default();

    for (register, value) in CPC_DEFAULTS.iter() {
        crtc.write_byte(0xbc00, *register);
        crtc.write_byte(0xbd00, *value);
    }

    let mut samples = Vec::new();
    loop {
        crtc.step(clock.current());
        clock.step(16);

        // sample after stepping to ignore uninitialized status registers
        crtc.write_byte(0xbc00, 11);
        let status2 = crtc.read_byte(0xbf00);

        let condition = (crtc.c0_horizontal_counter == crtc.registers[Register::R0HorizontalTotal]
            && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress])
            || (crtc.c0_horizontal_counter < crtc.registers[Register::R0HorizontalTotal]
                && crtc.c9_scan_line_counter == 0);

        samples.push((status2, condition));

        if crtc.c0_horizontal_counter == 0
            && crtc.c4_character_row_counter == 0
            && crtc.c9_scan_line_counter == 0
        {
            break;
        }
    }

    assert!(
        samples
            .iter()
            .all(|(status2, condition)| (status2 & (0x01 << 7) != 0) == *condition)
    );

    assert_eq!(
        samples
            .iter()
            .filter(|(status2, _)| status2 & (0x01 << 7) != 0)
            .count(),
        (crtc.registers[Register::R0HorizontalTotal] as usize + 1)
            * (crtc.registers[Register::R4VerticalTotal] as usize + 1)
    );
}
