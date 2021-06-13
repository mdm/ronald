use std::cell::RefCell;
use std::rc::Rc;

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