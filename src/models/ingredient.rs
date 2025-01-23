use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: String,
}