use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct KeyDefinition {
    pub line: usize,
    pub bit: u8,
}

pub const KEYS: [(&str, (bool, KeyDefinition)); 74] = [
    ("Escape", (false, KeyDefinition { line: 8, bit: 2 })),
    ("Key1", (true, KeyDefinition { line: 8, bit: 0 })),
    ("Key2", (true, KeyDefinition { line: 8, bit: 1 })),
    ("Key3", (true, KeyDefinition { line: 7, bit: 1 })),
    ("Key4", (true, KeyDefinition { line: 7, bit: 0 })),
    ("Key5", (true, KeyDefinition { line: 6, bit: 1 })),
    ("Key6", (true, KeyDefinition { line: 6, bit: 0 })),
    ("Key7", (true, KeyDefinition { line: 5, bit: 1 })),
    ("Key8", (true, KeyDefinition { line: 5, bit: 0 })),
    ("Key9", (true, KeyDefinition { line: 4, bit: 1 })),
    ("Key0", (true, KeyDefinition { line: 4, bit: 0 })),
    ("Minus", (true, KeyDefinition { line: 3, bit: 1 })),
    ("Caret", (true, KeyDefinition { line: 3, bit: 0 })),
    ("Clear", (false, KeyDefinition { line: 2, bit: 0 })),
    ("Delete", (false, KeyDefinition { line: 9, bit: 7 })),
    ("Tab", (false, KeyDefinition { line: 8, bit: 4 })),
    ("Q", (true, KeyDefinition { line: 8, bit: 3 })),
    ("W", (true, KeyDefinition { line: 7, bit: 3 })),
    ("E", (true, KeyDefinition { line: 7, bit: 2 })),
    ("R", (true, KeyDefinition { line: 6, bit: 2 })),
    ("T", (true, KeyDefinition { line: 6, bit: 3 })),
    ("Y", (true, KeyDefinition { line: 5, bit: 3 })),
    ("U", (true, KeyDefinition { line: 5, bit: 2 })),
    ("I", (true, KeyDefinition { line: 4, bit: 3 })),
    ("O", (true, KeyDefinition { line: 4, bit: 2 })),
    ("P", (true, KeyDefinition { line: 3, bit: 3 })),
    ("At", (true, KeyDefinition { line: 3, bit: 2 })),
    ("BracketLeft", (false, KeyDefinition { line: 2, bit: 1 })),
    ("Enter", (false, KeyDefinition { line: 2, bit: 2 })),
    ("CapsLock", (false, KeyDefinition { line: 8, bit: 6 })),
    ("A", (true, KeyDefinition { line: 8, bit: 5 })),
    ("S", (true, KeyDefinition { line: 7, bit: 4 })),
    ("D", (true, KeyDefinition { line: 7, bit: 5 })),
    ("F", (true, KeyDefinition { line: 6, bit: 5 })),
    ("G", (true, KeyDefinition { line: 6, bit: 4 })),
    ("H", (true, KeyDefinition { line: 5, bit: 4 })),
    ("J", (true, KeyDefinition { line: 5, bit: 5 })),
    ("K", (true, KeyDefinition { line: 4, bit: 5 })),
    ("L", (true, KeyDefinition { line: 4, bit: 4 })),
    ("Colon", (true, KeyDefinition { line: 3, bit: 5 })),
    ("Semicolon", (true, KeyDefinition { line: 3, bit: 4 })),
    ("BracketRight", (false, KeyDefinition { line: 2, bit: 3 })),
    ("ShiftLeft", (false, KeyDefinition { line: 2, bit: 5 })),
    ("Z", (true, KeyDefinition { line: 8, bit: 7 })),
    ("X", (true, KeyDefinition { line: 7, bit: 7 })),
    ("C", (true, KeyDefinition { line: 7, bit: 6 })),
    ("V", (true, KeyDefinition { line: 6, bit: 7 })),
    ("B", (true, KeyDefinition { line: 6, bit: 6 })),
    ("N", (true, KeyDefinition { line: 5, bit: 6 })),
    ("M", (true, KeyDefinition { line: 4, bit: 6 })),
    ("Comma", (true, KeyDefinition { line: 4, bit: 7 })),
    ("Period", (true, KeyDefinition { line: 3, bit: 7 })),
    ("Slash", (true, KeyDefinition { line: 3, bit: 6 })),
    ("Backslash", (false, KeyDefinition { line: 2, bit: 6 })),
    ("ShiftRight", (false, KeyDefinition { line: 2, bit: 5 })),
    ("Space", (false, KeyDefinition { line: 5, bit: 7 })),
    ("Control", (false, KeyDefinition { line: 2, bit: 7 })),
    ("ArrowUp", (false, KeyDefinition { line: 0, bit: 0 })),
    ("ArrowLeft", (false, KeyDefinition { line: 1, bit: 0 })),
    ("Copy", (false, KeyDefinition { line: 1, bit: 1 })),
    ("ArrowRight", (false, KeyDefinition { line: 0, bit: 1 })),
    ("ArrowDown", (false, KeyDefinition { line: 0, bit: 2 })),
    ("Numpad7", (false, KeyDefinition { line: 1, bit: 2 })),
    ("Numpad8", (false, KeyDefinition { line: 1, bit: 3 })),
    ("Numpad9", (false, KeyDefinition { line: 0, bit: 3 })),
    ("Numpad4", (false, KeyDefinition { line: 2, bit: 4 })),
    ("Numpad5", (false, KeyDefinition { line: 1, bit: 4 })),
    ("Numpad6", (false, KeyDefinition { line: 0, bit: 4 })),
    ("Numpad1", (false, KeyDefinition { line: 1, bit: 5 })),
    ("Numpad2", (false, KeyDefinition { line: 1, bit: 6 })),
    ("Numpad3", (false, KeyDefinition { line: 0, bit: 5 })),
    ("Numpad0", (false, KeyDefinition { line: 1, bit: 7 })),
    ("NumpadPeriod", (false, KeyDefinition { line: 0, bit: 7 })),
    ("NumpadEnter", (false, KeyDefinition { line: 0, bit: 6 })),
    // ("ToggleJoystick", (false, KeyDefinition { line: 10, bit: 0 })), // TODO: handle specially
    // ("JoystickUp", (false, KeyDefinition { line: 9, bit: 0 })),
    // ("JoystickLeft", (false, KeyDefinition { line: 9, bit: 2 })),
    // ("JoystickRight", (false, KeyDefinition { line: 9, bit: 3 })),
    // ("JoystickDown", (false, KeyDefinition { line: 9, bit: 1 })),
    // ("JoystickFire1", (false, KeyDefinition { line: 9, bit: 4 })),
    // ("JoystickFire2", (false, KeyDefinition { line: 9, bit: 5 })),
    // ("JoystickFire3", (false, KeyDefinition { line: 9, bit: 6 })),
];

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HostKey {
    pub scancode: u32,
    pub modifiers: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyConfig {
    pub normal: HostKey,
    pub shifted: Option<HostKey>,
}

pub struct Keyboard {
    lines: [u8; 10],
    active_line: usize,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            lines: [0xff; 10],
            active_line: 0,
        }
    }

    pub fn reset_all(&mut self) {
        self.lines = [0xff; 10];
    }

    pub fn set_key(&mut self, line: usize, bit: u8) {
        self.lines[line] &= !(1 << bit);
    }

    pub fn unset_key(&mut self, line: usize, bit: u8) {
        self.lines[line] |= 1 << bit;
    }

    pub fn set_active_line(&mut self, line: usize) {
        self.active_line = line;
    }

    pub fn scan_active_line(&self) -> u8 {
        if self.active_line < 10 {
            self.lines[self.active_line]
        } else {
            0xff
        }
    }
}
