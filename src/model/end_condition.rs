use std::sync::Arc;
use std::fmt;
use crate::model::card_game_model::CardGameModel;


pub struct EndCondition {
    pub condition: Arc<dyn Fn(&CardGameModel, usize) -> bool>,
    pub str_repr: String,
}
impl EndCondition {
    pub fn evaluate(&self, cgm: &CardGameModel, reps: usize) -> bool {
        (*self.condition)(cgm, reps)
    }
}
impl Clone for EndCondition {
    fn clone(&self) -> Self {
        EndCondition {
            condition: Arc::clone(&self.condition),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl fmt::Debug for EndCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<Condition>")
    }
}
