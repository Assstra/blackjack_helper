use ratatui::Frame;

use crate::{game::blackjack::Game, ui::player::draw_player_cards};

use super::dealer::draw_dealer_cards;

pub fn draw_board(game: &Game, frame: &mut Frame) {
    let players = &game.players;
    let dealer = &game.dealer;

    const CARD_WIDTH: u16 = 5;
    const CARD_HEIGHT: u16 = 4;

    // Dealer cards
    draw_dealer_cards(dealer, CARD_WIDTH, CARD_HEIGHT, frame);

    // Player cards
    draw_player_cards(players, CARD_WIDTH, CARD_HEIGHT, frame);
}
