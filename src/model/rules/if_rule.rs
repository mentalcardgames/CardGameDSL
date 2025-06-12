use crate::model::rules::condition::Condition;
use crate::model::rules::rule::Rule;
use crate::model::card_game_model::CardGameModel;
use crate::model::enums::rule_input::RuleInput;
use crate::model::enums::game_flow_change::GameFlowChange;


#[derive(Debug, Clone)]
pub struct IfRule {
    pub condition: Condition,
    pub rules: Vec<Rule>,
    pub str_repr: String,
}
impl IfRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, _: RuleInput) -> GameFlowChange {
        if self.condition.evaluate(cgm) {
            for i in 0..self.rules.len() { 
                let gfc = self.rules[i].run(cgm);
                if gfc != GameFlowChange::None {
                    return gfc;
                }
            }
        }
        
        GameFlowChange::None
    }
}
