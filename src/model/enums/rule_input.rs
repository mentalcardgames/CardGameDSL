use crate::model::location::location_ref::{LocationRef};

#[derive(Debug, Clone)]
pub enum RuleInput {
    None,
    DoOp,
    NoOp,
    Trigger,
    ChooseInput(usize),
    MoveCardSet,
    MoveInput(Vec<((LocationRef, usize), (LocationRef, usize))>),
}