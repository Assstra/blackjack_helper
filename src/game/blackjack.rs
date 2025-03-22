use crate::game::{core::deck::Deck, player::Player};
use player::{Dealer, Participant};

use super::{
    player::{self},
    player_actions::{PlayerAction, PlayerActions},
};

const DEFAULT_BLACKJACK_DECK_NUMBER: u8 = 8;

#[derive(PartialEq, Debug)]
pub enum GameState {
    CreatingGame,
    InitializingTurn,
    DealingCards,
    PlayerTurn(usize, usize), // Player ID, Hand ID
    DealerTurn,
    CalculatingResults,
    TurnOver,
    EndGame,
}

#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub players: Vec<Player>,
    pub dealer: Dealer,
    pub game_state: GameState,
    pub input_wanted: bool,
    pub available_actions: PlayerActions,
    pub player_action_selected: PlayerAction,
}

impl Game {
    pub fn new() -> Self {
        Game {
            deck: Deck::new(DEFAULT_BLACKJACK_DECK_NUMBER),
            players: Vec::new(),
            dealer: Dealer::new(),
            game_state: GameState::InitializingTurn,
            input_wanted: false,
            player_action_selected: PlayerAction::None,
            available_actions: PlayerActions::new(),
        }
    }

    pub fn init_game(&mut self, nb_players: u8) {
        // Create players
        for _ in 1..=nb_players {
            let player = Player::new();
            self.players.push(player);
        }
        self.deck.shuffle_deck();

        self.game_state = GameState::InitializingTurn;
    }

    pub fn init_turn(&mut self) {
        // Draw cards
        for num in 1..=2 {
            for player in self.players.iter_mut() {
                player.hands[0].draw_card(&mut self.deck, true);
                // println!("🃏 Player {} drew a card.", i + 1);
            }
            self.dealer.hand.draw_card(&mut self.deck, num == 2);
        }

        self.game_state = GameState::PlayerTurn(0, 0);
        self.player_turn(0, 0);
    }

    pub fn player_turn(&mut self, player_id: usize, hand_id: usize) {
        // Players' turns

        let player = &mut self.players[player_id];
        let hand = &mut player.hands[hand_id];

        if hand.calculate_score() >= 21 {
            self.stand(player_id, hand_id);
        } else {
            let mut actions = Vec::new();
            actions.push(PlayerAction::Hit);
            actions.push(PlayerAction::Stand);

            if hand_id == 0 && hand.cards.len() == 2 {
                if player.can_split() {
                    actions.push(PlayerAction::Split);
                }
            }

            self.available_actions.set_player_actions(actions);
            self.input_wanted = true;
        }
    }

    pub fn hit(&mut self, player_id: usize, hand_id: usize) {
        self.players[player_id].hands[hand_id].draw_card(&mut self.deck, true);
        self.player_turn(player_id, hand_id);
    }

    pub fn stand(&mut self, player_id: usize, hand_id: usize) {
        if hand_id + 1 < self.players[player_id].hands.len() {
            // Same player, next hand
            self.game_state = GameState::PlayerTurn(player_id, hand_id + 1);
            self.player_turn(player_id, hand_id + 1);
        } else if player_id +1 < self.players.len() {
            // Next player, first hand
            self.game_state = GameState::PlayerTurn(player_id + 1, 0);
            self.player_turn(player_id + 1, 0);
        } else {
            self.game_state = GameState::DealerTurn;
            self.dealer_turn();
        }
    }

    pub fn split(&mut self, player_id: usize, _: usize) {
        self.players[player_id].split(&mut self.deck);
        self.player_turn(player_id, 0)
    }

    pub fn dealer_turn(&mut self) {

        self.dealer.hand.cards[0].is_shown = true;

        while self.dealer.should_hit() {
            self.dealer.hand.draw_card(&mut self.deck, true);
        }

        self.game_state = GameState::CalculatingResults;
        self.calculate_score();
    }

    pub fn calculate_score(&mut self) {
        // Final results
        let dealer_score = self.dealer.hand.score;

        for (_, player) in self.players.iter_mut().enumerate() {
            for hand in player.hands.iter_mut() {
                let player_score = hand.score;

                if player_score > 21 {
                    // 💀 Bust! Dealer wins.
                    player.chips = player.chips - hand.bet
                } else if dealer_score > 21 || player_score > dealer_score {
                    // 🎉 Player wins!
                    if player_score == 21 {
                        // Blackjack (initial bet + bet x 1.5)
                        player.chips += hand.bet * 1.5;// + hand.bet as f32;
                    } else {
                        // Player wins (initial bet + bet x 1)
                        player.chips += hand.bet //+ hand.bet) as f32;
                    }
                } else if player_score == dealer_score {
                    // Push (tie, initial bet)
                    player.chips += hand.bet as f32;
                } else {
                    // Dealer wins
                    player.chips = player.chips - hand.bet
                }
            }
        }
        self.game_state = GameState::TurnOver;
        //self.restart_turn();
    }

    pub fn restart_turn(&mut self) {
        for player in &mut self.players {
            player.clear_hands()
        }

        self.dealer.clear_hands();

        self.game_state = GameState::InitializingTurn;
        self.init_turn()
    }
}
