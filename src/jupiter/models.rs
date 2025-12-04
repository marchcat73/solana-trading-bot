use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwapMode {
    #[serde(rename = "ExactIn")]
    ExactIn,
    #[serde(rename = "ExactOut")]
    ExactOut,
}

impl Default for SwapMode {
    fn default() -> Self {
        Self::ExactIn
    }
}

impl ToString for SwapMode {
    fn to_string(&self) -> String {
        match self {
            SwapMode::ExactIn => "ExactIn".to_string(),
            SwapMode::ExactOut => "ExactOut".to_string(),
        }
    }
}
