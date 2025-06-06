use std::collections::HashMap;

use crate::model::setup::Setup;
use crate::model::play::Play;
use crate::model::scoring::Scoring;
use crate::model::enums::out_of_game::OutOfGame;


#[derive(Debug, Clone)]
pub struct RuleSet {
    pub setup: Setup,
    pub play: Play,
    pub scoring: Scoring,
    // Player Names to keep in track who is still in the game!
    pub outofgame: HashMap<String, OutOfGame>,
    pub str_repr: String,
}
impl Default for RuleSet {
    fn default() -> Self {
        RuleSet {
            setup: Setup::default(),
            play: Play::default(),
            scoring: Scoring::default(),
            outofgame: HashMap::new(),
            str_repr: String::default(),
        }
    }
}
impl RuleSet {
    pub fn new() -> RuleSet {
        RuleSet {
            setup: Setup::default(),
            play: Play::default(),
            scoring: Scoring::default(),
            outofgame: HashMap::new(),
            str_repr: String::default(),
        }
    }

    pub fn assign_setup(&mut self, setup: Setup) {
        self.setup = setup;
    }

    pub fn assign_play(&mut self, play: Play) {
        self.play = play;
    }

    pub fn assign_scoring(&mut self, scoring: Scoring) {
        self.scoring = scoring;
    }

    pub fn out_of_game_init(&mut self, players: &Vec<String>) {
        for p in players.iter() {
            self.outofgame.insert(p.clone(), OutOfGame::None);
        }
    }

    pub fn is_player_out(&self, player: &str) -> bool {
        if let Some(b) = self.outofgame.get(player) {
            match b {
                OutOfGame::Successful => {
                    return true
                },
                OutOfGame::Fail => {
                    return true
                },
                OutOfGame::None => {
                    return false
                },
            }
        }

        // TODO:
        // Default value
        true
    }

    pub fn set_player_out_succ(&mut self, player: &str) {
        self.outofgame
            .entry(String::from(player))
            .and_modify(|v| *v = OutOfGame::Successful);   
    }

    pub fn set_player_out_fail(&mut self, player: &str) {
        self.outofgame
            .entry(String::from(player))
            .and_modify(|v| *v = OutOfGame::Fail);   
    }    
}
