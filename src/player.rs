use crate::core::{cards::Card, deck::Deck};

pub trait Participant {
    fn draw_card(&mut self, deck: &mut Deck);
    fn calculate_score(&self) -> u8;
    fn clear_hand(&mut self);
}

#[derive(Clone)]
pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Player { hand: Vec::new() }
    }
}

impl Participant for Player {
    fn draw_card(&mut self, deck: &mut Deck) {
        let card = deck.pick_card();
        self.hand.push(card);
    }

    fn calculate_score(&self) -> u8 {
        self.hand.iter().map(|card| card.value.to_u8()).sum()
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

    pub fn should_hit(&self) -> bool {
        self.calculate_score() < 17
    }
}

impl Participant for Dealer {
    fn draw_card(&mut self, deck: &mut Deck) {
        let card = deck.pick_card();
        self.hand.push(card);
    }

    fn calculate_score(&self) -> u8 {
        self.hand.iter().map(|card| card.value.to_u8()).sum()
    }

    fn clear_hand(&mut self) {
        self.hand.clear();
    }
}
