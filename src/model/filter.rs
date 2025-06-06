use std::sync::Arc;
use crate::model::gamedata::game_data::GameData;
use crate::model::function_types::TFilter;
use crate::model::card::card::Card;


pub struct Filter {
    pub func: TFilter,
    pub str_repr: String,
}
impl Filter {
    pub fn apply_func(&self, gd: &GameData, cards: Vec<Card>) -> Vec<Vec<Card>> {
        (self.func)(gd, cards)
    }
}
impl Clone for Filter {
    fn clone(&self) -> Self {
        Filter {
            func: Arc::clone(&self.func),
            str_repr: self.str_repr.clone(),
        }
    }
}
