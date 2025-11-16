#[derive(Debug, Clone)]
pub struct KeyDefinition {
    pub line: usize,
    pub bit: u8,
    pub shiftable: bool,
}

pub const KEYS: [(&str, KeyDefinition); 80] = [
    (
        "Escape",
        (KeyDefinition {
            line: 8,
            bit: 2,
            shiftable: false,
        }),
    ),
    (
        "Key1",
        (KeyDefinition {
            line: 8,
            bit: 0,
            shiftable: true,
        }),
    ),
    (
        "Key2",
        (KeyDefinition {
            line: 8,
            bit: 1,
            shiftable: true,
        }),
    ),
    (
        "Key3",
        (KeyDefinition {
            line: 7,
            bit: 1,
            shiftable: true,
        }),
    ),
    (
        "Key4",
        (KeyDefinition {
            line: 7,
            bit: 0,
            shiftable: true,
        }),
    ),
    (
        "Key5",
        (KeyDefinition {
            line: 6,
            bit: 1,
            shiftable: true,
        }),
    ),
    (
        "Key6",
        (KeyDefinition {
            line: 6,
            bit: 0,
            shiftable: true,
        }),
    ),
    (
        "Key7",
        (KeyDefinition {
            line: 5,
            bit: 1,
            shiftable: true,
        }),
    ),
    (
        "Key8",
        (KeyDefinition {
            line: 5,
            bit: 0,
            shiftable: true,
        }),
    ),
    (
        "Key9",
        (KeyDefinition {
            line: 4,
            bit: 1,
            shiftable: true,
        }),
    ),
    (
        "Key0",
        (KeyDefinition {
            line: 4,
            bit: 0,
            shiftable: true,
        }),
    ),
    (
        "Minus",
        (KeyDefinition {
            line: 3,
            bit: 1,
            shiftable: true,
        }),
    ),
    (
        "Caret",
        (KeyDefinition {
            line: 3,
            bit: 0,
            shiftable: true,
        }),
    ),
    (
        "Clear",
        (KeyDefinition {
            line: 2,
            bit: 0,
            shiftable: false,
        }),
    ),
    (
        "Delete",
        (KeyDefinition {
            line: 9,
            bit: 7,
            shiftable: false,
        }),
    ),
    (
        "Tab",
        (KeyDefinition {
            line: 8,
            bit: 4,
            shiftable: false,
        }),
    ),
    (
        "Q",
        (KeyDefinition {
            line: 8,
            bit: 3,
            shiftable: true,
        }),
    ),
    (
        "W",
        (KeyDefinition {
            line: 7,
            bit: 3,
            shiftable: true,
        }),
    ),
    (
        "E",
        (KeyDefinition {
            line: 7,
            bit: 2,
            shiftable: true,
        }),
    ),
    (
        "R",
        (KeyDefinition {
            line: 6,
            bit: 2,
            shiftable: true,
        }),
    ),
    (
        "T",
        (KeyDefinition {
            line: 6,
            bit: 3,
            shiftable: true,
        }),
    ),
    (
        "Y",
        (KeyDefinition {
            line: 5,
            bit: 3,
            shiftable: true,
        }),
    ),
    (
        "U",
        (KeyDefinition {
            line: 5,
            bit: 2,
            shiftable: true,
        }),
    ),
    (
        "I",
        (KeyDefinition {
            line: 4,
            bit: 3,
            shiftable: true,
        }),
    ),
    (
        "O",
        (KeyDefinition {
            line: 4,
            bit: 2,
            shiftable: true,
        }),
    ),
    (
        "P",
        (KeyDefinition {
            line: 3,
            bit: 3,
            shiftable: true,
        }),
    ),
    (
        "At",
        (KeyDefinition {
            line: 3,
            bit: 2,
            shiftable: true,
        }),
    ),
    (
        "BracketLeft",
        (KeyDefinition {
            line: 2,
            bit: 1,
            shiftable: true,
        }),
    ),
    (
        "Enter",
        (KeyDefinition {
            line: 2,
            bit: 2,
            shiftable: false,
        }),
    ),
    (
        "CapsLock",
        (KeyDefinition {
            line: 8,
            bit: 6,
            shiftable: false,
        }),
    ),
    (
        "A",
        (KeyDefinition {
            line: 8,
            bit: 5,
            shiftable: true,
        }),
    ),
    (
        "S",
        (KeyDefinition {
            line: 7,
            bit: 4,
            shiftable: true,
        }),
    ),
    (
        "D",
        (KeyDefinition {
            line: 7,
            bit: 5,
            shiftable: true,
        }),
    ),
    (
        "F",
        (KeyDefinition {
            line: 6,
            bit: 5,
            shiftable: true,
        }),
    ),
    (
        "G",
        (KeyDefinition {
            line: 6,
            bit: 4,
            shiftable: true,
        }),
    ),
    (
        "H",
        (KeyDefinition {
            line: 5,
            bit: 4,
            shiftable: true,
        }),
    ),
    (
        "J",
        (KeyDefinition {
            line: 5,
            bit: 5,
            shiftable: true,
        }),
    ),
    (
        "K",
        (KeyDefinition {
            line: 4,
            bit: 5,
            shiftable: true,
        }),
    ),
    (
        "L",
        (KeyDefinition {
            line: 4,
            bit: 4,
            shiftable: true,
        }),
    ),
    (
        "Colon",
        (KeyDefinition {
            line: 3,
            bit: 5,
            shiftable: true,
        }),
    ),
    (
        "Semicolon",
        (KeyDefinition {
            line: 3,
            bit: 4,
            shiftable: true,
        }),
    ),
    (
        "BracketRight",
        (KeyDefinition {
            line: 2,
            bit: 3,
            shiftable: true,
        }),
    ),
    (
        "Shift",
        (KeyDefinition {
            line: 2,
            bit: 5,
            shiftable: false,
        }),
    ),
    (
        "Z",
        (KeyDefinition {
            line: 8,
            bit: 7,
            shiftable: true,
        }),
    ),
    (
        "X",
        (KeyDefinition {
            line: 7,
            bit: 7,
            shiftable: true,
        }),
    ),
    (
        "C",
        (KeyDefinition {
            line: 7,
            bit: 6,
            shiftable: true,
        }),
    ),
    (
        "V",
        (KeyDefinition {
            line: 6,
            bit: 7,
            shiftable: true,
        }),
    ),
    (
        "B",
        (KeyDefinition {
            line: 6,
            bit: 6,
            shiftable: true,
        }),
    ),
    (
        "N",
        (KeyDefinition {
            line: 5,
            bit: 6,
            shiftable: true,
        }),
    ),
    (
        "M",
        (KeyDefinition {
            line: 4,
            bit: 6,
            shiftable: true,
        }),
    ),
    (
        "Comma",
        (KeyDefinition {
            line: 4,
            bit: 7,
            shiftable: true,
        }),
    ),
    (
        "Period",
        (KeyDefinition {
            line: 3,
            bit: 7,
            shiftable: true,
        }),
    ),
    (
        "Slash",
        (KeyDefinition {
            line: 3,
            bit: 6,
            shiftable: true,
        }),
    ),
    (
        "Backslash",
        (KeyDefinition {
            line: 2,
            bit: 6,
            shiftable: true,
        }),
    ),
    (
        "Space",
        (KeyDefinition {
            line: 5,
            bit: 7,
            shiftable: false,
        }),
    ),
    (
        "Control",
        (KeyDefinition {
            line: 2,
            bit: 7,
            shiftable: false,
        }),
    ),
    (
        "ArrowUp",
        (KeyDefinition {
            line: 0,
            bit: 0,
            shiftable: false,
        }),
    ),
    (
        "ArrowLeft",
        (KeyDefinition {
            line: 1,
            bit: 0,
            shiftable: false,
        }),
    ),
    (
        "Copy",
        (KeyDefinition {
            line: 1,
            bit: 1,
            shiftable: false,
        }),
    ),
    (
        "ArrowRight",
        (KeyDefinition {
            line: 0,
            bit: 1,
            shiftable: false,
        }),
    ),
    (
        "ArrowDown",
        (KeyDefinition {
            line: 0,
            bit: 2,
            shiftable: false,
        }),
    ),
    (
        "Numpad7",
        (KeyDefinition {
            line: 1,
            bit: 2,
            shiftable: false,
        }),
    ),
    (
        "Numpad8",
        (KeyDefinition {
            line: 1,
            bit: 3,
            shiftable: false,
        }),
    ),
    (
        "Numpad9",
        (KeyDefinition {
            line: 0,
            bit: 3,
            shiftable: false,
        }),
    ),
    (
        "Numpad4",
        (KeyDefinition {
            line: 2,
            bit: 4,
            shiftable: false,
        }),
    ),
    (
        "Numpad5",
        (KeyDefinition {
            line: 1,
            bit: 4,
            shiftable: false,
        }),
    ),
    (
        "Numpad6",
        (KeyDefinition {
            line: 0,
            bit: 4,
            shiftable: false,
        }),
    ),
    (
        "Numpad1",
        (KeyDefinition {
            line: 1,
            bit: 5,
            shiftable: false,
        }),
    ),
    (
        "Numpad2",
        (KeyDefinition {
            line: 1,
            bit: 6,
            shiftable: false,
        }),
    ),
    (
        "Numpad3",
        (KeyDefinition {
            line: 0,
            bit: 5,
            shiftable: false,
        }),
    ),
    (
        "Numpad0",
        (KeyDefinition {
            line: 1,
            bit: 7,
            shiftable: false,
        }),
    ),
    (
        "NumpadPeriod",
        (KeyDefinition {
            line: 0,
            bit: 7,
            shiftable: false,
        }),
    ),
    (
        "NumpadEnter",
        (KeyDefinition {
            line: 0,
            bit: 6,
            shiftable: false,
        }),
    ),
    (
        "JoystickUp",
        (KeyDefinition {
            line: 9,
            bit: 0,
            shiftable: false,
        }),
    ),
    (
        "JoystickLeft",
        (KeyDefinition {
            line: 9,
            bit: 2,
            shiftable: false,
        }),
    ),
    (
        "JoystickRight",
        (KeyDefinition {
            line: 9,
            bit: 3,
            shiftable: false,
        }),
    ),
    (
        "JoystickDown",
        (KeyDefinition {
            line: 9,
            bit: 1,
            shiftable: false,
        }),
    ),
    (
        "JoystickFire1",
        (KeyDefinition {
            line: 9,
            bit: 4,
            shiftable: false,
        }),
    ),
    (
        "JoystickFire2",
        (KeyDefinition {
            line: 9,
            bit: 5,
            shiftable: false,
        }),
    ),
    (
        "JoystickFire3",
        (KeyDefinition {
            line: 9,
            bit: 6,
            shiftable: false,
        }),
    ),
];

