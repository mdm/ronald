use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MasterClockTick(u64);

impl MasterClockTick {
    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MasterClock {
    current: MasterClockTick,
}

impl MasterClock {
    pub fn current(&self) -> MasterClockTick {
        self.current
    }

    pub fn step(&mut self, cycles: u64) {
        self.current.0 += cycles;
    }
}
