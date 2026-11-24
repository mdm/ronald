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
