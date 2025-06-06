use crate::model::action::move_action::{MoveAction};
use crate::model::action::deal_action::{DealAction};
use crate::model::action::move_card_set_action::{MoveCSAction};
use crate::model::action::cycle_action::{CycleAction};
use crate::model::action::shuffle_action::{ShuffleAction};
use crate::model::action::out_action::{OutAction};

use crate::model::card_game_model::{CardGameModel};
use crate::model::enums::play_output::{PlayOutput};
use crate::model::enums::game_flow_change::{GameFlowChange};
use crate::model::enums::rule_input::{RuleInput};



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
impl Action {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match self {
            Self::Move(mv) => mv.run(cgm, input),
            Self::Deal(deal) => deal.run(cgm, input),
            Self::MoveCardSet(mvcs) => mvcs.run(cgm, input),
            _ => {
                vec![GameFlowChange::None]
            }
        }
    }

    fn play<'a>(&self, cgm: &'a mut CardGameModel) -> PlayOutput<'a> {
        match self {
            Self::Move(mv) => {mv.play(cgm)},
            Self::Deal(deal) => {deal.play(cgm)},
            Self::MoveCardSet(mvcs) => {mvcs.play(cgm)},
            _ => {PlayOutput::EndAction},
        }
    }
}
