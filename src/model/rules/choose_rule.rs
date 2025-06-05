use crate::model::rules::rule::{Rule};
use crate::model::enums::rule_input::{RuleInput};
use crate::model::enums::game_flow_change::{GameFlowChange};
use crate::model::card_game_model::{CardGameModel};


#[derive(Debug, Clone)]
pub struct ChooseRule {
    pub rules: Vec<Rule>,
    pub str_repr: String,
}
impl ChooseRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match input {
            RuleInput::ChooseInput(i) => {
                let actype= self.rules[i].get_action_type();
                let input = cgm.get_input(actype);
                self.rules[i].run(cgm, input)
            },
            _ => {
                vec![GameFlowChange::None]
            },
        }
    }
}
