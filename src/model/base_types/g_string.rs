use crate::model::function_types::{TString};
use crate::model::gamedata::game_data::{GameData};
use std::sync::Arc;


pub struct GString {
    pub string: TString,
    pub str_repr: String,
}
impl Clone for GString {
    fn clone(&self) -> Self {
        GString { 
            string: Arc::clone(&self.string),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl GString {
    pub fn get_string(&self, gd: &GameData) -> String {
        (self.string)(gd)
    }
}
