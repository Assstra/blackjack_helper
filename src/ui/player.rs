use ratatui::{layout::Rect, style::{Color, Style}, widgets::{Block, Borders, Paragraph}, Frame};

use crate::game::{core::cards::Suit, player::Player};

pub fn draw_player_cards(players: &Vec<Player>, card_width: u16, card_height: u16, frame: &mut Frame) {

    const PADDING: u16 = 5;
    const MARGIN: u16 = 10; // Margin between cards in columns

    let mut x_offset = 2;

    for (player_index, player) in players.iter().enumerate() {
        // Draw player label
        frame.render_widget(
            Paragraph::new(format!("Player {} - {}", player_index + 1, player.chips))
                .style(Style::default().fg(Color::Yellow)),
            Rect::new(x_offset + PADDING, 42, frame.area().width, 1),
        );

        for hand in player.hands.iter() {
            for (card_index, card) in hand.cards.iter().enumerate() {
                // Calculate the position of the card
                let card_x = x_offset + PADDING + 3 * card_index as u16;
                let card_y = 38 - 3 * card_index as u16;

                // Draw the card as a Block with borders
                let card_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White));

                // Render the card block
                frame.render_widget(
                    card_block,
                    Rect::new(card_x, card_y, card_width, card_height),
                );

                // Draw the card value at the top-center of the card
                frame.render_widget(
                    Paragraph::new(card.value.to_string()).style(Style::default().fg(Color::White)),
                    Rect::new(card_x + 2, card_y + 1, card_width, 1),
                );

                // Draw the card suit in the center of the card
                frame.render_widget(
                    Paragraph::new(card.suit.to_string()).style(Style::default().fg(
                        match card.suit {
                            Suit::Hearts | Suit::Diamonds => Color::Red,
                            Suit::Clubs | Suit::Spades => Color::Black,
                        },
                    )),
                    Rect::new(card_x + 3, card_y + 3, card_width, 1),
                );
            }

            if hand.score > 21 {
                // Draw the skeleton head at the center of the last card in the hand
                let last_card_x = x_offset + PADDING + 3 * (hand.cards.len() - 1) as u16;
                let last_card_y = 38 - 3 * (hand.cards.len() - 1) as u16;
                frame.render_widget(
                    Paragraph::new("💀").style(Style::default().fg(Color::Red)),
                    Rect::new(last_card_x + 2, last_card_y + 2, card_width, 1),
                );
            }
        }
        // Update x_offset for the next card
        x_offset += card_width + MARGIN;
    }
}