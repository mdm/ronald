#[allow(unused_imports)]
use super::*;

#[test]
#[ignore]
fn test_type_all_r0_increase_before_c0_reaches_old_r0_lengthens_current_line() {
    // ACCC 13.6.1-13.6.3: an R0 update that lands before C0=R0 is taken into account on the current line.
    todo!()
}

#[test]
#[ignore]
fn test_type_all_r0_increase_at_c0_equal_old_r0_applies_from_next_line() {
    // ACCC 13.6.1-13.6.3: an update that lands at C0=R0 is too late; C0 still wraps to 0.
    todo!()
}

#[test]
#[ignore]
fn test_type_all_r0_zero_keeps_c0_at_zero() {
    // ACCC 13.2.6, 13.6.3: with R0=0, C0 stays 0 (1 µs lines) and never overflows.
    todo!()
}

#[test]
#[ignore]
fn test_type_124_r0_zero_keeps_vertical_counters_running() {
    // ACCC 13.3, 13.4, 13.5: R0 accepts any value; with R0=0, C9 and C4 are still managed on every 1 µs line.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r0_zero_advances_c9_once_then_freezes_it() {
    // ACCC 13.2.4: on the first C0=0 with R0=0, C9 is computed once (e.g. C9=4, R9=7 -> 5) then frozen.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r0_zero_with_c9_not_equal_r9_freezes_all_vertical_counters() {
    // ACCC 13.2.1: all counters stay frozen while R0=0; 64 x 8 µs of R0=0 "forgets" 8 lines.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r0_zero_with_c9_equal_r9_increments_c4_once_then_freezes() {
    // ACCC 13.2.1, 13.2.6: C4 increments once on the second C0=0 without C9 returning to 0.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r0_zero_on_last_line_starts_adjustment_that_persists_after_r0_restored() {
    // ACCC 13.2.6 examples: C0=R0=C4=R4=C9=R9=R5=0 gives C4=1; once R0>2 again, C9 counts until C9+1=R5.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r4_r5_and_r9_updates_are_ignored_while_r0_is_zero() {
    // ACCC 13.2.1, 13.2.3: C9 management is inhibited, so R4, R5 and R9 have no effect while R0=0.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r0_zero_freezes_c4_and_c9_on_adjustment_line() {
    // ACCC 13.2.3: R0=0 on C0=0 of an adjustment line (C4=R4+1, C9=0) keeps them until C0 reaches 1.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r0_one_on_last_line_adds_one_two_us_adjustment_line() {
    // ACCC 13.2.5: with R0=1, C0 never reaches 2, so adjustment is never disarmed;
    // after the last line comes a 2 µs line with C4=R4+1, C9=0.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r0_one_with_r4_and_r9_zero_alternates_c4_between_zero_and_one() {
    // ACCC 13.2.5, 20.3.1: 2 µs "frames" alternate C4=0 and C4=1, so R12/R13 are reloaded every 4 µs.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_r0_one_with_r9_above_zero_reaches_one_less_c9_value() {
    // ACCC 13.2.7: with R0=1 and R9>0, the extra C4=1 line after C9=R9 costs one C9 increment.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_increasing_r0_at_c0_one_with_c9_equal_r9_overflows_c4() {
    // ACCC 13.7.2, 13.7.2.1: R0 raised from 1 at C0=1 while C9=R9 and C4<>R4 increments C4
    // without resetting C9.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_increasing_r0_at_c0_one_on_last_line_leaves_adjustment_active() {
    // ACCC 13.7.2.2: on a last line, C9 then counts up to R5 (to 31 when R5=0) before C4 returns to 0.
    todo!()
}

#[test]
#[ignore]
fn test_type_0_increasing_r0_at_c0_zero_does_not_increment_c4() {
    // ACCC 13.7.2: raising R0 from 1 at C0=0 avoids the C4 increment.
    todo!()
}
