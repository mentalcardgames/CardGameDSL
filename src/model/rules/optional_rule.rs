use crate::model::rules::rule::Rule;
use crate::model::card_game_model::CardGameModel;
use crate::model::enums::rule_input::RuleInput;
use crate::model::enums::game_flow_change::GameFlowChange;

#[derive(Debug, Clone)]
pub struct OptionalRule {
    pub rules: Vec<Rule>,
    pub str_repr: String,
}
impl OptionalRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match input {
            RuleInput::DoOp => {
                let mut gfs = vec![];
                for i in 0..self.rules.len() {
                    let actype= self.rules[i].get_action_type();
                    let rulein = cgm.get_input(actype);
                    gfs = vec![gfs, self.rules[i].run(cgm, rulein).clone()].concat();
                }

                gfs
            },
            _ => {
                vec![GameFlowChange::None]
            },
        }
    }
}
