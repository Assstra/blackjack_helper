use std::fmt::Display;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Heart,      // ♥️
    Diamonds,   // ♦️
    Clubs,      // ♣️
    Spades,     // ♠️
}

impl Suit {
    /// Create an iterator over suites
    pub fn iter() -> impl Iterator<Item = Suit> {
        [Suit::Heart, Suit::Diamonds, Suit::Clubs, Suit::Spades].iter().copied()
    }   
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,  // J
    Queen, // Q
    King,  // K
    AceH,  // A 11
    AceL,  // A 1
}

impl Value {

    pub fn numeric_value(&self) -> u8 {
        match self {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
            Value::AceH => 11,
            Value::AceL => 1,
        }
    }
    
    /// Create an iterator over values
    pub fn iter() -> impl Iterator<Item = Value> {
        [
            Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven, 
            Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King, Value::AceH
        ].iter().copied()
    }   

}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.value, self.suit)
    }
}

#[derive(Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Card {
            suit,
            value,
        }
    }
}