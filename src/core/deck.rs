use super::cards::{Card, Suit, Value};
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(num_deck: u8) -> Self {
        let mut cards = Vec::new();
        for _ in 1..num_deck {
            for s in Suit::iter() {
                for v in Value::iter() {
                    cards.push(Card::new(s, v));
                }
            }
        }
        Self { cards }
    }

    pub fn shuffle_deck(&mut self) -> &Self {
        self.cards.shuffle(&mut thread_rng());
        return self;
    }

    pub fn pick_card(&mut self) -> Card {
        return self.cards.pop().expect("There is not enough cards in the deck left");
    }
}
