use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedConfig {
    pub name: String,
    pub value: i32,
}

impl SharedConfig {
    pub fn new(name: impl Into<String>, value: i32) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

pub fn get_shared_value() -> i32 {
    42
}
