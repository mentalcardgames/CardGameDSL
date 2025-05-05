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
    // current playerindex
    pub current: usize
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
                    current: 0
                }
    }
}
impl GameData {
    pub fn add_player(&mut self, name: &str) {
        self.players.insert(String::from(name), Player::new(String::from(name)));
    }

    pub fn add_players(&mut self, names: Vec<&str>) {
        for name in names {
            self.players.insert(String::from(name), Player::new(String::from(name)));
        }
    }

    fn get_mut_player(&mut self, name: &str) -> &mut Player {
        self.players.get_mut(name).expect(&format!("Could not find Player with name: {name}"))
    }

    pub fn get_player(&self, name: &str) -> &Player {
        self.players.get(name).expect(&format!("Could not find Player with name: {name}"))
    }

    pub fn add_loc_player(&mut self, locname: &str, name: &str) {
        let player = self.get_mut_player(name);
        player.locations.insert(
            locname.to_string(),
            Rc::new(RefCell::new(Location::new(locname.to_string())))
        );
    }

    fn get_mut_team(&mut self, name: &str) -> &mut Team {
        self.teams.get_mut(name).expect(&format!("Could not find team with name: {name}"))
    }    

    pub fn get_team(&self, name: &str) -> &Team {
        self.teams.get(name).expect(&format!("Could not find team with name: {name}"))
    }

    pub fn add_loc_team(&mut self, locname: &str, teamname: &str) {
        let team = self.get_mut_team(teamname); // Uses `?` to propagate error
        team.locations.insert(
            locname.to_string(),
            Rc::new(RefCell::new(Location::new(locname.to_string())))
        );
    }

    pub fn add_loc_table(&mut self, locname: &str) {
        self.table.locations.insert(String::from(locname),
        Rc::new(RefCell::new(Location::new(String::from(locname)))));
    }

