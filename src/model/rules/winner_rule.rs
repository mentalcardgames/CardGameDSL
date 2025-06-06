use std::sync::Arc;

use crate::model::card_game_model::CardGameModel;
use crate::model::owners::player::Player;


pub struct WinnerRule {
    // evaluates to the winning Player name
    pub winner: Arc<dyn Fn(&CardGameModel) -> &Player>,
    pub str_repr: String,
}
impl std::fmt::Debug for WinnerRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WinnerRule(<closure>)")
    }
}
impl Clone for WinnerRule {
    fn clone(&self) -> Self {
        WinnerRule {
            winner: Arc::clone(&self.winner),
            str_repr: self.str_repr.clone()
        }
    }
}
impl WinnerRule {
    pub fn run(&self, cgm: &CardGameModel) {
        let winner = (self.winner)(cgm);
        println!("The Winner is: {}!", winner.name);
    } 
}

