use std::collections::HashSet;

use crate::system::clock::MasterClock;

use super::*;

#[test]
fn test_type_all_video_address_bit_0_is_always_zero() {
    // ACCC 4.1, 20.2: each CRTC character is a 16-bit word, so address bit 0 is 0.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        loop {
            crtc.step(clock.current());
            clock.step(16);

            assert_eq!(crtc.read_address() & 0x0001, 0);

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }
    }
}

#[test]
fn test_type_all_video_address_bits_1_to_10_come_from_vma_bits_0_to_9() {
    // ACCC 20.2.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        loop {
            crtc.step(clock.current());
            clock.step(16);

            assert_eq!(
                (crtc.read_address() as u16 >> 1) & 0x03ff,
                dispatch!(crtc, crtc => crtc.video_memory_address) & 0x03ff
            );

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }
    }
}

#[test]
fn test_type_all_video_address_bits_11_to_13_come_from_c9_bits_0_to_2() {
    // ACCC 10.1, 20.2: C9 bits 3 and 4 are not part of the address (C9=8 is shown as line 0).

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        loop {
            crtc.step(clock.current());
            clock.step(16);

            assert_eq!(
                (crtc.read_address() as u16 >> 11) & 0x0007,
                dispatch!(crtc, crtc => crtc.c9_scan_line_counter as u16)
            );

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }
    }
}

#[test]
fn test_type_all_video_address_bits_14_and_15_come_from_vma_bits_12_and_13() {
    // ACCC 20.2, 20.5: R12 bits 4-5 select the 16 KB page.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        loop {
            crtc.step(clock.current());
            clock.step(16);

            assert_eq!(
                (crtc.read_address() as u16 >> 14) & 0x0003,
                (dispatch!(crtc, crtc =>crtc.video_memory_address) >> 12) & 0x0003
            );

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }
    }
}

#[test]
fn test_type_all_vma_is_loaded_from_r12_r13_at_start_of_frame() {
    // ACCC 17.4.1, 17.4.2, 20.3: the first line of a frame starts at the R12/R13 address.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter().chain([(12, 0xbe), (13, 0xef)].iter()) {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        loop {
            crtc.step(clock.current());
            clock.step(16);

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }

        let exected = (dispatch!(crtc, crtc => crtc.registers[Register::R12DisplayStartAddressHigh] as u16)
            << 8)
            + dispatch!(crtc, crtc =>  crtc.registers[Register::R13DisplayStartAddressLow] as u16);
        assert_eq!(dispatch!(crtc, crtc => crtc.video_memory_address), exected);
    }
}

#[test]
fn test_type_all_vma_increments_every_character_including_border() {
    // ACCC 17.1, 17.3, 17.4.2, 18.1: VMA counts on every character, displayed or not.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        let mut samples = vec![dispatch!(crtc, crtc => crtc.video_memory_address)];
        loop {
            crtc.step(clock.current());
            clock.step(16);

            samples.push(dispatch!(crtc, crtc => crtc.video_memory_address));

            if dispatch!(crtc, crtc =>  crtc.c0_horizontal_counter == crtc.registers[Register::R0HorizontalTotal])
            {
                break;
            }
        }

        assert!(samples.windows(2).all(|window| window[0] + 1 == window[1]));
    }
}

#[test]
fn test_type_all_vma_is_latched_at_c0_equal_r1_on_last_raster_of_character_row() {
    // ACCC 17.1, 17.2.1: when C0=R1 and C9=R9, VMA is copied to VMA'.

    for crtc in &mut crtcs!(All) {
        dbg!();
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        loop {
            crtc.step(clock.current());
            clock.step(16);

            let exception = is_crtc_type!(crtc, Type2)
                && dispatch!(crtc, crtc => crtc.c4_character_row_counter
                            == crtc.registers[Register::R4VerticalTotal]);
            if !exception
                && dispatch!(crtc, crtc => crtc.c0_horizontal_counter
                        == crtc.registers[Register::R1HorizontalDisplayed]
                        && crtc.c9_scan_line_counter
                            == crtc.registers[Register::R9MaximumRasterAddress])
            {
                assert_eq!(
                    dispatch!(crtc, crtc => crtc.video_memory_address_initial),
                    dispatch!(crtc, crtc => crtc.video_memory_address),
                );
            }

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }
    }
}

#[test]
fn test_type_all_vma_is_reloaded_from_latch_at_start_of_each_line() {
    // ACCC 17.1, 17.2.1: at C0=0, VMA=VMA', so every raster line of a character row starts at the same address.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        loop {
            crtc.step(clock.current());
            clock.step(16);

            let exception = is_crtc_type!(crtc, Type1)
                && dispatch!(crtc, crtc => crtc.c4_character_row_counter == 0 );
            if !exception && dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0) {
                assert_eq!(
                    dispatch!(crtc, crtc => crtc.video_memory_address),
                    dispatch!(crtc, crtc => crtc.video_memory_address_initial),
                );
            }

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }
    }
}

