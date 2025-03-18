use crate::player::Player;
use core::deck::Deck;
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
use std::io::Result;

mod cli;
mod core;
mod player;

const DEFAULT_BLACKJACK_DECK_NUMBER: u8 = 8;

fn main() -> Result<()> {
    println!("=====================================");
    println!("🎰 Welcome to Blackjack! 🎰");
    println!("=====================================");

    let nb_players = cli::input_cli();
    println!("🔢 Number of players: {}", nb_players);

    // Create players
    let mut players: Vec<Player> = Vec::new();
    for i in 1..=nb_players {
        let player = Player::new();
        println!("👤 Player {} has joined the game!", i);
        players.push(player);
    }
    let mut dealer = Dealer::new();
    println!("🤵 The dealer is ready.");

    // Create deck
    let mut deck = Deck::new(DEFAULT_BLACKJACK_DECK_NUMBER);
    println!("🃏 Initializing deck with {} decks...", DEFAULT_BLACKJACK_DECK_NUMBER);
    deck.shuffle_deck();
    println!("🔀 Deck shuffled!\n");

    // Draw cards
    println!("🎴 Dealing initial cards...\n");
    for _ in 0..2 {
        for (i, player) in players.iter_mut().enumerate() {
            player.draw_card(&mut deck, 0);
            println!("🃏 Player {} drew a card.", i + 1);
        }
        dealer.draw_card(&mut deck, 0);
        println!("🤵 Dealer drew a card.\n");
    }

    // Game loop
    println!("========== 🃏 Game Starting! 🃏 ==========\n");

    // Players' turns
    for (i, player) in players.iter_mut().enumerate() {
        let player_id = i + 1;
        println!("➡️ Player {}'s turn: {:?}", player_id, player.hand);

        if player.can_split() {
            let input = cli::hit_or_stand_input();
    
            if input == "split" {

                // Split the initial cards & Create the second hand (index 1) after the split
                let second_card = player.hand[0].remove(1).clone();
                player.hand.push(vec![second_card]);

                // Play 1st split
                println!("🃏 Player {} splits: split 1", player_id);
                player.draw_card(&mut deck, 0);
                player.play_hand(&mut deck, 0, player_id);

                // Play 2nd split
                println!("🃏 Player {} splits: split 2", player_id);
                player.draw_card(&mut deck, 1);
                player.play_hand(&mut deck, 1, player_id);
            }
        } else {
            player.play_hand(&mut deck, 0, player_id);
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
