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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_clock() {
        let mut clock = MasterClock::default();
        assert_eq!(clock.current().value(), 0);
        clock.step(1);
        assert_eq!(clock.current().value(), 1);
        clock.step(5);
        assert_eq!(clock.current().value(), 6);
    }

    #[test]
    fn test_master_clock_tick() {
        let tick1 = MasterClockTick(100);
        let tick2 = MasterClockTick(200);

        assert!(tick1 < tick2);
        assert_eq!(tick1.value(), 100);
        assert_eq!(tick2.value(), 200);
    }
}
