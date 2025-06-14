use crate::model::function_types::{TMoveCards};
use crate::model::card_game_model::{CardGameModel};
use crate::model::enums::game_flow_change::{GameFlowChange};
use crate::model::enums::play_output::{PlayOutput};
use crate::model::enums::rule_input::{RuleInput};

#[derive(Clone)]
pub struct MoveAction {
    pub action: TMoveCards,
}

impl MoveAction {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> GameFlowChange {
        match input {
            RuleInput::MoveInput(mv) => {
                ((self.action)(cgm))(mv);
            },
            // TODO: error handling here
            _ => {}
        }

        GameFlowChange::None
    }

    pub fn play<'a>(&self, cgm: &'a mut CardGameModel) -> PlayOutput<'a> {
        PlayOutput::Move((self.action)(cgm))
    }
}