use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    pub step: u32,
    pub text: String,
}