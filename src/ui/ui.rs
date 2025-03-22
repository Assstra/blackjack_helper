use ratatui::{style::Stylize, symbols::border, text::Line, widgets::Block, Frame};
use crate::{game::blackjack::GameState, App};
use super::{board::draw_board, debug::draw_app_debug, player_menu::player_menu};

pub fn ui(frame: &mut Frame, app: &App) {
    let title = Line::from(" BlackJack ".bold());

    let game = &app.game;

    // let instructions = Line::from(vec![
    //     " Decrement ".into(),
    //     "<Left>".blue().bold(),
    //     " Increment ".into(),
    //     "<Right>".blue().bold(),
    //     " Quit ".into(),
    //     "<Q> ".blue().bold(),
    // ]);

    let block = Block::bordered()
        .title(title.centered())
        //.title_bottom(instructions.centered())
        .border_set(border::THICK);

    if let GameState::PlayerTurn(_, _) = app.game.game_state {
        if app.game.input_wanted {
            player_menu(game, frame, app.menu_selected_value);
        }
    }

    draw_board(game, frame);
    draw_app_debug(app, frame);
    frame.render_widget(block, frame.area());
}
