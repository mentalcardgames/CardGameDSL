use crate::model::filter::{Filter};
use std::fmt;


pub struct CardCombination {
    pub name: String,
    pub attributes: Filter,
}

// Manual Debug implementation for CardCombination
impl fmt::Debug for CardCombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CardCombination {{ name: {:?}, attributes:  functions }}",
            self.name,
            // self.attributes
        )
    }
}
impl Clone for CardCombination {
    fn clone(&self) -> Self {
        CardCombination { 
            name: self.name.clone(),
            attributes: self.attributes.clone(),
        }
    }
}
