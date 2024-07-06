// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";
    tokens = handle_cost(tokens, pretend_user_input).unwrap();
}

pub fn handle_cost(tokens: i32, input: &str)  -> Result<i32, ParseIntError> {
    let mut tokens = tokens;
    let cost = total_cost(input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(tokens)
}
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// the same as stuff above

// pub fn total_cost(user_input: &str) -> Result<i32, ParseIntError> {
//     let processing_fee = 1;
//     let cost_per_item = 5;
//     let input_result = user_input.parse::<i32>();
    
//     match input_result {
//         Ok(quantity) => Ok(quantity * cost_per_item + processing_fee),
//         Err(e) => Err(e)
//     }
// }

