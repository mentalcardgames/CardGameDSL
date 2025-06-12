use crate::model::base_types::ref_player::{RefPlayer};
use crate::model::card_game_model::{CardGameModel};
use crate::model::enums::game_flow_change::{GameFlowChange};
use crate::model::enums::out_of::{OutOf};

// Just for Player for now 
// TODO:
// Do it for Team
pub struct OutAction {
    pub pref: RefPlayer,
    pub outof: OutOf,
}
impl Clone for OutAction {
    fn clone(&self) -> Self {
        OutAction {
            pref: self.pref.clone(),
            outof: self.outof.clone(),
        }
    }
}
impl std::fmt::Debug for OutAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action::OutAction(<closure>)")
    }
}
impl OutAction {
    pub fn run(&self, cgm: &CardGameModel) -> GameFlowChange {
        let pname = vec![(self.pref).get_ref(&cgm.gamedata).name];

        match self.outof {
            OutOf::Stage => {
                GameFlowChange::OutOfStage(pname)
            },
            OutOf::Play => {
                GameFlowChange::OutOfPlay(pname)
            },
            OutOf::GameSuccessful => {
                GameFlowChange::OutOfGameSuccessful(pname)
            },
            OutOf::GameFail => {
                GameFlowChange::OutOfGameFail(pname)
            },
        }
    } 
}
