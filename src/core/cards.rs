use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone)]
pub enum Suit {
    heart,      // ♥️
    diamonds,   // ♦️
    clubs,      // ♣️
    spades,     // ♠️
}

#[derive(EnumIter)]
pub enum Value {
    Two = 0,    // 2
    Three = 1,  // 3
    Four = 2,   // 4
    Five = 3,   // 5
    Six = 4,    // 6
    Seven = 5,  // 7
    Eight = 6,  // 8
    Nine = 7,   // 9
    Ten = 8,    // 10
    Jack = 9,   // J
    Queen = 10, // Q
    King = 11,  // K
    Ace = 12,   // A
}

pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Card {
            suit,
            value,
        }
    }
}