use core::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;


#[derive(Debug, Clone)]
pub struct GameData {
    pub table: Table,
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub turnorder: Vec<String>,
    pub precedences: Vec<Precedence>,
    pub pointmaps: Vec<PointMap>,
}
impl Default for GameData {
    fn default() -> Self {
        GameData { table: Table { locations: vec![] },
                    teams: vec![],
                    players: vec![],
                    turnorder: vec![],
                    precedences: vec![],
                    pointmaps: vec![] }
    }
}
impl GameData {
    fn add_player(&mut self, name: String, score: i32, locations: Vec<Rc<Location>>) {
        self.players.push(Player { name: name, score: score, locations: locations });
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

    fn add_team(&mut self, name: String, players: Vec<Rc<Player>>, locations: Vec<Rc<Location>>) {
        // TODO: locations
        self.teams.push(Team {teamname: name, players: players, locations: locations});
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
    pub locations: Vec<Rc<Location>>,
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            score: 0,
            locations: vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub struct Team {
    pub teamname: String,
    pub players: Vec<Rc<Player>>,
    pub locations: Vec<Rc<Location>>,
}
impl Team {
    pub fn new(name: String, players: Vec<Rc<Player>>) -> Team {
        Team {
            teamname: name,
            players: players,
            locations: vec![]
        }
    }
}


#[derive(Debug, Clone)]
pub struct Table {
    pub locations: Vec<Rc<Location>>,
}

#[derive(Debug, Clone)]
pub struct Location {
    //    AREA(Area),
    //    PILE(Pile),
    pub name: String,
    pub contents: Vec<Component>
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


#[derive(Debug, Clone)]
pub struct Precedence {
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


