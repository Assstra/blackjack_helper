#[derive(Debug, Clone)]
pub enum PlayerAction {
    Hit,
    Stand,
    Split,
    None,
}

#[derive(Debug)]
pub struct PlayerActions {
    actions: Vec<PlayerAction>,
}

impl PlayerActions {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn set_player_actions(&mut self, player_actions: Vec<PlayerAction>) {
        self.clear();
        for action in player_actions {
            self.actions.push(action);
        }
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn get_actions(&self) -> &Vec<PlayerAction> {
        &self.actions
    }
}
