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
fn test_type_1_enlarging_r0_at_c0_equal_r0_of_last_line_and_cancelling_last_line_triggers_rfd() {
    // ACCC 13.6.2, 13.7.1.2: R0 enlarged at C0=R0 of the last line (R5=0), then R4/R9 changed during the longer line, gives an RFD.
    todo!()
}
