//! --- Day 21: Allergen Assessment ---
//
// You reach the train's last stop and the closest you can get to your vacation
// island without getting wet. There aren't even any boats here, but nothing
// can stop you now: you build a raft. You just need a few days' worth of food
// for your journey.
//
// You don't speak the local language, so you can't read any ingredients lists.
// However, sometimes, allergens are listed in a language you do understand.
// You should be able to use this information to determine which ingredient
// contains which allergen and work out which foods are safe to take with you
// on your trip.
//
// You start by compiling a list of foods (your puzzle input), one food per
// line. Each line includes that food's ingredients list followed by some or
// all of the allergens the food contains.
//
// Each allergen is found in exactly one ingredient. Each ingredient contains
// zero or one allergen. Allergens aren't always marked; when they're listed
// (as in (contains nuts, shellfish) after an ingredients list), the ingredient
// that contains each listed allergen will be somewhere in the corresponding
// ingredients list. However, even if an allergen isn't listed, the ingredient
// that contains that allergen could still be present: maybe they forgot to
// label it, or maybe it was labeled in a language you don't know.
//
// For example, consider the following list of foods:
//
// mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
// trh fvjkl sbzzf mxmxvkd (contains dairy)
// sqjhc fvjkl (contains soy)
// sqjhc mxmxvkd sbzzf (contains fish)
//
// The first food in the list has four ingredients (written in a language you
// don't understand): mxmxvkd, kfcds, sqjhc, and nhms. While the food might
// contain other allergens, a few allergens the food definitely contains are
// listed afterward: dairy and fish.
//
// The first step is to determine which ingredients can't possibly contain any
// of the allergens in any food in your list. In the above example, none of the
// ingredients kfcds, nhms, sbzzf, or trh can contain an allergen. Counting the
// number of times any of these ingredients appear in any ingredients list
// produces 5: they all appear once each except sbzzf, which appears twice.
//
// Determine which ingredients cannot possibly contain any of the allergens in
// your list. How many times do any of those ingredients appear?

use std::fs;
use std::collections::{HashMap, HashSet};
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
// use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "ingredients.pest"]
pub struct IngredientsParser;

fn main() {
    println!("Advent of Code 2020 Day 21!");

    // let filename = "inputexample.txt";
    let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let mut foods: Vec<(Vec<String>, Vec<String>)> = Vec::new();

    let file = IngredientsParser::parse(Rule::file, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap().into_inner().next().unwrap();

    for food in file.into_inner() {
        let mut pair = food.into_inner();
        let ingredients: Vec<String>  = pair.next().unwrap().into_inner().map(|x| x.as_str().to_string()).collect();
        let allergens: Vec<String>  = pair.next().unwrap().into_inner().map(|x| x.as_str().to_string()).collect();
        println!("Ingredients: {:?}", ingredients);
        println!("Allegerns: {:?}", allergens);
        foods.push((ingredients, allergens));
    }

    let mut possibleingredients: HashMap<String, HashSet<String>>=HashMap::new();

    for (ingredients, allergens) in foods.iter() {
        let singredients: HashSet<String> = ingredients.iter().map(|s| (*s).clone()).collect();
        for allergen in allergens {
            if possibleingredients.contains_key(allergen) {
                let intersection: HashSet<String> = singredients.intersection(&possibleingredients[&allergen.clone()]).cloned().collect();
                *possibleingredients.get_mut(allergen).unwrap() = intersection;
            } else {
                possibleingredients.insert(allergen.clone(), singredients.clone());
            }
        }
    }

    let possibleingredients2 = possibleingredients.clone();

    println!("{:#?}", possibleingredients);

    let mut allergen_from_ingredient: HashMap<String, String> = HashMap::new();

    while possibleingredients.len() > 0 {
        for (allergen, ingredients) in possibleingredients.clone().iter() {
            if ingredients.len() == 1 {
                // This allergen can only be found in a single ingredient.
                println!("YES {}, {:?}", allergen, ingredients);
                allergen_from_ingredient.insert(allergen.clone(), ingredients.iter().next().unwrap().clone());
                // println!("{:#?}", allergen_from_ingredient);

                possibleingredients.remove(allergen);
            }

        }
        for (allergen, ingredient) in allergen_from_ingredient.iter() {
            for (mut allergen2, mut ingredients2) in possibleingredients.clone().iter() {
                if ingredients2.contains(ingredient) {
                    // ingredients2.remove(&ingredient.clone());

                    let mut ingredients3 = ingredients2.clone();
                    ingredients3.remove(ingredient);
                    *possibleingredients.get_mut(allergen2).unwrap() = ingredients3;

                    // ingredients2.remove(ingredient);

                    // (*ingredients2).remove(ingredient);
                }
            }
        }
    }

    let ingredients_bad: Vec<String> = allergen_from_ingredient.clone().iter().map(|x| x.1.clone()).collect::<_>();

    let mut ingredients_good : Vec<String> = Vec::new();
    for (ingredients, allergens) in foods.iter() {
        println!("Ingredients {:?}", ingredients);
        for ingredient in ingredients {
            if ! ingredients_bad.contains(ingredient) {
                ingredients_good.push(ingredient.clone());
            }
        }
    }
    println!("Number of good ingredients: {}", ingredients_good.len());

    // Experiment how intersection works.
    // let mut set1: HashSet<String> = ["hello", "world", "abc"].iter().map(|s| s.to_string()).collect();
    // let mut set2: HashSet<String> = ["tef", "world", "abc"].iter().map(|s| s.to_string()).collect();
    // println!("Set 1: {:?}", set1);
    // println!("Set 2: {:?}", set2);
    // let intersection: HashSet<_> = set1.intersection(&set2).collect();
    // println!("Set X: {:?}", intersection);
    // println!("Set 1: {:?}", set1);
    // println!("Set 2: {:?}", set2);

}
