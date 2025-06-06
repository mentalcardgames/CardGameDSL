use crate::model::function_types::{TBool};
use crate::model::card_game_model::{CardGameModel};
use std::sync::Arc;

pub struct GBool {
    pub value: TBool,
    pub str_repr: String,
}
impl Clone for GBool {
    fn clone(&self) -> Self {
        GBool { 
            value: Arc::clone(&self.value),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl GBool {
    pub fn get_value(&self, cgm: &CardGameModel) -> bool {
        (self.value)(cgm)
    }
}
