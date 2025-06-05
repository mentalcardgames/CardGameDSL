use crate::model::rules::score_rule::ScoreRule;
use crate::model::rules::winner_rule::WinnerRule;
use crate::model::card_game_model::CardGameModel;
use crate::model::enums::rule_input::RuleInput;
use crate::model::enums::game_flow_change::GameFlowChange;


#[derive(Debug, Clone)]
pub enum ScoringRule {
    Score(ScoreRule),
    Winner(WinnerRule)
}
impl ScoringRule {
    pub fn run(&self, cgm: &mut CardGameModel, _: RuleInput) -> Vec<GameFlowChange> {
        match self {
            Self::Score(s) => {
                s.run(cgm);
            },
            Self::Winner(w) => {
                w.run(cgm);
            },
        }

        return vec![GameFlowChange::None];
    }
}
