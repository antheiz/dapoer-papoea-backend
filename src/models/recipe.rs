use serde::{Deserialize, Serialize};
use super::ingredient::Ingredient;
use super::instruction::Instruction;

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub title: String,
    pub slug: String,
    pub category: String,
    pub description: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<Instruction>,
}