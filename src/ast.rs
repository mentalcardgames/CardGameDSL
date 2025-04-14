use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;


#[derive(Debug, Clone)]
pub struct CardGameModel {
    pub name: String,
    pub gamedata: GameData,
    pub ruleset: RuleSet,
}
impl CardGameModel {
    pub fn new(name: &str) -> CardGameModel {
        CardGameModel {
            name: name.to_string(),
            gamedata: GameData::default(),
            ruleset: RuleSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub table: Table,
    pub teams: HashMap<String, Team>,
    pub players: HashMap<String, Player>,
    pub playertoteam: HashMap<String, String>,
    pub turnorder: Vec<String>,
    pub precedences: HashMap<String, Precedence>,
    pub pointmaps: HashMap<String, PointMap>,
    pub cardcombinations: HashMap<String, CardCombination>,
}
impl Default for GameData {
    fn default() -> Self {
        GameData { table: Table { locations: HashMap::new() },
                    teams: HashMap::new(),
                    players: HashMap::new(),
                    playertoteam: HashMap::new(),
                    turnorder: vec![],
                    precedences: HashMap::new(),
                    pointmaps: HashMap::new(),
                    cardcombinations: HashMap::new(),
                }
    }
}
impl GameData {
    pub fn add_player(&mut self, name: String) {
        self.players.insert(name.clone(), Player::new(name));
    }

    pub fn add_players(&mut self, names: Vec<String>) {
        for name in names {
            self.players.insert(name.clone(), Player::new(name));

        }
    }

    pub fn lookup_player(&mut self, name: &str) -> Option<&mut Player> {
        // Find all players that match the name
        self.players.get_mut(name)
    }

    pub fn add_loc_player(&mut self, locname: String, playername: String) {
        match self.players.get_mut(&playername) { // Use find_player_mut to get a mutable reference
            Some(p) => {
                p.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname)))); // Modify player
            }
            None => {
                println!("Error: player not found!");
            }
        }
    }

    fn get_mut_team(&mut self, name: &str) -> Option<&mut Team> {
        self.teams.get_mut(name)
    }    

    pub fn add_loc_team(&mut self, locname: String, teamname: String) {
        match self.get_mut_team(&teamname) { // Use find_player_mut to get a mutable reference
            Some(t) => {
                t.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname)))); // Modify player
            }
            None => {
                println!("Error: team not found!");
            }
        }
    }

    pub fn add_loc_table(&mut self, locname: String) {
        self.table.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname))));
    }

    pub fn get_mut_locs(&mut self, locname: &str) -> Vec<&mut Rc<RefCell<Location>>> {
        let mut locs: Vec<&mut Rc<RefCell<Location>>> = vec![];
    
        // Check self.table
        if let Some(l) = self.table.locations.get_mut(locname) {
            locs.push(l);
        } else {
            println!("No location in table!");
        }
    
        // Iterate over self.players and collect matching locations
        for (_, v) in self.players.iter_mut() {
            if let Some(loc) = v.locations.get_mut(locname) {
                locs.push(loc);
            }
        }
    
        // Iterate over self.teams and collect matching locations
        for (_, v) in self.teams.iter_mut() {
            if let Some(loc) = v.locations.get_mut(locname) {
                locs.push(loc);
            }
        }
    
        locs
    }

    // Problem:
    // We can't find "all" Locations.
    // For example:
    // We have a Game like Uno where you might want to know
    // how many cards each player holds in their hand.
    // We can't know that.
    // Another example:
    // "Move your cards to P1's Location"
    // We can't do this because we do not have anything
    // that tells us that this Location exists!
    //
    // Solution1:
    // If you want to directly lookup a Location then do it lke this:
    // Player: P1's Location: P1_Location
    // Team:   T1's Location: T1_Location
    // (Table: Table Location:   Location)
    //
    // Solution2:
    // In a normal Card Game you would just ASK the Player how many Cards they have.
    // We could also do the same and add an "Information-Request"-Action.
    // This would be the best Solution, because you would still safe only your Information!
    // However, you also need to construct the Game-UI and with limited Information.
    //
    // => We will do Solution1 for now.
    pub fn get_mut_loc_name(&mut self, loc_name: String, pname: String) -> Option<&Rc<RefCell<Location>>> {
        let parts: Vec<&str> = loc_name.split('_').collect();

        if parts.len() > 2 {
            println!("The Name of the Location does NOT exist!");
        }

        if parts.len() == 1 {
            // Check own Player/Team-Locations and Table Locations
            let locname = parts[0];

            let loc = self.players.get(&pname).unwrap()
                .locations.get(locname);
            match loc {
                None => println!("The Name of the Location does NOT exist in Player!"),
                Some(l) => return Some(l)
            }

            let loc = self.table
                .locations.get(locname);
            match loc {
                None => println!("The Name of the Location does NOT exist in Player!"),
                Some(l) => return Some(l)
            }

            let tname = self.playertoteam.get(&pname);
            match tname {
                None => println!("Player is NOT in a Team!"),
                Some(t) => {
                    let loc = self.teams.get(t).unwrap()
                        .locations.get(locname);
                    match loc {
                        None => println!("No Location in Players!"),
                        Some(l) => return Some(l)
                    }
                }
            }
        }

        // if parts.len() == 2
        else {
            let name = parts[0];
            let locname = parts[1];

            // Check players
            let player = self.players.get(name);
            match player {
                None => println!("Player NOT found!"),
                Some(p) => return self
                    .get_mut_loc_name(locname.to_string(), p.name.clone())
            }

            // Check teams
            let team = self.teams.get(name);
            match team {
                None => println!("Player NOT found!"),
                Some(t) => return self
                    .get_mut_loc_name(locname.to_string(), t.teamname.clone())
            }
        }

        return None;
    }

    fn remove_player(&mut self, player_name: String) {
        // for i in 0.. self.players.len() {
        //     if self.players[i].name == player_name {
        //         self.players.remove(i);
        //     }
        //     // Check where the player is referenced elsewhere
        //     // remove from team
        //     // remove from pointmap etc...
        // }
    }

    pub fn add_team(&mut self, name: String, players: Vec<String>) {
        self.teams.insert(name.clone(), Team::new(name.clone(), players.clone()));

        for p in players {
            self.playertoteam.insert(p, name.clone());
        }
    }

    pub fn add_precedence(&mut self, precedence: Precedence) {
        self.precedences.insert(precedence.name.clone(),precedence);
    }

    pub fn add_pointmap(&mut self, pointmap: PointMap) {
        self.pointmaps.insert(pointmap.name.clone(), pointmap);
    }

    pub fn set_turnorder(&mut self, playernames: Vec<String>) {
        self.turnorder = playernames;
    }

    pub fn add_cardcombination(&mut self, name: String, cardcomb: CardCombination) {
        self.cardcombinations.insert(name, cardcomb);
    }

    // TODO:
    // has to be overworked later !
    pub fn apply_combo(&mut self, comboname: String, locname: String) -> Vec<Vec<Card>> {
        // UNWRAP USED!!!
        let loc = (*self.get_mut_locs(&locname)[0]).clone();
        self.cardcombinations
            .get(&comboname)
            .unwrap()
            .attributes
            .deref()(loc
                .clone()
                .borrow()
                .contents
                .iter()
                .filter_map(|c| c.clone().to_card())
                .collect())
    }

    pub fn move_card(&mut self, loc1: Rc<RefCell<Location>>, loc2: Rc<RefCell<Location>>, index: usize) {
        loc2.borrow_mut().add_component(loc1.borrow_mut().remove_card(index));
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
    // Location needs to be a Rc<RefCell<Location>>
    // Because it needs to be mut borrowed with other Locations
    // And because they are all in a Model that means they need to be
    // Rc<RefCell<...>
    pub locations: HashMap<String, Rc<RefCell<Location>>>,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.name.clone();
        let locs: Vec<Rc<RefCell<Location>>> = self.locations.values().cloned().collect();
        
        // Print the name first
        write!(f, "Player-name: {}", s)?;
        
        // Print each location
        for l in locs {
            write!(f, " Location: {}", l.borrow())?; 
        }

        // Print the score at the end
        write!(f, " score: {}", self.score)
    }
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
        self.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname))));
    }

    pub fn show_locations(&mut self) {
        if self.locations.is_empty() {
            println!("No Locations!")
        }
        for (k, _) in self.locations.iter() {
            println!("Player {}: locname={}", self.name, k.to_string())
        }
    }

    pub fn get_location(&mut self, locname: &str) -> Option<&Rc<RefCell<Location>>> {
        self.locations.get(locname)
    }
}