#[test]
fn test_type_all_r1_above_r0_repeats_same_character_row() {
    // ACCC 17.2.2, 17.4: C0 never equals R1, VMA' is never updated and character rows repeat.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        crtc.write_byte(0xbc00, 1);
        crtc.write_byte(0xbd00, CPC_DEFAULTS[0].1 + 1);
        let expected = 0xbeef;
        dispatch_mut!(crtc, crtc => crtc.video_memory_address_initial = expected);

        loop {
            crtc.step(clock.current());
            clock.step(16);

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }

            assert_eq!(
                dispatch!(crtc, crtc => crtc.video_memory_address_initial),
                expected
            );
        }
    }
}

#[test]
fn test_type_all_r1_reprogrammed_in_border_latches_vma_without_displaying() {
    // ACCC 17.3: a second C0=R1 on C9=R9 during border updates VMA'.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        const R1: u8 = CPC_DEFAULTS[1].1;

        for (register, value) in CPC_DEFAULTS.iter() {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        let mut samples = HashSet::new();
        loop {
            crtc.step(clock.current());
            clock.step(16);

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter>crtc.registers[Register::R1HorizontalDisplayed] && crtc.c9_scan_line_counter == crtc.registers[Register::R9MaximumRasterAddress])
            {
                assert!(!crtc.read_display_enabled());
                crtc.write_byte(0xbc00, 1);
                crtc.write_byte(0xbd00, R1 + 8);
            }

            samples.insert(dispatch!(crtc, crtc => crtc.video_memory_address_initial));

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }

        let samples = samples
            .into_iter()
            .filter(|sample| *sample != 0)
            .collect::<Vec<_>>();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].abs_diff(samples[1]), 8);
    }
}

#[test]
fn test_type_all_vma_wraps_within_16k_page_unless_r12_bits_2_and_3_are_set() {
    // ACCC 20.5: VMA is a 14-bit counter; carries go into the page bits only when R12 bits 2 and 3 are both 1.

    for r12_bits_2_and_3 in 0..3 {
        for crtc in &mut crtcs!(All) {
            let mut clock = MasterClock::default();

            for (register, value) in CPC_DEFAULTS
                .iter()
                .chain([(4, 30), (12, 0x30 | (r12_bits_2_and_3 << 2))].iter())
            {
                crtc.write_byte(0xbc00, *register);
                crtc.write_byte(0xbd00, *value);
            }

            loop {
                crtc.step(clock.current());
                clock.step(16);

                if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
                {
                    break;
                }
            }

            let page = dispatch!(crtc, crtc => (crtc.video_memory_address >> 12) & 0b11);

            loop {
                crtc.step(clock.current());
                clock.step(16);

                assert_eq!(
                    dispatch!(crtc, crtc => (crtc.video_memory_address >> 12) & 0b11),
                    page
                );

                if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
                {
                    break;
                }
            }
        }
    }
}

#[test]
fn test_type_all_vma_carry_changes_page_when_r12_bits_2_and_3_are_set() {
    // ACCC 17.4.2, 20.5 (overscan bits): with R12 bits 2-3 set, a carry past the 10 address bits changes
    // bits 14-15, allowing screens larger than 16 KB.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in CPC_DEFAULTS
            .iter()
            .chain([(4, 30), (12, 0x30 | (0x03 << 2))].iter())
        {
            crtc.write_byte(0xbc00, *register);
            crtc.write_byte(0xbd00, *value);
        }

        loop {
            crtc.step(clock.current());
            clock.step(16);

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }

        let page = dispatch!(crtc, crtc => (crtc.video_memory_address >> 12) & 0b11);

        let mut changed = false;
        loop {
            crtc.step(clock.current());
            clock.step(16);

            if dispatch!(crtc, crtc => (crtc.video_memory_address >> 12) & 0b11) == (page + 1) % 4 {
                changed = true;
            }

            if dispatch!(crtc, crtc => crtc.c0_horizontal_counter == 0
                        && crtc.c4_character_row_counter == 0
                        && crtc.c9_scan_line_counter == 0)
            {
                break;
            }
        }

        assert!(changed);
    }
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
fn test_type_2_r12_r13_written_after_c0_zero_of_last_line_with_r1_zero_are_anded_into_vma_prime() {
    // ACCC 17.4.3: only the AND half of the R12/R13 load happens: VMA' = VMA' & (R12 x 256 + R13).
    todo!()
}

#[test]
#[ignore]
fn test_type_2_r1_raised_from_zero_on_last_line_repeats_last_line_on_new_frame() {
    // ACCC 17.4.3: VMA' is loaded from VMA instead of R12/R13, so the first line of the new frame repeats the last line.
    todo!()
}