pub const SCREEN_BUFFER_WIDTH: usize = 48 * 16;
pub const SCREEN_BUFFER_HEIGHT: usize = 35 * 16;

// CPC hardware palette colors (RGBA format)
#[allow(clippy::identity_op, clippy::eq_op)]
pub const FIRMWARE_COLORS: [[u8; 4]; 27] = [
    [0x00, 0x00, 0x00, 0xff], // 0
    [0x00, 0x00, 0x80, 0xff], // 1
    [0x00, 0x00, 0xff, 0xff], // 2
    [0x80, 0x00, 0x00, 0xff], // 3
    [0x80, 0x00, 0x80, 0xff], // 4
    [0x80, 0x00, 0xff, 0xff], // 5
    [0xff, 0x00, 0x00, 0xff], // 6
    [0xff, 0x00, 0x80, 0xff], // 7
    [0xff, 0x00, 0xff, 0xff], // 8
    [0x00, 0x80, 0x00, 0xff], // 9
    [0x00, 0x80, 0x80, 0xff], // 10
    [0x00, 0x80, 0xff, 0xff], // 11
    [0x80, 0x80, 0x00, 0xff], // 12
    [0x80, 0x80, 0x80, 0xff], // 13
    [0x80, 0x80, 0xff, 0xff], // 14
    [0xff, 0x80, 0x00, 0xff], // 15
    [0xff, 0x80, 0x80, 0xff], // 16
    [0xff, 0x80, 0xff, 0xff], // 17
    [0x00, 0xff, 0x00, 0xff], // 18
    [0x00, 0xff, 0x80, 0xff], // 19
    [0x00, 0xff, 0xff, 0xff], // 20
    [0x80, 0xff, 0x00, 0xff], // 21
    [0x80, 0xff, 0x80, 0xff], // 22
    [0x80, 0xff, 0xff, 0xff], // 23
    [0xff, 0xff, 0x00, 0xff], // 24
    [0xff, 0xff, 0x80, 0xff], // 25
    [0xff, 0xff, 0xff, 0xff], // 26
];

// Hardware to firmware color mapping
pub const HARDWARE_TO_FIRMWARE_COLORS: [usize; 32] = [
    13, // 0 (0x40)
    13, // 1 (0x41)
    19, // 2 (0x42)
    25, // 3 (0x43)
    1,  // 4 (0x44)
    7,  // 5 (0x45)
    10, // 6 (0x46)
    16, // 7 (0x47)
    7,  // 8 (0x48)
    25, // 9 (0x49)
    24, // 10 (0x4a)
    26, // 11 (0x4b)
    6,  // 12 (0x4c)
    8,  // 13 (0x4d)
    15, // 14 (0x4e)
    17, // 15 (0x4f)
    1,  // 16 (0x50)
    19, // 17 (0x51)
    18, // 18 (0x52)
    20, // 19 (0x53)
    0,  // 20 (0x54)
    2,  // 21 (0x55)
    9,  // 22 (0x56)
    11, // 23 (0x57)
    4,  // 24 (0x58)
    22, // 25 (0x59)
    21, // 26 (0x5a)
    23, // 27 (0x5b)
    3,  // 28 (0x5c)
    5,  // 29 (0x5d)
    12, // 30 (0x5e)
    14, // 31 (0x5f)
];
