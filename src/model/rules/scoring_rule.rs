use crate::model::rules::score_rule::ScoreRule;
use crate::model::rules::winner_rule::WinnerRule;

#[derive(Debug, Clone)]
pub enum ScoringRule {
    Score(ScoreRule),
    Winner(WinnerRule)
}
