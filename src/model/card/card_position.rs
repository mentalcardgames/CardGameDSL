use crate::model::function_types::{TCardPosition};
use crate::model::gamedata::game_data::{GameData};
use std::collections::HashMap;
use crate::model::location::location_ref::{LocationRef};
use crate::model::card::card::{Card};
use std::sync::Arc;

pub struct CardPosition {
    pub pos: TCardPosition,
    pub str_repr: String,
}
impl CardPosition {
    pub fn get_card_position(&self, gd: &GameData) -> HashMap<LocationRef, Vec<Card>> {
        (self.pos)(gd)
    }
}
impl Clone for CardPosition {
    fn clone(&self) -> Self {
        CardPosition { 
            pos: Arc::clone(&self.pos),
            str_repr: self.str_repr.clone(),
        }
    }
}
