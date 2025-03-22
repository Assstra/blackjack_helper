use std::fmt::{Display, Formatter};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Hearts,      // ♥️
    Diamonds,   // ♦️
    Clubs,      // ♣️
    Spades,     // ♠️
}

impl Suit {
    /// Create an iterator over suites
    pub fn iter() -> impl Iterator<Item = Suit> {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades].iter().copied()
    }   
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Suit::Diamonds =>  "♦️",
            Suit::Clubs => "♣️",
            Suit::Hearts => "♥️",
            Suit::Spades => "♠️"
        })
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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::AceH | Value::AceL => "A",
        })
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
    pub is_shown: bool,
}

impl Card {
    pub fn new(suit: Suit, value: Value, is_shown: bool) -> Self {
        Card {
            suit,
            value,
            is_shown,
        }
    }
}