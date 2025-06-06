use crate::model::action::cycle_action::{CycleAction};

#[derive(Debug, Clone)]
pub enum GameFlowChange {
    None,
    // for current Player
    EndTurn,
    EndStage,
    EndPlay,
    EndGame,
    // for ANY Player and Players
    // TODO:
    OutOfStage(Vec<String>),
    OutOfPlay(Vec<String>),
    OutOfGameSuccessful(Vec<String>),
    OutOfGameFail(Vec<String>),
    // cycle to someone else
    CycleTo(CycleAction),
}

impl PartialEq for GameFlowChange {
    fn eq(&self, other: &Self) -> bool {
        use GameFlowChange::*;

        match (self, other) {
            (None, None)
            | (EndTurn, EndTurn)
            | (EndStage, EndStage)
            | (EndPlay, EndPlay)
            | (EndGame, EndGame) => true,

            (OutOfStage(a), OutOfStage(b))
            | (OutOfPlay(a), OutOfPlay(b))
            | (OutOfGameSuccessful(a), OutOfGameSuccessful(b)) => a == b,
            | (OutOfGameFail(a), OutOfGameFail(b)) => a == b,


            (CycleTo(_), CycleTo(_)) => true, // ignore the function, just match variant

            _ => false,
        }
    }
}
impl Eq for GameFlowChange {}
