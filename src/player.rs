use crate::{cli, core::{cards::{Card, Value}, deck::Deck}};

pub trait Participant {
    fn draw_card(&mut self, deck: &mut Deck, hand_index: usize) -> &Card;
    fn play_hand(&mut self, deck: &mut Deck, hand_index: usize, player_id: usize) -> u8;
    fn calculate_score(&self, hand_index: usize) -> u8;
    fn clear_hand(&mut self);
}

#[derive(Clone)]
pub struct Player {
    pub hand: Vec<Vec<Card>>,
}

impl Player {
    pub fn new() -> Self {
        Player { hand: vec![Vec::new()] }
    }

    pub fn can_split(&mut self) -> bool {
        self.hand[0][0].value == self.hand[0][1].value
    }
}

impl Participant for Player {
    fn draw_card(&mut self, deck: &mut Deck, hand_index: usize) -> &Card {
        let card = deck.pick_card();
        self.hand[hand_index].push(card);
        
        self.hand[hand_index].last().unwrap()
    }

    fn play_hand(&mut self, deck: &mut Deck, hand_index: usize, player_id: usize) -> u8 {
        loop {
            let score = self.calculate_score(hand_index);
            println!("📊 Current score: {}", score);
    
            if score >= 21 {
                if score == 21 {
                    println!("🃏 Player {} BLACKJACK.", player_id);
                } else if self.hand[hand_index].iter().any(|card| card.value == Value::AceH) {
                    println!("🃏 Player {} has busted, adjusting Ace value to 1.", player_id);
                    self.hand[hand_index][index].value == Value::AceL
                }
                break // Stop loop if score is 21 or more
            } else {
                let input = cli::hit_or_stand_input();
    
                if input == "hit" {
                    let card = self.draw_card(deck, hand_index);
                    println!("🃏 Player {} hits and draws a {}.", player_id, card);
                } else {
                    break
                }
            }
        }
        
        let new_score = self.calculate_score(hand_index);
        println!("📊 New score: {}", new_score);
        
        new_score

    }

    fn calculate_score(&self, hand_index: usize) -> u8 {
        self.hand[hand_index].iter().map(|card| card.value.numeric_value()).sum()
    }

    fn clear_hand(&mut self) {
        self.hand.clear();
    }
}

pub struct Dealer {
    hand: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { hand: Vec::new() }
    }

    pub fn should_hit(&self, hand_index: usize) -> bool {
        self.calculate_score(hand_index) < 17
    }
}

impl Participant for Dealer {
    fn draw_card(&mut self, deck: &mut Deck, _hand_index: usize) -> &Card {
        let card = deck.pick_card();
        self.hand.push(card);
        self.hand.last().unwrap()
    }

    fn play_hand(&mut self, deck: &mut Deck, _hand_index: usize, _player_id: usize) -> u8 {
        while {
            let dealer_score = self.calculate_score(_hand_index);
            println!("📊 Dealer's score: {}", dealer_score);
    
            if self.should_hit(_hand_index) {
                let card = self.draw_card(deck, _hand_index);
                println!("🃏 Dealer hits and draws a {}", card );
                true  // Continue looping
            } else {
                println!("🛑 Dealer stands.");
                false // Stop looping
            }
        } {}
        self.calculate_score(_hand_index)
    }

    fn calculate_score(&self, _hand_index: usize) -> u8 {
        self.hand.iter().map(|card| card.value.numeric_value()).sum()
    }

    fn clear_hand(&mut self) {
        self.hand.clear();
    }
}
