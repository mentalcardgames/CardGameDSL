use crate::model::base_types::g_string::{GString};
use crate::model::gamedata::game_data::{GameData};


pub struct StringCollection {
    pub strings: Vec<GString>,
    pub str_repr: String,
}
impl StringCollection {
    pub fn eval_strings(&self, gd: &GameData) -> Vec<String> {
        self.strings.iter().map(|tint| tint.get_string(gd)).collect()
    }

    pub fn get_value_at(&self, gd: &GameData, index: usize) -> String {
        self.strings[index].get_string(gd)
    }

    pub fn get_at(&self, index: usize) -> GString {
        self.strings[index].clone()
    }
}