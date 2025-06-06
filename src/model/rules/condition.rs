use std::fmt;

use crate::model::base_types::g_bool::{GBool};
use crate::model::card_game_model::{CardGameModel};


pub struct Condition {
    pub condition: GBool,
    pub str_repr: String,
}
impl Condition {
    pub fn evaluate(&self, cgm: &CardGameModel) -> bool {
        self.condition.get_value(cgm)
    }
}
impl Clone for Condition {
    fn clone(&self) -> Self {
        Condition {
            condition: self.condition.clone(),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<Condition>")
    }
}
