use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TapeController {}

impl TapeController {
    pub fn switch_motor(&mut self, on: bool) {
        // TODO: implement this
    }

    pub fn read_sample(&self) -> bool {
        false // TODO: implement this
    }

    pub fn write_sample(&mut self, high_amplitude: bool) {
        // TODO: implement this
    }
}
