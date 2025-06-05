use crate::model::action::action::{Action};

#[derive(Debug, Clone)]
pub struct ActionRule {
    pub action: Action,
    pub str_repr: String,
}
