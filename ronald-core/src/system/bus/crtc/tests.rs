//! Test suite derived from "The Amstrad CPC CRTC Compendium" (ACCC) v1.11 by Longshot / Logon System.
//! Section numbers in the comments refer to the compendium. One `step` is one CRTC character (1 µs).
//!
//! Test names start with `test_type_<types>_`, listing the CRTC types the behaviour applies to:
//!   type 0: Hitachi HD6845S / UMC UM6845
//!   type 1: UMC UM6845R
//!   type 2: Motorola MC6845
//!   type 4: Amstrad pre-ASIC 40226
//! `all` means types 0, 1, 2 and 4. Type 3 (CPC+ ASIC) is out of scope.

use super::*;

macro_rules! crtcs {
        (All) => {
            [
                AnyCrtController::new(CrtcType::Type0),
                AnyCrtController::new(CrtcType::Type1),
                AnyCrtController::new(CrtcType::Type2),
                AnyCrtController::new(CrtcType::Type4),
            ]
        };
        ($($variant:ident),+) => {
            [$(
                AnyCrtController::new(CrtcType::$variant)
            ),+]
        };
    }

macro_rules! is_crtc_type {
    ($any_crtc:expr, $variant:ident) => {
        matches!($any_crtc.inner, AnyCrtControllerInner::$variant(_))
    };
}

// (register, value)
const CPC_DEFAULTS: &[(u8, u8)] = &[
    (0, 63),
    (1, 40),
    (2, 46),
    (3, 128 + 14),
    (4, 38),
    (6, 25),
    (7, 30),
    (9, 7),
    (12, 48),
];

mod display_enable;
mod frame_geometry;
mod horizontal_sync;
mod horizontal_total;
mod interlace;
mod last_line;
mod raster_counter;
mod register_access;
mod rupture_for_dummies;
mod skew;
mod vertical_adjustment;
mod vertical_sync;
mod video_pointer;
