use crate::types::gregtech_machine::GregtechMachine;
use crate::types::gregtech_recipe::GregtechRecipe;
use crate::types::ingredients::item::Item;
use serde::{Deserialize, Serialize};
use strsim::jaro_winkler;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(default)]
    pub recipes: Vec<serde_json::Value>,
    #[serde(default)]
    pub machines: Vec<GregtechMachine>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonFormat {
    pub sources: Vec<Source>,
}

impl JsonFormat {
    pub fn get_recipe_count(&self) -> usize {
        let mut count = 0;
        for source in &self.sources {
            count += source.recipes.len();
            for machine in &source.machines {
                count += machine.recipes.len();
            }
        }
        count
    }

    pub fn search(&self, keyword: &String) -> Vec<(String, GregtechRecipe)> {
        let mut results = Vec::<(String, GregtechRecipe)>::new();

        const SEARCH_THRESHOLD: f64 = 0.7;
        
        for source in &self.sources {
            for machine in &source.machines {
                for recipe in &machine.recipes {
                    let mut found = false;
                    for item in &recipe.item_inputs {
                        if let Some(ref unlocalized_name) = item.unlocalized_name {
                            if jaro_winkler(unlocalized_name, &keyword) > SEARCH_THRESHOLD {
                                found = true;
                                continue;
                            }
                        }
                        if let Some(ref localized_name) = item.localized_name {
                            if jaro_winkler(localized_name, &keyword) > SEARCH_THRESHOLD {
                                found = true;
                                continue;
                            }
                        }
                    }
                    for item in &recipe.item_outputs {
                        if let Some(ref unlocalized_name) = item.unlocalized_name {
                            if jaro_winkler(unlocalized_name, &keyword) > SEARCH_THRESHOLD {
                                found = true;
                                continue;
                            }
                        }
                        if let Some(ref localized_name) = item.localized_name {
                            if jaro_winkler(localized_name, &keyword) > SEARCH_THRESHOLD {
                                found = true;
                                continue;
                            }
                        }
                    }
                    for fluid in &recipe.fluid_inputs {
                        if let Some(ref unlocalized_name) = fluid.unlocalized_name {
                            if jaro_winkler(unlocalized_name, &keyword) > SEARCH_THRESHOLD {
                                found = true;
                                continue;
                            }
                        }
                        if let Some(ref localized_name) = fluid.localized_name {
                            if jaro_winkler(localized_name, &keyword) > SEARCH_THRESHOLD {
                                found = true;
                                continue;
                            }
                        }
                    }
                    for fluid in &recipe.fluid_outputs {
                        if let Some(ref unlocalized_name) = fluid.unlocalized_name {
                            if jaro_winkler(unlocalized_name, &keyword) > SEARCH_THRESHOLD {
                                found = true;
                                continue;
                            }
                        }
                        if let Some(ref localized_name) = fluid.localized_name {
                            if jaro_winkler(localized_name, &keyword) > SEARCH_THRESHOLD {
                                found = true;
                                continue;
                            }
                        }
                    }
                    if found {
                        //dbg!(recipe);
                        results.push((machine.name.clone(), recipe.clone()))
                    };
                }
            }
        }

        results
    }
}
