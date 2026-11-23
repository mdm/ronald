use crate::system::clock::MasterClock;

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
fn test_type_all_hsync_lasts_r3_low_nibble_characters() {
    // ACCC 14.1: HSYNC lasts R3l µs; the high nibble does not change the HSYNC width.

    for crtc in &mut crtcs!(All) {
        let mut clock = MasterClock::default();

        for (register, value) in [(0, 63), (2, 46), (3, 0xff)] {
            crtc.write_byte(0xbc00, register);
            crtc.write_byte(0xbd00, value);
        }

        let mut hsync_width = 0;
        for _ in 0..=63 {
            if crtc.read_horizontal_sync() {
                hsync_width += 1;
            }
            crtc.step(clock.current());
            clock.step(16);
        }

        assert_eq!(hsync_width, 0x0f);
    }
}

#[test]
fn test_type_01_hsync_width_zero_produces_no_hsync() {
    // ACCC 14.1, 14.6, 27.6.3, 28.1.5: with R3l=0 no HSYNC is generated.

    for crtc in &mut crtcs!(Type0, Type1) {
        let mut clock = MasterClock::default();

        for (register, value) in [(0, 63), (2, 46), (3, 0x00)] {
            crtc.write_byte(0xbc00, register);
            crtc.write_byte(0xbd00, value);
        }

        let mut hsync_seen = false;
        for _ in 0..=63 {
            hsync_seen |= crtc.read_horizontal_sync();
            crtc.step(clock.current());
            clock.step(16);
        }

        assert!(!hsync_seen, "HSYNC should never be active when R3l=0");
    }
}

#[test]
fn test_type_24_hsync_width_zero_produces_sixteen_character_hsync() {
    // ACCC 14.6, 27.6.4, 27.6.5, 28.1.5: R3l=0 gives a 16 µs HSYNC.

    for crtc in &mut crtcs!(Type2, Type4) {
        let mut clock = MasterClock::default();

        for (register, value) in [(0, 63), (2, 46), (3, 0x00)] {
            crtc.write_byte(0xbc00, register);
            crtc.write_byte(0xbd00, value);
        }

        let mut hsync_width = 0;
        for _ in 0..=63 {
            if crtc.read_horizontal_sync() {
                hsync_width += 1;
            }
            crtc.step(clock.current());
            clock.step(16);
        }

        assert_eq!(hsync_width, 16);
    }
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
fn test_type_0_hsync_restarts_without_c3_reset_when_r3_modified_where_it_ends_on_c0_equal_r2() {
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
