use ratatui::{widgets::Wrap, Frame};

use crate::App;

use ratatui::{ widgets::Paragraph, layout::Rect};


pub fn draw_app_debug(app: &App, frame: &mut Frame) {
    if app.show_debug {
        frame.render_widget(
            Paragraph::new(format!("{:?} {:?} {:?}", app.game.players[0].chips, app.game.game_state, app.game.dealer)).wrap(Wrap { trim: false }),
            Rect::new(100, 2, 80, 42),
        );
    }
}