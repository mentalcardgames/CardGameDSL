use crate::model::action::move_action::{MoveAction};
use crate::model::action::deal_action::{DealAction};
use crate::model::action::move_card_set_action::{MoveCSAction};
use crate::model::action::cycle_action::{CycleAction};
use crate::model::action::shuffle_action::{ShuffleAction};
use crate::model::action::out_action::{OutAction};


#[derive(Clone)]
pub enum Action {
    Move(MoveAction),
    Deal(DealAction),
    MoveCardSet(MoveCSAction),
    CycleAction(CycleAction),
    EndTurn,
    EndStage,
    EndPlay,
    EndGame,
    ShuffleAction(ShuffleAction),
    OutAction(OutAction),
}
impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move(_) => f.write_str("Action::Move(<closure>)"),
            Action::Deal(_) => f.write_str("Action::Deal(<closure>)"),
            Action::MoveCardSet(_) => f.write_str("Action::MoveCardSet(<closure>)"),
            _ => f.write_str("Action::EndAction(<closure>)"),
        }
    }
}
