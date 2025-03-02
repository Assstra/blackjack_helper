
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
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 10,  // J
    Queen = 10, // Q
    King = 10,  // K
    Ace = 11,   // A (typically 1 or 11 in Blackjack)
}

impl Value {

    /// Create an iterator over values
    pub fn iter() -> impl Iterator<Item = Value> {
        [
            Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven, 
            Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King, Value::Ace
        ].iter().copied()
    }   

    /// Convert `Value` to `u8` so we can sum it
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Clone)]
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