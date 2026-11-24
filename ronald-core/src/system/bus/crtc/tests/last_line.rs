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
