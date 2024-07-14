use crate::types::gregtech_recipe::GregtechRecipe;
use crate::types::ingredients::fluid::Fluid;
use crate::types::ingredients::item::Item;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub struct RecipeStats {
    pub total_input_items: Vec<Item>,
    pub total_input_fluids: Vec<Fluid>,
    pub total_output_items: Vec<Item>,
    pub total_output_fluids: Vec<Fluid>,
}

impl RecipeStats {
    pub fn new(input: Vec<GregtechRecipe>) -> Self {
        let mut iter = input.iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        RecipeStats {
            total_input_items: Self::combine_items(&input),
            total_input_fluids: Self::combine_fluids(&first.fluid_inputs, &second.fluid_inputs),
            total_output_items: Self::combine_items(&input),
            total_output_fluids: Self::combine_fluids(&first.fluid_outputs, &second.fluid_outputs),
        }
    }

    fn combine_items(recipes: &Vec<GregtechRecipe>) -> Vec<Item> {
        let mut item_map: HashMap<String, Item> = HashMap::new();

        for recipe in recipes {
            for item in &recipe.item_inputs {
                let name = item.get_name();
                item_map
                    .entry(name)
                    .or_insert(Item::new(&item.unlocalized_name, &item.localized_name))
                    .amount += item.amount;
            }
        }
        item_map.values().cloned().collect()
    }

    fn combine_fluids(fluids1: &Vec<Fluid>, fluids2: &Vec<Fluid>) -> Vec<Fluid> {
        let mut fluid_map: HashMap<String, usize> = HashMap::new();

        for fluid in fluids1.iter().chain(fluids2.iter()) {
            let name = fluid
                .unlocalized_name
                .clone()
                .unwrap_or_else(|| fluid.localized_name.clone().unwrap());
            *fluid_map.entry(name).or_insert(0) += fluid.amount;
        }

        fluid_map
            .into_iter()
            .map(|(name, amount)| Fluid {
                amount,
                unlocalized_name: Some(name.clone()),
                localized_name: None,
            })
            .collect()
    }
}

impl Display for RecipeStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let item_inputs = self
            .total_input_items
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(" + ");

        let fluid_inputs = self
            .total_input_fluids
            .iter()
            .map(|fluid| format!("{}", fluid))
            .collect::<Vec<String>>()
            .join(" + ");

        let item_outputs = self
            .total_output_items
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(" + ");

        let fluid_outputs = self
            .total_output_fluids
            .iter()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_stats() {
        let input_items = [
            Item {
                amount: 10,
                unlocalized_name: Some("item1".to_string()),
                localized_name: Some("Item 1".to_string()),
            },
            Item {
                amount: 12,
                unlocalized_name: Some("item2".to_string()),
                localized_name: Some("Item 2".to_string()),
            },
        ];
        let output_items = [Item {
            amount: 3,
            unlocalized_name: Some("item3".to_string()),
            localized_name: Some("Item 3".to_string()),
        }];
        let first_recipe = GregtechRecipe {
            enabled: false,
            duration: 0,
            eut: 0,
            item_inputs: Vec::<Item>::from(input_items),
            item_outputs: Vec::<Item>::from(output_items),
            fluid_inputs: vec![],
            fluid_outputs: vec![],
        };
        let input_items_2 = [
            Item {
                amount: 2,
                unlocalized_name: Some("item1".to_string()),
                localized_name: Some("Item 1".to_string()),
            },
            Item {
                amount: 1,
                unlocalized_name: Some("item4".to_string()),
                localized_name: Some("Item 4".to_string()),
            },
            Item {
                amount: 1,
                unlocalized_name: Some("item3".to_string()),
                localized_name: Some("Item 3".to_string()),
            },
        ];
        let output_items_2 = [
            Item {
                amount: 3,
                unlocalized_name: Some("item3".to_string()),
                localized_name: Some("Item 3".to_string()),
            },
            Item {
                amount: 4,
                unlocalized_name: Some("item5".to_string()),
                localized_name: Some("Item 5".to_string()),
            },
        ];
        let second_recipe = GregtechRecipe {
            enabled: false,
            duration: 0,
            eut: 0,
            item_inputs: Vec::<Item>::from(input_items_2),
            item_outputs: Vec::<Item>::from(output_items_2),
            fluid_inputs: vec![],
            fluid_outputs: vec![],
        };

        let mut both_recipes = Vec::<GregtechRecipe>::new();
        both_recipes.push(first_recipe);
        both_recipes.push(second_recipe);
        let stats = RecipeStats::new(both_recipes);


        let total_input_items = [
            Item {
                amount: 12,
                unlocalized_name: Some("item1".to_string()),
                localized_name: Some("Item 1".to_string()),
            },
            Item {
                amount: 12,
                unlocalized_name: Some("item2".to_string()),
                localized_name: Some("Item 2".to_string()),
            },
            Item {
                amount: 1,
                unlocalized_name: Some("item4".to_string()),
                localized_name: Some("Item 4".to_string()),
            },
            Item {
                amount: 1,
                unlocalized_name: Some("item3".to_string()),
                localized_name: Some("Item 3".to_string()),
            },
        ];

        assert_eq!(stats.total_input_items, total_input_items);
    }
}
