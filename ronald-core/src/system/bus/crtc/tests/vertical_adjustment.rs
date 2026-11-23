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
