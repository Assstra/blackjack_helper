use cli_log::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use game::{
    blackjack::{Game, GameState},
    player_actions::PlayerAction,
};
use ratatui::DefaultTerminal;
use std::io::{self, Error};

use ui::ui::ui;

mod game;
mod ui;

#[derive(Debug)]
pub struct App {
    pub game: Game,
    pub menu_selected_value: usize,
    pub show_debug: bool,
    pub exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            game: Game::new(),
            menu_selected_value: 0,
            show_debug: false,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // Initialize the game
        self.game.init_game(2);

        self.game.init_turn();

        while !self.exit && self.game.game_state != GameState::EndGame {
            terminal.draw(|frame| ui(frame, &self))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(250))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => {
                if self.game.input_wanted {
                    if let GameState::PlayerTurn(_, _) = self.game.game_state {
                        let actions = self.game.available_actions.get_actions();
                        self.menu_selected_value =
                            (self.menu_selected_value.wrapping_sub(1)) % actions.len();
                    }
                }
            }
            KeyCode::Down => {
                if self.game.input_wanted {
                    if let GameState::PlayerTurn(_, _) = self.game.game_state {
                        let actions = self.game.available_actions.get_actions();
                        self.menu_selected_value = (self.menu_selected_value + 1) % actions.len();
                    }
                }
            }
            KeyCode::Enter => {
                if matches!(self.game.game_state, GameState::PlayerTurn(_, _))
                    && self.game.input_wanted
                {
                    // if casting is possible
                    if let GameState::PlayerTurn(player_id, hand_id) = self.game.game_state {
                        // Disable input
                        self.game.input_wanted = false;
                        let menu_selected_index = self.menu_selected_value;
                        self.game.player_action_selected = self.game.available_actions.get_actions()[menu_selected_index].clone();

                        match self.game.player_action_selected {
                            PlayerAction::Hit => {
                                self.game.hit(player_id, hand_id);
                            }
                            PlayerAction::Stand => {
                                self.game.stand(player_id, hand_id);
                            }
                            PlayerAction::Split => {
                                self.game.split(player_id, hand_id)
                            }
                            PlayerAction::None => {
                                self.game.player_turn(player_id, 0)
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn main() -> Result<(), Error> {
    init_cli_log!();
    debug!("test");
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}
