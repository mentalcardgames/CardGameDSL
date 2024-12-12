use std::vec;
use rand::thread_rng;
use rand::seq::SliceRandom;
use itertools::Itertools;

/*
TODO:
- write a update(...) function that updates each corresponding value
Ex.:
Team { ... Player: {player: jimmy, locations: []}}
player jimmy gets locations [a, b, c]
update(...)
Team { ... Player: {player: jimmy, locations: [a, b, c]}}
*/


#[derive(Debug, Clone)]
pub struct GameData {
    pub table: Table,
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    // String => PlayerName => Find corresponding Player
    pub turnorder: Vec<String>,
    pub precedences: Vec<Precedence>,
    pub pointmaps: Vec<PointMap>,
}
impl Default for GameData {
    fn default() -> Self {
        GameData { table: Table { locations: vec![], remembrances: vec![] },
                    teams: vec![],
                    players: vec![],
                    turnorder: vec![],
                    precedences: vec![],
                    pointmaps: vec![] }
    }
}
impl GameData {
    pub fn set_players(&mut self, player_names: Vec<String>) {
        let mut players: Vec<Player> = vec![];
        for name in player_names.iter() {
            players.push(Player { name: name.to_string(), score: 0, locations: vec![], remembrances: vec![]});
        }
        self.players = players;
    }

