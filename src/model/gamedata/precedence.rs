use std::collections::HashMap;

use crate::model::card::card::{Card};

#[derive(Debug, Clone)]
pub struct Precedence {
    pub name: String,
    pub attributes: HashMap<String, usize>,
}

impl Default for Precedence {
    fn default() -> Self {
        Precedence { name: format!("default"), attributes: HashMap::new() }
    }
}

impl Precedence {
    pub fn get_card_value_ref(&self, card: &Card) -> Option<usize> {
        for (_, value) in &card.attributes {
            if let Some(score) = self.attributes.get(value) {                
                return Some(*score);
            }
        }

        None
    }

    pub fn get_card_value(&self, card: Card) -> Option<usize> {
        for (_, value) in card.attributes {
            if let Some(score) = self.attributes.get(&value) {
                return Some(*score);
            }
        }

        None
    }
}
