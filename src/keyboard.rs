use std::cell::RefCell;
use std::rc::Rc;

use serde::{Serialize, Deserialize};

pub const KEYS: [(&str, bool); 82] = [
    ("Escape", false),
    ("Key1", true),
    ("Key2", true),
    ("Key3", true),
    ("Key4", true),
    ("Key5", true),
    ("Key6", true),
    ("Key7", true),
    ("Key8", true),
    ("Key9", true),
    ("Key0", true),
    ("Minus", true),
    ("Caret", true),
    ("Clear", false),
    ("Delete", false),
    ("Tab", false),
    ("Q", true),
    ("W", true),
    ("E", true),
    ("R", true),
    ("T", true),
    ("Y", true),
    ("U", true),
    ("I", true),
    ("O", true),
    ("P", true),
    ("At", true),
    ("BracketLeft", false),
    ("Enter", false),
    ("CapsLock", false),
    ("A", true),
    ("S", true),
    ("D", true),
    ("F", true),
    ("G", true),
    ("H", true),
    ("J", true),
    ("K", true),
    ("L", true),
    ("Colon", true),
    ("Semicolon", true),
    ("BracketRight", false),
    ("ShiftLeft", false),
    ("Z", true),
    ("X", true),
    ("C", true),
    ("V", true),
    ("B", true),
    ("N", true),
    ("M", true),
    ("Comma", true),
    ("Period", true),
    ("Slash", true),
    ("Backslash", false),
    ("ShiftRight", true),
    ("Space", false),
    ("Control", false),
    ("ArrowUp", false),
    ("ArrowLeft", false),
    ("Copy", false),
    ("ArrowRight", false),
    ("ArrowDown", false),
    ("Numpad7", false),
    ("Numpad8", false),
    ("Numpad9", false),
    ("Numpad4", false),
    ("Numpad5", false),
    ("Numpad6", false),
    ("Numpad1", false),
    ("Numpad2", false),
    ("Numpad3", false),
    ("Numpad0", false),
    ("NumpadPeriod", false),
    ("NumpadEnter", false),
    ("ToggleJoystick", false),
    ("JoystickUp", false),
    ("JoystickLeft", false),
    ("JoystickRight", false),
    ("JoystickDown", false),
    ("JoystickFire1", false),
    ("JoystickFire2", false),
    ("JoystickFire3", false),
];

#[derive(Debug, Serialize, Deserialize)]
pub struct ScancodeAndModifiers {
    pub scancode: u32,
    pub modifiers: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyConfig {
    pub normal: ScancodeAndModifiers,
    pub shifted: Option<ScancodeAndModifiers>,
}

pub type KeyboardShared = Rc<RefCell<Keyboard>>;

pub struct Keyboard {
    lines: [u8; 10],
    active_line: usize,
}

impl Keyboard {
    pub fn new_shared() -> KeyboardShared {
        let keyboard = Keyboard {
            lines: [0xff; 10],
            active_line: 0,
        };

        Rc::new(RefCell::new(keyboard))
    }

    pub fn reset_all(&mut self) {
        self.lines = [0xff; 10];
    }

    pub fn set_key(&mut self, line: usize, bit: u8) {
        self.lines[line] &= !(1 << bit);
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
