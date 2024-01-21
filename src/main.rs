use std::io::stdin;

use crate::player::Player;
mod player;

fn main() {
    println!("Hello, world!");
    // TODO:
    // 1. Ask the user how many players
    while {
        let mut s: String = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        let input = s.trim().parse::<u8>();
        match input {
            Ok(input) => {
                println!("You entered {}", input);
            }
            Err(..) => {
                println!("Please enter a number!");
            }
        }
        input.is_err()
    } { /* EMPTY */ }
    // 1. Create the player(s)
    // 2. Create the dealer
    // 3. Create the deck
    // 4. Shuffle the deck
    // 5. Deal the cards
    // 6. Show the cards
    // 7. Ask the player to hit or stand
    // 8. If hit, deal another card
    // 9. If stand, dealer deals cards until 17 or higher
    // 10. If player busts, dealer wins
    // 11. If dealer busts, player wins
    // 12. If player has higher value, player wins
}
