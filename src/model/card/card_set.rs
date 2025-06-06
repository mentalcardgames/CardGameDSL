use crate::model::function_types::{TCardSet};
use crate::model::gamedata::game_data::{GameData};
use std::collections::HashMap;
use crate::model::location::location_ref::{LocationRef};
use crate::model::card::card::{Card};
use std::sync::Arc;

pub struct CardSet {
    pub set: TCardSet,
    pub str_repr: String,
}
impl CardSet {
    pub fn get_card_set(&self, gd: &GameData) -> HashMap<LocationRef, Vec<Card>> {
        (self.set)(gd)
    }
}
impl Clone for CardSet {
    fn clone(&self) -> Self {
        CardSet { 
            set: Arc::clone(&self.set),
            str_repr: self.str_repr.clone(),
        }
    }
}
