use crate::model::function_types::{TRefPlayer};
use crate::model::gamedata::game_data::{GameData};
use crate::model::owners::player::{Player};
use std::sync::Arc;


pub struct RefPlayer {
    pub player: TRefPlayer,
    pub str_repr: String,
}
impl Default for RefPlayer {
    fn default() -> Self {
        RefPlayer { player:
            Arc::new(|gd: &GameData| {
                gd.get_player_copy(&gd.turnorder[gd.current])
            }),
            str_repr: String::default()
        }
    }
}
impl Clone for RefPlayer {
    fn clone(&self) -> Self {
        RefPlayer { 
            player: Arc::clone(&self.player),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl RefPlayer {
    pub fn get_ref(&self, gd: &GameData) -> Player {
        (self.player)(gd)
    }
}