#[derive(Debug, Clone)]
pub struct Team {
    pub teamname: String,
    pub players: Vec<String>,
    pub locations: HashMap<String, Rc<RefCell<Location>>>,
}
impl Team {
    pub fn new(name: String, players: Vec<String>) -> Team {
        Team {
            teamname: name,
            players: players,
            locations: HashMap::new()
        }
    }

    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname))));
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
    pub locations: HashMap<String, Rc<RefCell<Location>>>,
}
impl Table {
    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname))));
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

    pub fn get_cards(self) -> Vec<Card> {
        self.contents
            .iter()
            .filter_map(|c| c.clone().to_card())
            .collect()
    }

    pub fn add_card(&mut self, card: Card) {
        self.contents.push(Component::CARD(card));
    }

    pub fn add_component(&mut self, comp: Component) {
        self.contents.push(comp);
    }

    pub fn remove_card(&mut self, index: usize) -> Component {
        self.contents.remove(index)
    }
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.name.clone();
        write!(f, "{}\n", s)?;
        write!(f, "content-len: {}", self.contents.len())
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

impl Component {
    pub fn to_card(self) -> Option<Card> {
        match self {
            Component::CARD(card) => Some(card), // Properly destructure `Component::CARD`
            _ => None, // Return `None` if it's not a `CARD`
        }
    }
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

