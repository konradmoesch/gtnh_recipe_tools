use serde::{Deserialize, Serialize};
use crate::types::gregtech_recipe::GregtechRecipe;


#[derive(Serialize, Deserialize, Debug)]
pub struct GregtechMachine {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "recs")]
    pub recipes: Vec<GregtechRecipe>
}