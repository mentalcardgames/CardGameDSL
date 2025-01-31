use core::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;


#[derive(Debug, Clone)]
pub struct CardGameModel {
    pub name: String,
    pub gamedata: GameData,
    pub ruleset: RuleSet,
}
impl CardGameModel {
    pub fn new(name: String) -> CardGameModel {
        CardGameModel {
            name: name,
            gamedata: GameData::default(),
            ruleset: RuleSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub table: Table,
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub turnorder: Vec<String>,
    // precedences should be a HashMap<String (Key-Name), HashMap<...> (precedence hashmap)>
    // then we can just call precedence!("key-name", "same" using precedence).
    // Because we can just look up the key-name.
    // We could maybe leave out the "using precedence" (but thats a "fine-tuning" question).
    pub precedences: Vec<Precedence>,
    pub pointmaps: Vec<PointMap>,
}
impl Default for GameData {
    fn default() -> Self {
        GameData { table: Table { locations: HashMap::new() },
                    teams: vec![],
                    players: vec![],
                    turnorder: vec![],
                    precedences: vec![],
                    pointmaps: vec![] }
    }
}
impl GameData {
    fn add_player(&mut self, name: String) {
        self.players.push(Player::new(name));
    }

    fn remove_player(&mut self, player_name: String) {
        for i in 0.. self.players.len() {
            if self.players[i].name == player_name {
                self.players.remove(i);
            }
            // Check where the player is referenced elsewhere
            // remove from team
            // remove from pointmap etc...
        }
    }

    fn add_team(&mut self, name: String, players: Vec<Rc<RefCell<Player>>>) {
        // TODO: locations
        self.teams.push(Team::new(name, players));
    }

    fn add_precedence(&mut self, precedence: Precedence) {
        self.precedences.push(precedence);
    }

    fn add_pointmap(&mut self, pointmap: PointMap) {
        self.pointmaps.push(pointmap);
    }


    
}

#[derive(Debug, Clone)]
pub enum Status {
    FACEUP,
    FACEDOWN,
    PRIVATE,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub locations: HashMap<String, Location>,
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            score: 0,
            locations: HashMap::new()
        }
    }

    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Location::new(locname));
    }

    pub fn show_locations(&mut self) {
        if self.locations.is_empty() {
            println!("No Locations!")
        }
        for (k, _) in self.locations.iter() {
            println!("Player {}: locname={}", self.name, k.to_string())
        }
    }

}

#[derive(Debug, Clone)]
pub struct Team {
    pub teamname: String,
    pub players: Vec<Rc<RefCell<Player>>>,
    pub locations: HashMap<String, Location>,
}
impl Team {
    pub fn new(name: String, players: Vec<Rc<RefCell<Player>>>) -> Team {
        Team {
            teamname: name,
            players: players,
            locations: HashMap::new()
        }
    }

    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Location::new(locname));
    }

    pub fn show_locations(&mut self) {
        if self.locations.is_empty() {
            println!("No Locations!")
        }
        for (k, v) in self.locations.iter() {
            println!("Player {}: locname={}", self.teamname, k.to_string())
        }
    }
}


#[derive(Debug, Clone)]
pub struct Table {
    pub locations: HashMap<String, Location>,
}
impl Table {
    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Location::new(locname));
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    //    AREA(Area),
    //    PILE(Pile),
    pub name: String,
    pub contents: Vec<Component>
}
impl Location {
    pub fn new(locname: String) -> Location {
        Location { name: locname, contents: vec![]}
    }
}

#[derive(Debug, Clone)]
pub struct Area {
    pub name: String,
    pub contents: Vec<Component>
}

#[derive(Debug, Clone)]
pub struct Pile {
    pub name: String,
    pub contents: Vec<Component>
}

#[derive(Debug, Clone)]
pub enum Component {
    CARD(Card),
    TOKEN,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub status: Status,
    pub attributes: HashMap<String, String>,
}
impl Card {
    pub fn new(attributes: HashMap<String, String>) -> Card {
        Card {
            status: Status::FACEUP,
            attributes: attributes,
        }
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement how you want to format the output
        let mut card = "Card:\n".to_string();
        for key in self.attributes.keys() {
            card = card + key + ": [";
            for value in self.attributes.get(key).iter() {
                card = card + value;
            }
            card = card + &"]\n";
        }
        write!(f, "{}", card)
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if self.attributes.len() != other.attributes.len() {
            return false;
        }
        for kv in &self.attributes {
            if *kv.1 != other.attributes[kv.0] {
                return false;
            }
        }
        return true;
    }
}
impl Eq for Card {}

#[derive(Debug, Clone)]
pub struct Precedence {
    pub name: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct CardCombination {
    pub name: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PointMap {
    pub name: String,
    pub entries: HashMap<String, i32>,
}

#[derive(Debug, Clone)]
pub enum Stage {
    SIM(StageS),
    SEQ(StageS),
}

#[derive(Debug, Clone)]
pub struct StageS {
    pub name: String,
    pub endconditions: Vec<Condition>,
    pub substages: Vec<Stage>,
    pub turncounter: i32,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Condition {
    // maybe Box::new(dyn Fn(...))
}

#[derive(Debug, Clone)]
pub struct ConditionalCase {
    pub conditions: Vec<Condition>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct RuleSet {
    pub setup: Setup,
    pub play: Play,
    pub scoring: Scoring,
}
impl RuleSet {
    pub fn new() -> RuleSet {
        RuleSet {
            setup: Setup {setuprules: vec![]},
            play: Play { endconditions: vec![], stages: vec![]},
            scoring: Scoring {scoringrules: vec![]}
        }
    }
}

#[derive(Debug, Clone)]
pub enum Rule {
    SETUPRULE,
    SCORINGRULE,
    PLAYRULE,
}

#[derive(Debug, Clone)]
pub enum PlayRule {
    CONDITIONALRULE(Vec<ConditionalCase>),
    ACTIONRULE(),
    OPTIONALRULE(Vec<Rule>),
    CHOOSERULE(Vec<Rule>),
}

#[derive(Debug, Clone)]
pub struct Setup {
    // SetupRules
    pub setuprules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Scoring {
    // ScoringRules
    pub scoringrules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Play {
    pub endconditions: Vec<Condition>,
    pub stages: Vec<Stage>,
}


