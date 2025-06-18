use serde::{Deserialize, Serialize};

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
