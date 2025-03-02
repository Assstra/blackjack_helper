use crate::player::Player;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use player::{Dealer, Participant};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use core::deck::Deck;
use std::io::{stdin, stdout, Result};

mod cli;
mod core;
mod player;

const DEFAULT_BLACKJACK_DECK_NUMBER: u8 = 8;

fn main() -> Result<()> {
    let nb_players = cli::input_cli();

    // Create players
    let mut players: Vec<Player> = Vec::new();
    for _ in 0..nb_players {
        players.push(Player::new());
    }
    let mut dealer = Dealer::new();

    // Create deck
    let mut deck = Deck::new(DEFAULT_BLACKJACK_DECK_NUMBER);
    deck.shuffle_deck();

    // Draw cards
    for _ in 0..2 {
        for player in &mut players {
            player.draw_card(&mut deck);
        }
        dealer.draw_card(&mut deck);
    }

    // While each player plays
    for player in &mut players {
        while player.calculate_score() <= 21 {
            let mut s: String = String::new();
            stdin().read_line(&mut s);

            let input = s.trim().parse::<String>();
            match input {
                Ok(input) => {
                    if input == String::from("hit"){
                        player.draw_card(&mut deck);
                    }
                }
                Err(..) => {
                    println!("Please enter a char!");
                }
            };
        }
    }

    // 6. Show the cards
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    loop {
        // TODO draw the UI
        // TODO handle events
    }
    disable_raw_mode()?;
    Ok(())
    // 7. Ask the player to hit or stand
    // 8. If hit, deal another card
    // 9. If stand, dealer deals cards until 17 or higher
    // 10. If player busts, dealer wins
    // 11. If dealer busts, player wins
    // 12. If player has higher value, player wins
}
