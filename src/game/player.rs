use super::{core::deck::Deck, hand::Hand};

pub trait Participant {
    fn clear_hands(&mut self);
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Player {
    pub hands: Vec<Hand>,
    pub chips: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hands: vec![Hand::new(10.0)],
            chips: 500.0,
        }
    }

    pub fn can_split(&mut self) -> bool {
        self.hands[0].cards[0].value == self.hands[0].cards[1].value
    }

    pub fn split(&mut self, deck: &mut Deck) {
        if self.can_split() {
            let second_card = self.hands[0].cards.remove(1).clone();
            self.hands.push(Hand {
                cards: vec![second_card],
                score: 0,
                bet: 10.0,
            });

            let (hand_one, hand_two) = self.hands.split_at_mut(1);
            let hand_one = &mut hand_one[0];
            let hand_two = &mut hand_two[0];

            hand_one.draw_card(deck, true);
            hand_two.draw_card(deck, true);
        }
    }
}

impl Participant for Player {
    fn clear_hands(&mut self) {
        for hand in &mut self.hands {
            hand.cards.clear();
            hand.bet = 0.0;
        }
    }
}

#[derive(Debug)]
pub struct Dealer {
    pub hand: Hand,
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { hand: Hand::new(0.0) }
    }

    pub fn should_hit(&mut self) -> bool {
        self.hand.calculate_score() < 17
    }
}

impl Participant for Dealer {
    fn clear_hands(&mut self) {
        self.hand.cards.clear();
    }
}
