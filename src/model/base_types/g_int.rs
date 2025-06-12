use crate::model::function_types::{TInt};
use crate::model::gamedata::game_data::{GameData};
use std::sync::Arc;


pub struct GInt {
    pub value: TInt,
    pub str_repr: String,
}
impl GInt {
    pub fn get_value_isize(&self, gd: &GameData) -> isize {
        (self.value)(gd)
    }

    pub fn get_value_usize(&self, gd: &GameData) -> usize {
        (self.value)(gd) as usize
    }
}
impl Clone for GInt {
    fn clone(&self) -> Self {
        GInt { 
            value: Arc::clone(&self.value),
            str_repr: self.str_repr.clone(),
        }
    }
}