    pub fn get_mut_locs(&mut self, locname: &str) -> Vec<&mut Rc<RefCell<Location>>> {
        let mut locs: Vec<&mut Rc<RefCell<Location>>> = vec![];
    
        // Check self.table
        if let Some(l) = self.table.locations.get_mut(locname) {
            locs.push(l);
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
    
        if locs.is_empty() {
            eprintln!("No Location found in the whole Game with the name: {locname}");
        }

        locs
    }

    pub fn get_location(&self, loc_ref: &LocationRef) -> &Rc<RefCell<Location>> {
        match loc_ref {
            LocationRef::Own(locname) => {
                let pname = &self.turnorder[self.current];
                self
                    .get_player(pname)
                    .locations
                    .get(locname)
                    .or_else(|| {
                        self
                            .table
                            .locations
                            .get(locname)
                        }
                    )
                    .expect(&format!("No Location found at Player '{pname}' with name: {locname}\n
                                 AND No Location found at Table with name: {locname}"))
            }
            LocationRef::Player(pname, locname) => {
                self
                    .get_player(pname)
                    .locations
                    .get(locname)
                    .expect(&format!("No Location found at Player '{pname}' with name: {locname}"))
            }
            LocationRef::Team(teamname, locname) => {
                self
                    .get_team(teamname)
                    .locations
                    .get(locname)
                    .expect(&format!("No Location found at Team '{teamname}' with name: {locname}"))
            }
            LocationRef::Table(locname) => {
                self
                    .table
                    .locations
                    .get(locname)
                    .expect(&format!("No Location found at Table with name: {locname}"))
            }
        }
    }


    // move quantity cards
    pub fn move_q_cards<'a>(&'a self, q: usize, mut fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> impl FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a{
        move |cardsfromto: Vec<((LocationRef, usize), (LocationRef, usize))>| {
            use std::collections::HashMap;

            // Validate all locations exist
            for ((from_loc, i1), (to_loc, _)) in &cardsfromto {
                if !fromcs.contains_key(from_loc) {
                    panic!("Source location {:?} not in fromcs", from_loc);
                } else {
                    let location_ref = self.get_location(from_loc);
                    let location_borrow = location_ref.borrow();
                    let cards_ref = location_borrow.get_cards_ref();
                    let card = cards_ref[*i1].clone();

                    if !fromcs.get(from_loc).unwrap().contains(&card) {
                        panic!("Card not on Source-CardSet {:?} in fromcs", card);
                    }
                }
                if !tocs.contains_key(to_loc) {
                    panic!("Target location {:?} not in tocs", to_loc);
                }
            }
        
            // Group by from location
            let mut grouped_from: HashMap<LocationRef, Vec<usize>> = HashMap::new();
            for ((from_loc, index), _) in &cardsfromto {
                grouped_from.entry(from_loc.clone()).or_default().push(*index);
            }
        
            // For each from location, sort indices descending and remove cards
            let mut moved_cards: Vec<(Card, LocationRef, usize)> = vec![];
            for (loc, mut indices) in grouped_from {
                indices.sort_unstable_by(|a, b| b.cmp(a)); // high to low
                let from_vec = fromcs.get_mut(&loc).unwrap();
        
                for index in indices {
                    let card = from_vec.remove(index);
                    // Find destination info from original list
                    let (_, (to_loc, to_index)) = cardsfromto
                        .iter()
                        .find(|((f, i), _)| f == &loc && *i == index)
                        .unwrap();
                    moved_cards.push((card, to_loc.clone(), *to_index));
                }
            }

            if moved_cards.len() != q {
                panic!("Player has to move {} Cards!", q)
            }

            for (_, loc_ref, i) in moved_cards.iter() {
                let location = self.get_location(loc_ref);
                location.borrow_mut().remove_card_at_index(*i);
            }
        
            // Sort by destination index descending and insert
            moved_cards.sort_by(|a, b| b.2.cmp(&a.2));
            for (card, to_loc, index) in moved_cards {
                let location = self.get_location(&to_loc);
                location.borrow_mut().add_card_index(card, index);
            }
        }
    }

    // moving something bound means,
    // that after the cards have been moved,
    // they stay 'glued' together and you can reference all of the cards by one index
    // (in my opinion its very 'annoying' to implement and not an important feature, but i can be wrong)
    // pub fn move_q_cards_bound<'a>(&'a self, q: usize, mut fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> impl FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a{
    //        
    // }

    fn move_cardsets(&mut self, fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) {
        for (from_locref, cards) in fromcs.into_iter() {
            let _: Vec<Card> = cards;
            let from_loc = self.get_location(&from_locref);
            for (to_locref, _) in &tocs {
                let to_loc = self.get_location(to_locref);
                from_loc.borrow_mut().move_cards(&mut to_loc.borrow_mut(), &cards);
                break; // Only move to one destination per source
            } 
        }
    }

    pub fn deal_1_card<'a>(&'a self, fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> impl FnOnce((LocationRef, LocationRef)) + 'a{
        move |cardsfromto: (LocationRef, LocationRef)| {
            let from_loc = cardsfromto.0;
            let to_loc = cardsfromto.1;
            
            // Validate all locations exist
            if !fromcs.contains_key(&from_loc) {
                panic!("Source location {:?} not in fromcs", from_loc);
            }
            if !tocs.contains_key(&to_loc) {
                panic!("Target location {:?} not in tocs", to_loc);
            }
            
            let mut fromlocation = self.get_location(&from_loc).borrow_mut();
            let mut tolocation = self.get_location(&to_loc).borrow_mut();

            fromlocation.move_card_index(&mut *tolocation, 0, 0);        
        }
    }

    pub fn add_team(&mut self, name: &str, players: Vec<&str>) {
        self.teams.insert(String::from(name),
        Team::new(String::from(name), players
            .iter()
            .map(|p| String::from(*p))
            .collect()));

        for p in players {
            self.playertoteam.insert(String::from(p), String::from(name));
        }
    }

    pub fn add_precedence(&mut self, precedence: Precedence) {
        self.precedences.insert(precedence.name.clone(),precedence);
    }

    pub fn get_precedence(&self, precname: &str) -> &Precedence {
        self.precedences.get(precname).expect(&format!("No Precedence found with name: {precname}"))
    }

    pub fn add_pointmap(&mut self, pointmap: PointMap) {
        self.pointmaps.insert(pointmap.name.clone(), pointmap);
    }

    pub fn get_pointmap(&self, pname: &str) -> &PointMap {
        self.pointmaps.get(pname).expect(&format!("No PointMap with name: {pname}"))
    }

    pub fn set_turnorder(&mut self, playernames: Vec<String>) {
        self.turnorder = playernames;
    }

    pub fn add_cardcombination(&mut self, name: &str, cardcomb: CardCombination) {
        self.cardcombinations.insert(String::from(name), cardcomb);
    }

    pub fn get_combo(&self, comboname: &str) -> &CardCombination {
        self.cardcombinations.get(comboname).expect(&format!("No CardCombination with the name: {comboname}"))
    }

    pub fn apply_combo(&mut self, comboname: &str, locref: &LocationRef) -> Vec<Vec<Card>> {
        let loc = self.get_location(locref);
        let cardcombo: &CardCombination = self.get_combo(comboname);
        let cards = cardcombo
            .attributes
            .deref()(loc
                .borrow()
                .contents
                .clone());
        cards
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
}
// This can be done better, but it is complicated with the Rc<RefCell<...>>
// Lets stick to this one and change it if we have time left.
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.score == other.score
        // Not comparing locations!
    }
}
impl Eq for Player {}

impl Default for Player {
    fn default() -> Player {
        Player {
            name: format!("default"),
            score: 0,
            locations: HashMap::new()
        }
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
        for (k, _) in self.locations.iter() {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationRef {
    Own(String),            // e.g., Own("hand")
    Player(String, String), // e.g., Player("P2", "hand")
    Table(String),          // e.g., Table("drawpile")
    Team(String, String),   // e.g., Team("TeamA", "bench")
}
impl fmt::Display for LocationRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationRef::Own(loc) => write!(f, "\"{}\"", loc),
            LocationRef::Player(player, loc) => write!(f, "\"{}\" of {}", loc, player),
            LocationRef::Table(loc) => write!(f, "\"{}\" of Table", loc),
            LocationRef::Team(team, loc) => write!(f, "\"{}\" of Team {}", loc, team),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    //    AREA(Area),
    //    PILE(Pile),
    pub name: String,
    // I dont like that we can have Tokens and Cards in one Vec.
    // I would rather have a seperate Location that either takes tokens or cards.
    // This is just inconvenient for everything and can lead to unwanted bugs.
    pub contents: Vec<Card>
}
impl Location {
    pub fn new(locname: String) -> Location {
        Location { name: locname, contents: vec![]}
    }

    pub fn get_cards(self) -> Vec<Card> {
        self.contents
    }

    pub fn get_cards_ref(&self) -> &Vec<Card> {
        &self.contents
    }

    pub fn remove_card_at_index(&mut self, i: usize) -> Card {
        self.contents.remove(i)
    }

    pub fn add_card(&mut self, card: Card) {
        self.contents.push(card);
    }

    pub fn add_card_index(&mut self, card: Card, index: usize) {
        self.contents.insert(index, card);
    }

    pub fn remove_card(&mut self, card: &Card) {
        self.contents.retain(|c| {
            c != card
        });
    }

    pub fn extract_cards(self) -> Vec<Card> {
        self.contents
    }

    pub fn has_card(&self, card: &Card) -> bool {
        self.contents.contains(card)
    }

    pub fn move_card(&mut self, target: &mut Location, card: &Card) -> bool {
        if let Some(pos) = self.contents.iter().position(|c| c == card) {
            let removed = self.contents.remove(pos);
            target.contents.push(removed);
            true
        } else {
            false
        }
    }

    pub fn move_cards(&mut self, target: &mut Location, cards: &Vec<Card>) -> usize {
        let mut moved_count = 0;

        for card in cards {
            if let Some(index) = self.contents.iter().position(|c| c == card) {
                let comp = self.contents.remove(index);
                target.contents.push(comp);
                moved_count += 1;
            }
        }

        moved_count
    }

    pub fn move_card_index(
        &mut self,
        target: &mut Location,
        target_index: usize,
        card_index: usize
    ) {
        let card = self.remove_card_at_index(card_index);
        target.add_card_index(card, target_index);
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

impl Default for Precedence {
    fn default() -> Self {
        Precedence { name: format!("default"), attributes: HashMap::new() }
    }
}

impl Precedence {
    pub fn get_card_value_ref(&self, card: &Card) -> Option<usize> {
        for (_, value) in &card.attributes {
            if let Some(score) = self.attributes.get(value) {                
                return Some(*score);
            }
        }

        None
    }

    pub fn get_card_value(&self, card: Card) -> Option<usize> {
        for (_, value) in card.attributes {
            if let Some(score) = self.attributes.get(&value) {
                return Some(*score);
            }
        }

        None
    }
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

impl PointMap {
    pub fn get_card_value_ref(&self, card: &Card) -> Option<Vec<i32>> {
        for (_, value) in &card.attributes {
            if let Some(score) = self.entries.get(value) {                
                return Some(score.clone());
            }
        }

        None
    }

    pub fn get_card_value(&self, card: Card) -> Option<Vec<i32>> {
        for (_, value) in card.attributes {
            if let Some(score) = self.entries.get(&value) {
                return Some(score.clone());
            }
        }

        println!("SOMETHING WENT WRONG!");

        None
    }   
}

#[derive(Debug, Clone)]
pub enum Stage {
    SIM(StageS),
    SEQ(StageS),
}

#[derive(Debug)]
pub struct StageS {
    pub name: String,
    pub endconditions: Vec<Condition>,
    pub substages: Vec<Stage>,
    pub turncounter: i32,
    pub rules: Vec<Rule>,
}

impl Clone for StageS {
    fn clone(&self) -> Self {
        StageS {
            name: self.name.clone(),
            endconditions: vec![], // or panic!(), or skip, or clone dummy data
            substages: self.substages.clone(),
            turncounter: self.turncounter,
            rules: self.rules.clone(),
        }
    }
}

// #[derive(Debug, Clone)]
pub struct Condition {
    condition: Box<dyn Fn(&CardGameModel) -> bool>,
}
impl Condition {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&CardGameModel) -> bool + 'static,
    {
        Self {
            condition: Box::new(f),
        }
    }

    pub fn evaluate(&self, model: &CardGameModel) -> bool {
        (self.condition)(model)
    }
}
impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<Condition>")
    }
}

#[derive(Debug)]
pub struct ConditionalCase {
    pub conditions: Vec<Condition>,
    pub rules: Vec<Rule>,
}

impl Clone for ConditionalCase {
    fn clone(&self) -> Self {
        ConditionalCase {
            conditions: vec![], // or panic!(), or skip, or clone dummy data
            rules: self.rules.clone(),
        }
    }
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

#[derive(Debug)]
pub struct Play {
    pub endconditions: Vec<Condition>,
    pub stages: Vec<Stage>,
}

impl Clone for Play {
    fn clone(&self) -> Self {
        Play {
            endconditions: vec![], // or panic!(), or skip, or clone dummy data
            stages: self.stages.clone(),
        }
    }
}

impl Play {
    pub fn add_endcondition(&mut self, end_cond: Condition) {
        self.endconditions.push(end_cond);
    }

    pub fn add_stage(&mut self, stage: Stage) {
        self.stages.push(stage);
    }
}


// // Error-handling Logic
// #[derive(Debug)]
// pub enum GameError {
//     PlayerNameNotFound(String),
//     TeamNameNotFound(String),
//     LocationNameNotFound(String),
//     ComboNameNotFound(String),
//     PrecedenceNameNotFound(String),
//     PlayerNotFound,
//     TeamNotFound,
//     LocationNotFound,
//     InvalidInput(String),
// }

// impl fmt::Display for GameError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             // MyError::NotFound => write!(f, "Item not found"),
//             // MyError::InvalidInput => write!(f, "Invalid input provided"),
//             GameError::PlayerNameNotFound(pname) => write!(f, "PlayerName: {pname} not found"),
//             GameError::TeamNameNotFound(tname) => write!(f, "TeamName: {tname} not found"),
//             GameError::LocationNameNotFound(lname) => write!(f, "LocationName: {lname} not found"),
//             GameError::ComboNameNotFound(cname) => write!(f, "ComboName: {cname} not found"),
//             GameError::PrecedenceNameNotFound(precname) => write!(f, "ComboName: {precname} not found"),
//             GameError::PlayerNotFound => write!(f, "Player not found"),
//             GameError::TeamNotFound => write!(f, "Team not found"),
//             GameError::LocationNotFound => write!(f, "Location not found"),
//             GameError::InvalidInput(iinput) => write!(f, "Invalid input provided: {iinput}"),
//         }
//     }
// }
