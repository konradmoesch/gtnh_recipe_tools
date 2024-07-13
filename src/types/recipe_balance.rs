use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::types::gregtech_recipe::GregtechRecipe;
use crate::types::ingredients::item::{Fluid, Item};

pub struct RecipeBalance {
    pub input_items: Vec<Item>,
    pub input_fluids: Vec<Fluid>,
    pub output_items: Vec<Item>,
    pub output_fluids: Vec<Fluid>,
}

impl RecipeBalance {
    pub fn new(re1: &GregtechRecipe, re2: &GregtechRecipe) -> Self {
        let combined_input_items = Self::combine_items(&re1.item_inputs, &re2.item_inputs);
        let combined_input_fluids = Self::combine_fluids(&re1.fluid_inputs, &re2.fluid_inputs);
        let combined_output_items = Self::combine_items(&re1.item_outputs, &re2.item_outputs);
        let combined_output_fluids = Self::combine_fluids(&re1.fluid_outputs, &re2.fluid_outputs);

        let intermediate_items = Self::find_intermediate_items(&re1.item_outputs, &re2.item_inputs);
        let intermediate_fluids = Self::find_intermediate_fluids(&re1.fluid_outputs, &re2.fluid_inputs);

        let final_input_items = Self::remove_intermediate_items(combined_input_items, &intermediate_items);
        let final_input_fluids = Self::remove_intermediate_fluids(combined_input_fluids, &intermediate_fluids);
        let final_output_items = Self::remove_intermediate_items(combined_output_items, &intermediate_items);
        let final_output_fluids = Self::remove_intermediate_fluids(combined_output_fluids, &intermediate_fluids);

        RecipeBalance {
            input_items: final_input_items,
            input_fluids: final_input_fluids,
            output_items: final_output_items,
            output_fluids: final_output_fluids,
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
    fn find_intermediate_items(outputs: &Vec<Item>, inputs: &Vec<Item>) -> HashMap<String, usize> {
        let mut intermediate_map: HashMap<String, usize> = HashMap::new();

        for output in outputs {
            let output_name = output.unlocalized_name.clone().unwrap_or_else(|| output.localized_name.clone().unwrap());
            for input in inputs {
                let input_name = input.unlocalized_name.clone().unwrap_or_else(|| input.localized_name.clone().unwrap());
                if output_name.clone() == input_name {
                    *intermediate_map.entry(output_name.clone()).or_insert(0) += std::cmp::min(output.amount, input.amount);
                }
            }
        }

        intermediate_map
    }

    fn find_intermediate_fluids(outputs: &Vec<Fluid>, inputs: &Vec<Fluid>) -> HashMap<String, usize> {
        let mut intermediate_map: HashMap<String, usize> = HashMap::new();

        for output in outputs {
            let output_name = output.unlocalized_name.clone().unwrap_or_else(|| output.localized_name.clone().unwrap());
            for input in inputs {
                let input_name = input.unlocalized_name.clone().unwrap_or_else(|| input.localized_name.clone().unwrap());
                if output_name.clone() == input_name {
                    *intermediate_map.entry(output_name.clone()).or_insert(0) += std::cmp::min(output.amount, input.amount);
                }
            }
        }

        intermediate_map
    }

    fn remove_intermediate_items(items: Vec<Item>, intermediates: &HashMap<String, usize>) -> Vec<Item> {
        items.into_iter().filter_map(|item| {
            let name = item.unlocalized_name.clone().unwrap_or_else(|| item.localized_name.clone().unwrap());
            if let Some(&intermediate_amount) = intermediates.get(&name) {
                if item.amount > intermediate_amount {
                    Some(Item {
                        amount: item.amount - intermediate_amount,
                        unlocalized_name: item.unlocalized_name,
                        localized_name: item.localized_name,
                    })
                } else {
                    None
                }
            } else {
                Some(item)
            }
        }).collect()
    }

    fn remove_intermediate_fluids(fluids: Vec<Fluid>, intermediates: &HashMap<String, usize>) -> Vec<Fluid> {
        fluids.into_iter().filter_map(|fluid| {
            let name = fluid.unlocalized_name.clone().unwrap_or_else(|| fluid.localized_name.clone().unwrap());
            if let Some(&intermediate_amount) = intermediates.get(&name) {
                if fluid.amount > intermediate_amount {
                    Some(Fluid {
                        amount: fluid.amount - intermediate_amount,
                        unlocalized_name: fluid.unlocalized_name,
                        localized_name: fluid.localized_name,
                    })
                } else {
                    None
                }
            } else {
                Some(fluid)
            }
        }).collect()
    }
}

impl Display for RecipeBalance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let item_inputs = self.input_items.iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(" + ");

        let fluid_inputs = self.input_fluids.iter()
            .map(|fluid| format!("{}", fluid))
            .collect::<Vec<String>>()
            .join(" + ");

        let item_outputs = self.output_items.iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(" + ");

        let fluid_outputs = self.output_fluids.iter()
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