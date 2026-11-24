use crate::system::clock::MasterClock;

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
fn test_type_04_vsync_lasts_r3_high_nibble_lines() {
    // ACCC 14.1, 14.2: R3h sets the VSYNC length in lines (ROM default R3h=8).

    for crtc in &mut crtcs!(Type0, Type4) {
        let mut clock = MasterClock::default();

        for (register, value) in [(4, 38), (7, 30), (3, 0xff)] {
            crtc.write_byte(0xbc00, register);
            crtc.write_byte(0xbd00, value);
        }

        let mut vsync_length = 0;
        for _ in 0..=(30 + 0x10 + 1) {
            if crtc.read_vertical_sync() {
                vsync_length += 1;
            }
            crtc.step(clock.current());
            clock.step(16);
        }

        assert_eq!(vsync_length, 0x0f);
    }
}

#[test]
fn test_type_04_vsync_width_zero_means_sixteen_lines() {
    // ACCC 14.1, 14.2: R3h=0 gives a 16-line VSYNC.

    for crtc in &mut crtcs!(Type0, Type4) {
        let mut clock = MasterClock::default();

        for (register, value) in [(4, 38), (7, 30), (3, 0x00)] {
            crtc.write_byte(0xbc00, register);
            crtc.write_byte(0xbd00, value);
        }

        let mut vsync_length = 0;
        for _ in 0..=(30 + 0x10 + 1) {
            if crtc.read_vertical_sync() {
                vsync_length += 1;
            }
            crtc.step(clock.current());
            clock.step(16);
        }

        assert_eq!(vsync_length, 16);
    }
}

#[test]
fn test_type_12_vsync_always_lasts_sixteen_lines() {
    // ACCC 14.1, 14.2, 28.1.4: R3h is ignored.

    for crtc in &mut crtcs!(Type1, Type2) {
        let mut clock = MasterClock::default();

        for (register, value) in [(4, 38), (7, 30), (3, 0xff)] {
            crtc.write_byte(0xbc00, register);
            crtc.write_byte(0xbd00, value);
        }

        let mut vsync_length = 0;
        for _ in 0..=(30 + 0x10 + 1) {
            if crtc.read_vertical_sync() {
                vsync_length += 1;
            }
            crtc.step(clock.current());
            clock.step(16);
        }

        assert_eq!(vsync_length, 16);
    }
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
