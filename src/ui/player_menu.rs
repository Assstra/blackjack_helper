use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::{game::blackjack::{Game, GameState}, PlayerAction};

pub fn player_menu(game: &Game, frame: &mut Frame, selected: usize) {
    let actions = game.available_actions.get_actions();

    if let GameState::PlayerTurn(player_id, _) = game.game_state {
        // Create a block for the menu
        let menu_block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Player {} - Choose an action", player_id))
            .style(Style::default().fg(Color::White));

        // Dynamically create paragraphs for the menu options
        let mut paragraphs = Vec::new();
        for (index, action) in actions.iter().enumerate() {
            let action_text = match action {
                PlayerAction::Hit => "Hit",
                PlayerAction::Stand => "Stand",
                PlayerAction::Split => "Split",
                PlayerAction::None => "None",
            };

            let paragraph_text = if selected == index {
                format!("> {} <", action_text) // Highlight selected action
            } else {
                action_text.to_string()
            };

            let paragraph = Paragraph::new(paragraph_text).style(Style::default().fg(Color::Yellow));

            paragraphs.push(paragraph);
        }

        // Calculate positions for the menu
        let menu_x = frame.area().width / 2;
        let menu_y = frame.area().height / 2;

        // Render the menu block
        frame.render_widget(
            menu_block,
            Rect::new(menu_x, menu_y, 20, paragraphs.len() as u16 + 2),
        ); // Adjust height based on the number of actions

        // Render each menu option dynamically
        for (index, paragraph) in paragraphs.iter().enumerate() {
            frame.render_widget(
                paragraph,
                Rect::new(menu_x + 2, menu_y + 1 + index as u16, 16, 1),
            );
        }
    }
}
