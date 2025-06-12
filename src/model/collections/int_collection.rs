use crate::model::base_types::g_int::{GInt};
use crate::model::gamedata::game_data::{GameData};


pub struct IntCollection {
    pub ints: Vec<GInt>,
    pub str_repr: String,
}
impl IntCollection {
    pub fn eval_ints(&self, gd: &GameData) -> Vec<isize> {
        self.ints.iter().map(|tint| tint.get_value_isize(gd)).collect()
    }

    pub fn get_isize_at(&self, gd: &GameData, index: usize) -> isize {
        self.ints[index].get_value_isize(gd)
    }

    pub fn get_usize_at(&self, gd: &GameData, index: usize) -> usize {
        self.ints[index].get_value_usize(gd)
    }

    pub fn get_at(&self, index: usize) -> GInt {
        self.ints[index].clone()
    }

    pub fn get_min(&self, gd: &GameData) -> isize {
        let ints = self.eval_ints(gd);
        *ints.iter().min().expect(&format!("No Minimum found in {}!", self.str_repr))
    }

    pub fn get_max(&self, gd: &GameData) -> isize {
        let ints = self.eval_ints(gd);
        *ints.iter().max().expect(&format!("No Minimum found in {}!", self.str_repr))
    }

    pub fn get_sum(&self, gd: &GameData) -> isize {
        let ints = self.eval_ints(gd);
        ints.iter().sum::<isize>()
    }    
}
