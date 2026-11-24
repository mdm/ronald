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
