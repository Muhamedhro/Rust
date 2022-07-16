
// Zoek algoritmen:
// Binary search
// Linear search
// Tree search
// Breadth first search
// Depth first search

use math::round::floor;

// Helper functie
fn make_even(input_number: f32, add_one: bool) -> f32
{
    if input_number % 2f32 == 0f32
    {
        return input_number;
    }
    else
    {
        let mut output_number = input_number.floor();
        if add_one
        {
            output_number = input_number + 1f32;
        }
        else 
        {
            output_number = input_number - 1f32;
        }
        return output_number;
    }
}

// Voor nu een lijst met getallen doorzoeken
// er zit nog een bugje in
pub fn binary_search(item_to_find: f32, list: &[f32]) -> Option<f32>
{
    println!("Start binary search.");

    // Heeft lijst even of oneven items, deze check hoeft eigenlijk niet, rond maar af naar beneden
    let middle_of_list = make_even((list.len() as f32 / 2.0).floor(), false) as usize;
    let mut current_item = list[middle_of_list];

    // Bij het midden beginnen, is het groter, dan moeten we de helft van de helft naar achteren
    // Is hij groter dan moet hij de helft van de helft naar voren
    // En zo door tot we het resultaat hebben
    let mut steps_to_move = make_even((middle_of_list as f32 / 2.0).floor(), false);

    while current_item != item_to_find
    {
        if item_to_find < current_item
        {
            println!("Item -> {} is KLEINER dan huidige item -> {}", item_to_find, current_item);
            current_item -= steps_to_move;
            steps_to_move /= 2f32;
            steps_to_move = make_even(steps_to_move.floor(), false);
        }
        else if item_to_find > current_item
        {
            println!("Item -> {} is GROTER dan huidige item -> {}", item_to_find, current_item);
            current_item += steps_to_move;
            steps_to_move /= 2f32;
            steps_to_move = make_even(steps_to_move.floor(), false);
        }
        else if item_to_find == current_item
        {
            println!("GEVONDEN! Item om te zoeken -> {}, gevonden -> {}", item_to_find, current_item);
            return Some(item_to_find);
        }
    }
    Some(0f32)
}