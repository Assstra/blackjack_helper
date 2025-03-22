use super::core::{
        cards::Card,
        deck::Deck,
    };

#[derive(Clone)]
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub score: u8,
    pub bet: f32,
}

impl Hand {
    pub fn new(bet: f32) -> Self {
        Hand {
            cards: Vec::new(),
            score: 0,
            bet: bet,
        }
    }

    fn update_score(&mut self, score: u8) {
        self.score = score;
    }

    pub fn draw_card(&mut self, deck: &mut Deck, is_shown: bool)  {
        let mut card = deck.pick_card();
        card.is_shown = is_shown;
        self.cards.push(card);
    }

    pub fn calculate_score(&mut self) -> u8 {
        let score = self.cards
            .iter()
            .map(|card| card.value.numeric_value())
            .sum();

        self.update_score(score);

        score
    }
}
