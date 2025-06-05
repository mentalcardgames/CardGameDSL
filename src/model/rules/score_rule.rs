use crate::model::base_types::g_int::GInt;
use crate::model::base_types::ref_player::RefPlayer;
use crate::model::card_game_model::CardGameModel;


pub struct ScoreRule {
    pub set: bool,
    pub score: GInt,
    pub pref: RefPlayer,
    pub str_repr: String,
}
impl std::fmt::Debug for ScoreRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ScoreRule(<closure>)")
    }
}
impl Clone for ScoreRule {
    fn clone(&self) -> Self {
        ScoreRule {
            set: self.set.clone(),
            score: self.score.clone(),
            pref: self.pref.clone(),
            str_repr: self.str_repr.clone(),
        }
    }
}
impl ScoreRule {
    pub fn run(&self, cgm: &mut CardGameModel) {
        let score = (self.score).get_value_i32(&cgm.gamedata);
        let name = (self.pref).get_ref(&cgm.gamedata).name;

        let player = cgm.gamedata.get_mut_player(&name);
        if self.set {
            player.score = score;
        } else {
            player.score += score;
        }
    }
}
