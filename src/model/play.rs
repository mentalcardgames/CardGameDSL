use std::collections::HashMap;

use crate::model::rules::condition::Condition;
use crate::model::stage::Stage;


#[derive(Debug)]
pub struct Play {
    // Should be endcondition???
    pub endconditions: Vec<Condition>,
    pub stages: Vec<Stage>,
    // current player
    pub current: String,
    pub reps: HashMap<String, usize>,
    pub outofplay: HashMap<String, bool>,
    pub str_repr: String,
}
impl Default for Play {
    fn default() -> Self {
        Play {
            endconditions: vec![], // or panic!(), or skip, or clone dummy data
            stages: vec![],
            current: String::default(),
            reps: HashMap::new(),
            outofplay: HashMap::new(),
            str_repr: String::default(),
        }
    }
}
impl Clone for Play {
    fn clone(&self) -> Self {
        Play {
            endconditions: self.endconditions.clone(), // or panic!(), or skip, or clone dummy data
            stages: self.stages.clone(),
            current: self.current.clone(),
            reps: self.reps.clone(),
            outofplay: self.outofplay.clone(),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl Play {
    pub fn add_endcondition(&mut self, end_cond: Condition) {
        self.endconditions.push(end_cond);
    }

    pub fn add_stage(&mut self, stage: Stage) {
        self.stages.push(stage);
    }

    pub fn out_of_play_init(&mut self, players: &Vec<String>) {
        for p in players.iter() {
            self.outofplay.insert(p.clone(), false);
        }
    }

    pub fn is_player_out(&mut self, player: &str) -> bool {
        if let Some(b) = self.outofplay.get(player) {
            return *b
        }

        // TODO:
        // Default value
        false
    }

    pub fn set_player_out(&mut self, player: &str) {
        self.outofplay
            .entry(String::from(player))
            .and_modify(|v| *v = true);   
    }
}
