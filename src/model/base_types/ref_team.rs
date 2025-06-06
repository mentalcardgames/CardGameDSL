use crate::model::function_types::{TRefTeam};
use crate::model::gamedata::game_data::{GameData};
use crate::model::owners::team::{Team};
use std::sync::Arc;

pub struct RefTeam {
    pub team: TRefTeam,
    pub str_repr: String,
}
impl Clone for RefTeam {
    fn clone(&self) -> Self {
        RefTeam { 
            team: Arc::clone(&self.team),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl RefTeam {
    pub fn get_ref(&self, gd: &GameData) -> Team {
        (self.team)(gd)
    }
}
