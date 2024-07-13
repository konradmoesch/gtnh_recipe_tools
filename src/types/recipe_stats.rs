use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::types::gregtech_recipe::GregtechRecipe;
use crate::types::ingredients::item::{Fluid, Item};

pub struct RecipeStats {
    pub total_input_items: Vec<Item>,
    pub total_input_fluids: Vec<Fluid>,
    pub total_output_items: Vec<Item>,
    pub total_output_fluids: Vec<Fluid>,
}

impl RecipeStats {
    pub fn new(re1: &GregtechRecipe, re2: &GregtechRecipe) -> Self {
        RecipeStats {
            total_input_items: Self::combine_items(&re1.item_inputs, &re2.item_inputs),
            total_input_fluids: Self::combine_fluids(&re1.fluid_inputs, &re2.fluid_inputs),
            total_output_items: Self::combine_items(&re1.item_outputs, &re2.item_outputs),
            total_output_fluids: Self::combine_fluids(&re1.fluid_outputs, &re2.fluid_outputs),
        }
    }

    fn combine_items(items1: &Vec<Item>, items2: &Vec<Item>) -> Vec<Item> {
        let mut item_map: HashMap<String, usize> = HashMap::new();

        for item in items1.iter().chain(items2.iter()) {
            let name = item.unlocalized_name.clone().unwrap_or_else(|| item.localized_name.clone().unwrap());
            *item_map.entry(name).or_insert(0) += item.amount;
        }

        item_map.into_iter().map(|(name, amount)| {
            Item {
                amount,
                unlocalized_name: Some(name.clone()),
                localized_name: None,
            }
        }).collect()
    }

    fn combine_fluids(fluids1: &Vec<Fluid>, fluids2: &Vec<Fluid>) -> Vec<Fluid> {
        let mut fluid_map: HashMap<String, usize> = HashMap::new();

        for fluid in fluids1.iter().chain(fluids2.iter()) {
            let name = fluid.unlocalized_name.clone().unwrap_or_else(|| fluid.localized_name.clone().unwrap());
            *fluid_map.entry(name).or_insert(0) += fluid.amount;
        }

        fluid_map.into_iter().map(|(name, amount)| {
            Fluid {
                amount,
                unlocalized_name: Some(name.clone()),
                localized_name: None,
            }
        }).collect()
    }
}

impl Display for RecipeStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let item_inputs = self.total_input_items.iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(" + ");

        let fluid_inputs = self.total_input_fluids.iter()
            .map(|fluid| format!("{}", fluid))
            .collect::<Vec<String>>()
            .join(" + ");

        let item_outputs = self.total_output_items.iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(" + ");

        let fluid_outputs = self.total_output_fluids.iter()
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