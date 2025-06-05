use crate::model::base_types::ref_player::{RefPlayer};
use crate::model::card_game_model::{CardGameModel};



pub struct CycleAction {
    pub pref: RefPlayer,
}
impl CycleAction {
    pub fn get_name(&self, cgm: &CardGameModel) -> String  {
        ((self.pref).get_ref(&cgm.gamedata)).name.clone()
    }

    pub fn get_pos(&self, cgm: &CardGameModel) -> usize {
        let pname = self.get_name(cgm);
        for i in 0..cgm.gamedata.turnorder.len() {
            if cgm.gamedata.turnorder[i] == pname {
                return i;
            }
        }

        // TODO:
        // Default return
        0
    }
}
impl Clone for CycleAction {
    fn clone(&self) -> Self {
        CycleAction {
            pref: self.pref.clone(),
        }
    }
}
impl std::fmt::Debug for CycleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action::CycleAction(<closure>)")
    }
}