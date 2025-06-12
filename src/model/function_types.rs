use std::sync::Arc;
use std::collections::HashMap;

use crate::model::card_game_model::CardGameModel;
use crate::model::gamedata::game_data::GameData;
use crate::model::location::location_ref::LocationRef;
use crate::model::owners::player::Player;
use crate::model::owners::team::Team;
use crate::model::card::card::Card;

pub type TMoveCards    = Arc<dyn for<'a> Fn(&'a mut CardGameModel) -> Box<dyn FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a>>;
pub type TMoveCardSet  = Arc<dyn Fn(&mut CardGameModel) + Send + Sync + 'static>;
pub type TCardSet      = Arc<dyn Fn(&GameData) -> HashMap<LocationRef, Vec<Card>> + Send + Sync + 'static>;
pub type TRefPlayer    = Arc<dyn Fn(&GameData) -> Player>;
pub type TRefTeam      = Arc<dyn Fn(&GameData) -> Team>;
pub type TInt          = Arc<dyn Fn(&GameData) -> isize>;
pub type TBool         = Arc<dyn Fn(&CardGameModel) -> bool>;
pub type TString       = Arc<dyn Fn(&GameData) -> String>;
pub type TFilter       = Arc<dyn Fn(&GameData, Vec<Card>) -> Vec<Vec<Card>>>;
pub type TCardPosition = Arc<dyn Fn(&GameData) -> HashMap<LocationRef, Vec<Card>> + Send + Sync + 'static>;
