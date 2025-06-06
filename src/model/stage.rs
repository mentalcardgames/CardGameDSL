use std::collections::HashMap;
use std::fmt;

use crate::model::rules::rule::Rule;
use crate::model::end_condition::EndCondition;
use crate::model::base_types::ref_player::RefPlayer;



pub struct Stage {
    pub name: String,
    pub endconditions: Vec<EndCondition>,
    pub substages: Vec<Stage>,
    // TODO: should be synchronuzed with the gamedata turncounter (self.current)
    pub turncounter: i32,
    pub rules: Vec<Rule>,
    pub pref: RefPlayer,
    // Keeping track how often a Player has been in this Stage
    pub reps: HashMap<String, usize>,
    // Players Out of this Stage:
    // Are not allowed to play in this stage anymore but the others are still able to participate in the Stage
    pub playersout: HashMap<String, bool>,
    // Name of current Player (Because of some Examples of Games it is necessary)
    // Example: 
    // set Player out of stage
    // cycle to next
    pub current: String,
    pub str_repr: String,
}

impl Stage {
    pub fn new(name: &str) -> Self {
        Stage {
            name: String::from(name),
            endconditions: vec![],
            substages: vec![],
            turncounter: 0,
            rules: vec![],
            pref: RefPlayer::default(),
            reps: HashMap::new(),
            playersout: HashMap::new(),
            current: String::from(""),
            str_repr: String::default(),
        }
    }

    pub fn add_setup_rule(&mut self, setup: Rule) {
        self.rules.push(setup);
    }

    pub fn add_play_rule(&mut self, play: Rule) {
        self.rules.push(play);
    }

    pub fn add_scoring_rule(&mut self, scoring: Rule) {
        self.rules.push(scoring);
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_sub_stage<'a>(&'a mut self, sub: Stage) {
        self.substages.push(sub);
    }

    pub fn add_end_condition(&mut self, endcond: EndCondition) {
        self.endconditions.push(endcond);
    }

    pub fn set_player_reference(&mut self, pref: RefPlayer) {
        self.pref = pref;
    }

    // do this before u start the Stage
    fn init_reps(&mut self, players: &Vec<String>) {
        self.current = players[0].clone();

        for p in players.iter() {
            self.reps.insert(p.clone(), 0);
        }
    }

    fn init_playersout(&mut self, players: &Vec<String>) {
        self.current = players[0].clone();

        for p in players.iter() {
            self.playersout.insert(p.clone(), false);
        }
    }

    pub fn init_stage_logic(&mut self, players: &Vec<String>) {
        self.init_reps(players);
        self.init_playersout(players);
    }

    // only update if the player is ends his turn (so if something changes in the GameFlow)
    pub fn update_reps(&mut self) {
        self.reps
            .entry(self.current.clone())
            .and_modify(|v| *v += 1);
    }

    // set player out
    pub fn set_player_out(&mut self, player: &str) {
        self.playersout
            .entry(String::from(player))
            .and_modify(|v| *v = true);
    }

    pub fn set_current(&mut self, name: &str) {
        self.current = String::from(name);
    }

    pub fn get_current_reps(&self, name: &str) -> usize {
        if let Some(rep) = self.reps.get(name) {
            return *rep;
        }

        // TODO:
        // what if player is not found?
        return 0
    }

    pub fn is_player_out(&self, name: &str) -> bool {
        if let Some(b) = self.playersout.get(name) {
            return *b 
        }

        // TODO:
        // Default Value
        // Give a message or crash game if Player is not found!
        false
    }
}
impl<'a> Clone for Stage {
    fn clone(&self) -> Self {
        Stage {
            name: self.name.clone(),
            endconditions: self.endconditions.clone(),
            substages: self.substages.clone(),
            turncounter: self.turncounter,
            rules: self.rules.clone(),
            pref: self.pref.clone(),
            reps: self.reps.clone(),
            playersout: self.playersout.clone(),
            current: self.current.clone(),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl fmt::Debug for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stage")
            .field("name", &self.name)
            .field("endconditions", &self.endconditions)
            .field("substages", &self.substages)
            .field("turncounter", &self.turncounter)
            .field("rules", &self.rules)
            .field("pref", &"<function>") // Custom placeholder for non-Debug field
            .finish()
    }
}