    pub fn change_status(&mut self, status: Status) {
        self.status = status;
        // HERE HAS TO HAPPEN THE ENCRYPTION OF THE CARD!
        //
        //
        //
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
    pub attributes: HashMap<String, usize>,
}

// Wrapper for function to avoid Debug issue
pub struct CardFunction(Rc<dyn Fn(Vec<Card>) -> Vec<Vec<Card>>>);

impl CardFunction {
    pub fn new(fun: Rc<dyn Fn(Vec<Card>) -> Vec<Vec<Card>>>) -> Self {
        Self(fun)
    }
}

impl Deref for CardFunction {
    type Target = dyn Fn(Vec<Card>) -> Vec<Vec<Card>>;

    fn deref(&self) -> &Self::Target {
        &*self.0 // Dereferences Rc/Arc to get the function
    }
}

impl Clone for CardFunction {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

#[derive(Clone)]
pub struct CardCombination {
    pub name: String,
    // in the thesis there is attributes: HashMap<String, Filter>
    // Which i dont get at all
    // Why does ONE CardCombination have multiple CardCombinations???
    pub attributes: CardFunction,
}

impl CardCombination {
    // // Constructor
    // pub fn new(name: &str) -> Self {
    //     Self {
    //         name: name.to_string(),
    //         attributes: HashMap::new(),
    //     }
    // }

    // // Method to add an attribute function
    // pub fn add_attribute<F>(&mut self, key: &str, func: F)
    // where
    //     F: Fn(Vec<Card>) -> Vec<Vec<Card>> + Send + Sync + 'static,
    // {
    //     self.attributes.insert(key.to_string(), CardFunction(Arc::new(func)));
    // }

    // // Method to apply an attribute function
    // pub fn apply_attribute(&self, key: &str, cards: Vec<Card>) -> Option<Vec<Vec<Card>>> {
    //     self.attributes.get(key).map(|func| (func.0)(cards))
    // }
}

// Manual Debug implementation for CardCombination
impl fmt::Debug for CardCombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CardCombination {{ name: {:?}, attributes:  functions }}",
            self.name,
            // self.attributes
        )
    }
}

#[derive(Debug, Clone)]
pub struct PointMap {
    pub name: String,
    pub entries: HashMap<String, Vec<i32>>,
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
    // maybe Box::new(dyn Fn(...))?
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

impl Play {
    pub fn add_endcondition(&mut self, end_cond: Condition) {
        self.endconditions.push(end_cond);
    }

    pub fn add_stage(&mut self, stage: Stage) {
        self.stages.push(stage);
    }

}
