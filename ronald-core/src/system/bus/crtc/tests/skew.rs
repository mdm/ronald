#[allow(unused_imports)]
use super::*;

#[test]
#[ignore]
fn test_type_04_r8_border_on_function_disables_display_immediately() {
    // ACCC 19.1, 19.2.1: R8=%001100xx (non-output) turns DISPEN off at once.
    todo!()
}

#[test]
#[ignore]
fn test_type_04_r8_border_on_function_keeps_video_pointer_counting() {
    // ACCC 19.2.1: VMA keeps incrementing and is still latched at C0=R1.
    todo!()
}

#[test]
#[ignore]
fn test_type_04_r8_border_on_function_does_not_affect_r6_border_state() {
    // ACCC 19.2.1: it is independent of C4=R6.
    todo!()
}

#[test]
#[ignore]
fn test_type_04_r8_border_off_restores_normal_display() {
    // ACCC 19.2.2: R8=%000000xx stops all skew functions.
    todo!()
}

#[test]
#[ignore]
fn test_type_04_r8_one_character_skew_delays_display_enable_by_one_character() {
    // ACCC 19.2.3, 19.2.4: R8=%000100xx delays the R1 border start/end by one character.
    todo!()
}

#[test]
#[ignore]
fn test_type_04_r8_two_character_skew_delays_display_enable_by_two_characters() {
    // ACCC 19.2.3, 19.2.4: R8=%001000xx delays the R1 border start/end by two characters.
    todo!()
}

#[test]
#[ignore]
fn test_type_12_r8_skew_bits_are_ignored() {
    // ACCC 19.1, 28.1.6: only bits 0-1 of R8 are used on these types.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_skew_counts_from_c0_transitions_when_r1_above_r0() {
    // ACCC 19.2.4, 19.2.5: with R1>R0, C0=R0 replaces C0=R1 as border trigger, and the skew delay applies to it.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_skew_reprogrammed_on_next_line_can_cancel_border_between_lines() {
    // ACCC 19.2.5: with R0=R1=63, changing skew across the line boundary can remove the border character.
    todo!()
}
