use std::fs::File;
use std::io::Read;
use gtnh_recipe_lib::types::gregtech_recipe::{filter_recipes_by_input_fluid, filter_recipes_by_output_fluid, Recipes};
use gtnh_recipe_lib::types::json::JsonFormat;
use gtnh_recipe_lib::types::recipe_balance::RecipeBalance;
use gtnh_recipe_lib::types::recipe_stats::RecipeStats;

fn main() {
    println!("Hello, world!");
    let mut file = File::open("./recipes_2.json").unwrap();
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("unable to read file to string");
    let deserialized: JsonFormat = serde_json::from_str(&file_str).unwrap();
    //dbg!(&deserialized);
    let mut source_iter = deserialized.sources.iter();
    let gregtech_source = source_iter.next().unwrap();
    let gt_machine_count = gregtech_source.machines.len();
    dbg!(gt_machine_count);
    let mut gt_recipe_count = 0;
    for machine in &gregtech_source.machines {
        gt_recipe_count += machine.recipes.len();
        let mut max_item_input_count = 0;
        let mut max_item_output_count = 0;
        let mut max_fluid_input_count = 0;
        let mut max_fluid_output_count = 0;
        for recipe in machine.clone().recipes.clone() {
            if recipe.item_inputs.len() > max_item_input_count {max_item_input_count = recipe.item_inputs.len()};
            if recipe.item_outputs.len() > max_item_output_count {max_item_output_count = recipe.item_outputs.len()};
            if recipe.fluid_inputs.len() > max_fluid_input_count {max_fluid_input_count = recipe.fluid_inputs.len()};
            if recipe.fluid_outputs.len() > max_fluid_output_count {max_fluid_output_count = recipe.fluid_outputs.len()};
        }
        println!("{}: {}, Max items: {} / {}, Max fluids: {} / {}", machine.name, machine.recipes.len(),
        max_item_input_count, max_item_output_count, max_fluid_input_count, max_fluid_output_count);
    }
    dbg!(gt_recipe_count);
    let shaped_recipe_source = source_iter.next().unwrap();
    let shaped_recipe_count = shaped_recipe_source.recipes.len();
    dbg!(shaped_recipe_count);
    let shapeless_recipe_source = source_iter.next().unwrap();
    let shapeless_recipe_count = shapeless_recipe_source.recipes.len();
    dbg!(shapeless_recipe_count);
    let shaped_oredict_recipe_source = source_iter.next().unwrap();
    let shaped_oredict_recipe_count = shaped_oredict_recipe_source.recipes.len();
    dbg!(shaped_oredict_recipe_count);

    let large_chemical_reactor_recipes = &gregtech_source.machines.iter().find(|&machine| machine.name=="Large Chemical Reactor").unwrap().recipes;
    //dbg!(&large_chemical_reactor_recipes);
    let nitric_acid_output_recipes = filter_recipes_by_output_fluid(&large_chemical_reactor_recipes, "Nitric Acid", Some(2000));
    dbg!(nitric_acid_output_recipes.len());
    println!("Found these Recipes on 'Large Chemical Reactor' with output fluid 'Nitric Acid (2000l)': \n{}", Recipes::from(nitric_acid_output_recipes.clone()));
    let no2_to_nitric_acid_recipes = filter_recipes_by_input_fluid(&nitric_acid_output_recipes, "Nitrogen Dioxide", Some(3000));
    println!("Filtered by input fluid 'Nitrogen Dioxide (3000l)': \n{}", Recipes::from(no2_to_nitric_acid_recipes.clone()));

    let no_to_no2_recipe = filter_recipes_by_input_fluid(&filter_recipes_by_output_fluid(&large_chemical_reactor_recipes, "Nitrogen Dioxide", Some(1000)), "Nitric Oxide", Some(1000));
    println!("NO to NO2: \n{}", Recipes::from(no_to_no2_recipe.clone()));

    let balance = RecipeBalance::new(no2_to_nitric_acid_recipes.first().unwrap(), no_to_no2_recipe.first().unwrap());
    println!("Resulting balance: \n{}", balance);

    let stats = RecipeStats::new(no2_to_nitric_acid_recipes.first().unwrap(), no_to_no2_recipe.first().unwrap());
    println!("Total stats: \n{}", stats);
}
