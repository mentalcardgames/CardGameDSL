use crate::model::rules::conditional_case::ConditionalCase;
use crate::model::card_game_model::CardGameModel;
use crate::model::enums::rule_input::RuleInput;
use crate::model::enums::game_flow_change::GameFlowChange;


#[derive(Debug, Clone)]
pub struct ConditionalRule {
    pub condcases: Vec<ConditionalCase>,
    pub str_repr: String,
}
impl ConditionalRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, _: RuleInput) -> GameFlowChange {
        for i in 0..self.condcases.len() {    
            if self.condcases[i].condition.evaluate(cgm) {
                for j in 0..self.condcases[i].rules.len() { 
                    let gfc = self.condcases[i].rules[j].run(cgm);
                    if gfc != GameFlowChange::None {
                        return gfc
                    }
                }
            }
        }

        // TODO: Error handling
        GameFlowChange::None
    }
}