    pub fn set_teams(&mut self, parsed_teams: Vec<(String, Vec<Player>)>) {
        let mut teams: Vec<Team> = vec![];
        for name_players in parsed_teams.iter() {
            teams.push(Team { teamname: (name_players.0).to_string(),
                              players: (name_players.1).clone(),
                              locations: vec![],
                              remembrances: vec![]}
            );
        }
        self.teams = teams;
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn add_team(&mut self, team: Team) {
        self.teams.push(team);
    }

    pub fn find_player_name(&mut self, name: String) -> Player {
        for player in self.players.iter() {
            if player.name == name {
                return player.clone();
            }
        }
        return Player { name: "ERROR".to_string(), score: 0, locations: vec![], remembrances: vec![] };
    }

    pub fn find_team_name(&mut self, name: String) -> Team {
        for team in self.teams.iter() {
            if team.teamname == name {
                return team.clone();
            }
        }
        return Team { teamname: "ERROR".to_string(), players: vec![], locations: vec![], remembrances: vec![] };
    }

    pub fn add_locations_players(&mut self, locs: Vec<Location>, players: Vec<Player>) {
        for i in 0..players.len() {
            for j in 0..self.players.len() {       
                if self.players[j].name == players[i].name {
                    self.players[i].locations = locs.clone();
                }
            }
        }
    }

    pub fn set_player_locations(&mut self, loc_names: Vec<String>, players: Vec<Player>) {
        let mut locations = vec![];
        for loc in loc_names.iter() {
            locations.push( Location { name: loc.to_string(), contents: vec![] } );
        }
        for player in players.iter() {
            for i in 0..self.players.len() {
                if self.players[i].name == player.name {
                    self.players[i].locations = locations.clone();
                } 
            }
        }
    }

    pub fn set_team_locations(&mut self, loc_names: Vec<String>, teams: Vec<Team>) {
        let mut locations = vec![];
        for loc in loc_names.iter() {
            locations.push( Location { name: loc.to_string(), contents: vec![] } );
        }
        for team in teams.iter() {
            for i in 0..self.teams.len() {
                if self.teams[i].teamname == team.teamname {
                    self.teams[i].locations = locations.clone();
                } 
            }
        }
    }

    pub fn set_table_locations(&mut self, loc_names: Vec<String>) {
        let mut locations = vec![];
        for loc in loc_names.iter() {
            locations.push( Location { name: loc.to_string(), contents: vec![] } );
        }
        self.table.locations = locations;
    }

    pub fn set_turnorder(&mut self, players: Vec<Player>, random: bool) {
        let mut turnorder = vec![];
        for player in players.iter() {
            turnorder.push(player.name.clone());
        }
        if random {
            // do random permutation
            turnorder.shuffle(&mut thread_rng());
        }
        self.turnorder = turnorder;
    }
    
    pub fn create_cards(&mut self, loc: String, key: String, values: Vec<String>) {
        // loc is still the name of the location
        // -------------------------------------
        // loc should be on Table, because the rest should be dealt to player locations
        // and we do not have references to locations from players
        // so we can not do "Player.hand" and also cannot just say "hand" because which "hand" do we mean??
        let mut contents = vec![];
        for value in values.iter() {
            contents.push(
                Component::CARD(Card { status: Status::PRIVATE,
                       attributes: vec![Attribute { key: key.clone(), value: value.clone() }] }))
        }

        for i in 0..self.table.locations.len() {
            if self.table.locations[i].name == loc {
                self.table.locations[i].contents = contents.clone();
            }
        }
    }
    
    pub fn create_cards_for_key_value(&mut self, loc: String, key: String, values: Vec<String>, key_value_pairs: Vec<(String, Vec<String>)>) {
        // loc is still the name of the location
        // We have: for Suit (Spades, Clubs) for Color (Black) => Card(..., Spades, Black), Card(..., Clubs, Black)
        // Spades x Black, Clubs x Black
        // Extract just the Vec<Value> parts for Cartesian product
        let mut kvp: Vec<(String, Vec<String>)> = key_value_pairs.clone();
        let mut keys: Vec<String> = kvp.iter().map(|k| k.0.clone()).collect();
        kvp.push((key.clone(), values.clone()));
        keys.push(key.clone());

        let value_vectors: Vec<_> = kvp.iter().map(|(_, values)| values.iter()).collect();

        // Compute Cartesian product
        let cartesian_product = value_vectors
            .into_iter()
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        // Display the results
        let mut contents = vec![];
        for combination in cartesian_product {
            let mut attrs = vec![];
            for i in 0..combination.len() {
                attrs.push(Attribute { key: keys[i].clone(), value: combination[i].clone() });
            }
            contents.push(Component::CARD(Card { status: Status::PRIVATE, attributes: attrs.clone() }));
        }

        for i in 0..self.table.locations.len() {
            if self.table.locations[i].name == loc {
                self.table.locations[i].contents = contents.clone();
            }
        }
    }

    pub fn create_prec(&mut self, name: String, ref_key: String, ref_values: Vec<String>) {
        let attributes = ref_values
                                        .iter()
                                        .map(|x| Attribute { key: ref_key.clone(), value: x.clone()})
                                        .collect();
        let precedence = Precedence { name: name, attributes: attributes };
        self.precedences.push(precedence);
    }

    pub fn create_prec_kv(&mut self, name: String, ref_key_value_pair: Vec<(String, String)>) {
        let attributes = ref_key_value_pair
                                        .iter()
                                        .map(|x| Attribute { key: x.0.clone(), value: x.1.clone()})
                                        .collect();
        let precedence = Precedence { name: name, attributes: attributes };
        self.precedences.push(precedence);
    }

    pub fn create_pointmap(&mut self, name: String, ref_key: String, ref_val_int: Vec<(String, i32)>) {
        let entries: Vec<PointMapEntry> = ref_val_int
                                        .iter()
                                        .map(|x|
                                                PointMapEntry { mapentrykey: Attribute { 
                                                                                key: ref_key.clone(),
                                                                                value: x.0.clone()
                                                                },
                                                                mapentryvalue: x.1.clone() })
                                        .collect();
        self.pointmaps.push(PointMap { name: name, entries: entries });
    }

    pub fn create_pointmap_kvi(&mut self, name: String, kvi_pairs: Vec<(String, String, i32)>) {
        let entries: Vec<PointMapEntry> = kvi_pairs
                                        .iter()
                                        .map(|x|
                                                PointMapEntry { mapentrykey: Attribute { 
                                                                                key: x.0.clone(),
                                                                                value: x.1.clone()
                                                                },
                                                                mapentryvalue: x.2.clone() })
                                        .collect();
        self.pointmaps.push(PointMap { name: name, entries: entries });
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    FACEUP,
    FACEDOWN,
    PRIVATE,
}

#[derive(Debug, Clone)]
pub enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD
}

#[derive(Debug, Clone)]
pub enum Collection {
    IntCollection(Vec<i32>),
    StringCollection(Vec<String>),
    PlayerCollection(Vec<Player>),
    TeamCollection(Vec<Team>),
    // You can add other collections here...
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub locations: Vec<Location>,
    pub remembrances: Vec<Memory>,
}

#[derive(Debug, Clone)]
pub struct Team {
    pub teamname: String,
    pub players: Vec<Player>,
    pub locations: Vec<Location>,
    pub remembrances: Vec<Memory>,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub locations: Vec<Location>,
    pub remembrances: Vec<Memory>,
}


#[derive(Debug, Clone)]
pub enum Memory {
    INTEGERSTORAGE(IntegerStorage),
    ATTRIBUTESTORAGE(AttributeStorage),
}

#[derive(Debug, Clone)]
pub struct IntegerStorage {
    name: String,
    value: i32,
}

#[derive(Debug, Clone)]
pub struct AttributeStorage {
    name: String,
    attribute: String,
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
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Precedence {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct PointMap {
    pub name: String,
    pub entries: Vec<PointMapEntry>,
}

#[derive(Debug, Clone)]
pub struct PointMapEntry {
    pub mapentrykey: Attribute,
    pub mapentryvalue: i32,
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


