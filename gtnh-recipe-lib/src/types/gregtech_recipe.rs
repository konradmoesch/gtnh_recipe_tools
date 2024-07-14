use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::types::ingredients::item::Item;
use crate::types::ingredients::fluid::Fluid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GregtechRecipe {
    #[serde(rename = "en")]
    pub enabled: bool,
    #[serde(rename = "dur")]
    pub duration: usize,
    pub eut: usize,
    #[serde(rename = "iI")]
    pub item_inputs: Vec<Item>,
    #[serde(rename = "iO")]
    pub item_outputs: Vec<Item>,
    #[serde(rename = "fI")]
    pub fluid_inputs: Vec<Fluid>,
    #[serde(rename = "fO")]
    pub fluid_outputs: Vec<Fluid>
}

impl Display for GregtechRecipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let item_inputs = self.item_inputs.iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(" + ");

        let fluid_inputs = self.fluid_inputs.iter()
            .map(|fluid| format!("{}", fluid))
            .collect::<Vec<String>>()
            .join(" + ");

        let item_outputs = self.item_outputs.iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(" + ");

        let fluid_outputs = self.fluid_outputs.iter()
            .map(|fluid| format!("{}", fluid))
            .collect::<Vec<String>>()
            .join(" + ");

        if !item_inputs.is_empty() && !fluid_inputs.is_empty() {
            write!(f, "{}, {}", item_inputs, fluid_inputs)?;
        } else if !item_inputs.is_empty() {
            write!(f, "{}", item_inputs)?;
        } else {
            write!(f, "{}", fluid_inputs)?;
        }

        write!(f, " -> ")?;

        if !item_outputs.is_empty() && !fluid_outputs.is_empty() {
            write!(f, "{}, {}", item_outputs, fluid_outputs)?;
        } else if !item_outputs.is_empty() {
            write!(f, "{}", item_outputs)?;
        } else {
            write!(f, "{}", fluid_outputs)?;
        }

        Ok(())
    }
}

pub struct Recipes(Vec<GregtechRecipe>);

impl From<Vec<GregtechRecipe>> for Recipes {
    fn from(vec: Vec<GregtechRecipe>) -> Self {
        Recipes(vec)
    }
}

impl Display for Recipes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

pub fn filter_recipes_by_input_item(recipes: &Vec<GregtechRecipe>, item_name: &str) -> Vec<GregtechRecipe> {
    recipes.iter()
        .filter(|recipe| recipe.item_inputs.iter().any(|item| item.localized_name.as_deref() == Some(item_name)))
        .cloned()
        .collect()
}
pub fn filter_recipes_by_output_item(recipes: &Vec<GregtechRecipe>, item_name: &str) -> Vec<GregtechRecipe> {
    recipes.iter()
        .filter(|recipe| recipe.item_outputs.iter().any(|item| item.localized_name.as_deref() == Some(item_name)))
        .cloned()
        .collect()
}
pub fn filter_recipes_by_input_fluid(recipes: &Vec<GregtechRecipe>, fluid_name: &str, amount: Option<usize>) -> Vec<GregtechRecipe> {
    recipes.iter()
        .filter(|recipe| recipe.fluid_inputs.iter().any(|fluid| {
            if amount != None {
                (fluid.localized_name.as_deref() == Some(fluid_name)) && (fluid.amount == amount.unwrap())
            } else {
                fluid.localized_name.as_deref() == Some(fluid_name)
            }
        }))
        .cloned()
        .collect()
}
pub fn filter_recipes_by_output_fluid(recipes: &Vec<GregtechRecipe>, fluid_name: &str, amount: Option<usize>) -> Vec<GregtechRecipe> {
    recipes.iter()
        .filter(|recipe| recipe.fluid_outputs.iter().any(|fluid|{
            if amount != None {
                (fluid.localized_name.as_deref() == Some(fluid_name)) && (fluid.amount == amount.unwrap())
            } else {
                fluid.localized_name.as_deref() == Some(fluid_name)
            }
        }))
        .cloned()
        .collect()
}