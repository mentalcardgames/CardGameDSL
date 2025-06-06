use std::collections::HashMap;

use crate::model::card::card::{Card};

#[derive(Debug, Clone)]
pub struct PointMap {
    pub name: String,
    pub entries: HashMap<String, Vec<i32>>,
}

impl PointMap {
    pub fn get_card_value_ref(&self, card: &Card) -> Option<Vec<i32>> {
        for (_, value) in &card.attributes {
            if let Some(score) = self.entries.get(value) {                
                return Some(score.clone());
            }
        }

        None
    }

    pub fn get_card_value(&self, card: Card) -> Option<Vec<i32>> {
        for (_, value) in card.attributes {
            if let Some(score) = self.entries.get(&value) {
                return Some(score.clone());
            }
        }

        println!("SOMETHING WENT WRONG!");

        None
    }   
}
