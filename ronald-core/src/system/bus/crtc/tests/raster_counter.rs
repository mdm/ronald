#[allow(unused_imports)]
use super::*;

#[test]
#[ignore]
fn test_type_all_r9_update_is_considered_up_to_c0_equal_r0() {
    // ACCC 10.2: setting R9 to 0 up to C0=R0 of a C9=0 line still ends the character on that line.
    // (The type 3/4 diagrams are shifted by one µs, which matches the ASIC's later I/O cycle, ACCC 4.4.3.)
    todo!()
}

#[test]
#[ignore]
fn test_type_all_setting_r9_to_current_c9_ends_character_on_next_line() {
    // ACCC 10.3.1.1, 10.3.2.1, 10.3.4.1: C9=3 and R9 changed from 7 to 3 gives C9=0 on the next line.
    todo!()
}

#[test]
#[ignore]
fn test_type_all_setting_r9_above_c9_increments_c9_without_changing_c4() {
    // ACCC 10.3.1.1, 10.3.2.1, 10.3 tables: C9=0 and R9 set to 7 gives C9=1 on the next line, C4 unchanged.
    todo!()
}

#[test]
#[ignore]
fn test_type_012_setting_r9_below_c9_overflows_c9_up_to_31() {
    // ACCC 10.3, 10.3.1.1, 10.3.2.1, 10.3.3: C9=3 and R9 set to 1 gives C9=4; C9 counts to 31 before looping to 0.
    todo!()
}

#[test]
#[ignore]
fn test_type_1_c9_overflow_keeps_c4_unchanged_until_c9_wraps() {
    // ACCC 10.3.2.1: while C9 overflows, C4 does not change; the offset is only reloaded if C4=C9=C0=0.
    todo!()
}

#[test]
#[ignore]
fn test_type_4_setting_r9_at_or_below_c9_resets_c9_on_next_line() {
    // ACCC 10.3.4.1, 12.5, 13.5: if C9 >= R9 the next C9 is 0 and C4 increments (or goes to 0 if C4=R4); C9 cannot overflow.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r9_changed_at_c0_equal_r0_on_character_end_increments_c4_and_c9() {
    // ACCC 10.3.1.2: if R9 changes exactly at C0=R0 while C9 was R9, C4 is still incremented,
    // so C4 and C9 can both increment at once.
    todo!()
}
