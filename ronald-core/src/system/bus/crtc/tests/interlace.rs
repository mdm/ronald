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
