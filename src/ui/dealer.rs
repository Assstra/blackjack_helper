use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::{core::cards::Suit, player::Dealer};

pub fn draw_dealer_cards(dealer: &Dealer, card_width: u16, card_height: u16, frame: &mut Frame) {
    // Dealer cards
    frame.render_widget(
        Paragraph::new(format!("Dealer")).style(Style::default().fg(Color::LightRed)),
        Rect::new(91, 2, frame.area().width, 1),
    );

    for (card_index, card) in dealer.hand.cards.iter().enumerate() {
        // Calculate the position of the card
        let card_x = 89 + 5 * card_index as u16;
        let card_y = 3;

        // Draw the card as a Block with borders
        let card_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        // Render the card block
        frame.render_widget(
            card_block,
            Rect::new(card_x, card_y, card_width, card_height),
        );

        if card.is_shown {
            // Draw the card value at the top-center of the card
            frame.render_widget(
                Paragraph::new(card.value.to_string()).style(Style::default().fg(Color::White)),
                Rect::new(card_x + 2, card_y + 1, card_width, 1),
            );

            // Draw the card suit in the center of the card
            frame.render_widget(
                Paragraph::new(card.suit.to_string()).style(Style::default().fg(match card.suit {
                    Suit::Hearts | Suit::Diamonds => Color::Red,
                    Suit::Clubs | Suit::Spades => Color::Black,
                })),
                Rect::new(card_x + 3, card_y + 3, card_width, 1),
            );
        }
    }
}
