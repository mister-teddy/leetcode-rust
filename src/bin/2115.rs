use std::collections::HashSet;

/// 2115. Find All Possible Recipes from Given Supplies
///
/// You have information about `n` different recipes. You are given a string array `recipes` and a 2D string array `ingredients`.
/// The `i`th recipe has the name `recipes[i]`, and you can create it if you have all the needed ingredients from `ingredients[i]`.
/// A recipe can also be an ingredient for other recipes, i.e., `ingredients[i]` may contain a string that is in `recipes`.
///
/// You are also given a string array `supplies` containing all the ingredients that you initially have, and you have an infinite supply of all of them.
///
/// Return a list of all the recipes that you can create. You may return the answer in any order.
///
/// Note that two recipes may contain each other in their ingredients.
///
/// Constraints:
/// - `n == recipes.length == ingredients.length`
/// - `1 <= n <= 100`
/// - `1 <= ingredients[i].length, supplies.length <= 100`
/// - `1 <= recipes[i].length, ingredients[i][j].length, supplies[k].length <= 10`
/// - `recipes[i], ingredients[i][j], and supplies[k]` consist only of lowercase English letters.
/// - All the values of `recipes` and `supplies` combined are unique.
/// - Each `ingredients[i]` does not contain any duplicate values.
struct Solution {}

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        // We can solve this problem using a hash set
        // First we start with the elements in supplies
        let mut pot: HashSet<String> = HashSet::from_iter(supplies);
        loop {
            // Keep track of how many new recipes we've created
            let previous_pot_size = pot.len();
            // Keep looping over the recipes
            for (i, recipe) in recipes.iter().enumerate() {
                if !pot.contains(recipe) {
                    let mut can_create = true;
                    for ingredient in &ingredients[i] {
                        // Reject the recipe if the pot does not have the necessary ingredient
                        if !pot.contains(ingredient) {
                            can_create = false;
                            break;
                        }
                    }
                    if can_create {
                        pot.insert(recipe.clone());
                    }
                }
            }
            // If no new recipe has been created in this iteration, that's the most we can reach
            if pot.len() == previous_pot_size {
                break;
            }
        }
        // Creatable recipes are those inside the pot
        recipes
            .into_iter()
            .filter(|recipe| pot.contains(recipe))
            .collect()
    }
}

fn main() {
    fn to_string_vec(input: &[&str]) -> Vec<String> {
        input.iter().map(|&s| s.to_string()).collect()
    }

    // Example 1
    let recipes = to_string_vec(&["bread"]);
    let ingredients = vec![to_string_vec(&["yeast", "flour"])];
    let supplies = to_string_vec(&["yeast", "flour", "corn"]);
    let result = Solution::find_all_recipes(recipes.clone(), ingredients.clone(), supplies.clone());
    assert_eq!(result, vec!["bread"]);

    // Example 2
    let recipes = to_string_vec(&["bread", "sandwich"]);
    let ingredients = vec![
        to_string_vec(&["yeast", "flour"]),
        to_string_vec(&["bread", "meat"]),
    ];
    let supplies = to_string_vec(&["yeast", "flour", "meat"]);
    let result = Solution::find_all_recipes(recipes.clone(), ingredients.clone(), supplies.clone());
    assert_eq!(result, vec!["bread", "sandwich"]);

    // Example 3
    let recipes = to_string_vec(&["bread", "sandwich", "burger"]);
    let ingredients = vec![
        to_string_vec(&["yeast", "flour"]),
        to_string_vec(&["bread", "meat"]),
        to_string_vec(&["sandwich", "meat", "bread"]),
    ];
    let supplies = to_string_vec(&["yeast", "flour", "meat"]);
    let result = Solution::find_all_recipes(recipes.clone(), ingredients.clone(), supplies.clone());
    assert_eq!(result, vec!["bread", "sandwich", "burger"]);
}
