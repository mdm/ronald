use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyboard {
    lines: [u8; 10],
    active_line: usize,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            lines: [0xff; 10],
            active_line: 0,
        }
    }
}

impl Keyboard {
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
