use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


use crate::model::owners::table::{Table};
use crate::model::owners::team::{Team};
use crate::model::owners::player::{Player};
use crate::model::gamedata::point_map::{PointMap};
use crate::model::gamedata::precedence::{Precedence};
use crate::model::card::card_combination::{CardCombination};
use crate::model::location::location::{Location};
use crate::model::location::location_ref::{LocationRef};
use crate::model::card::card::{Card};




#[derive(Debug)]
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

    pub fn get_mut_player(&mut self, name: &str) -> &mut Player {
        self.players.get_mut(name).expect(&format!("Could not find Player with name: {name}"))
    }

    pub fn get_player(&self, name: &str) -> &Player {
        self.players.get(name).expect(&format!("Could not find Player with name: {name}"))
    }

    pub fn get_player_copy(&self, name: &str) -> Player {
        self.players.get(name).expect(&format!("Could not find Player with name: {name}")).clone()
    }

    pub fn add_loc_player(&mut self, locname: &str, name: &str) {
        let player = self.get_mut_player(name);
        player.locations.insert(
            locname.to_string(),
            Rc::new(RefCell::new(Location::new(locname.to_string())))
        );
    }

    pub fn get_current(&self) -> &Player {
        let pname = self.turnorder[self.current].clone();
        self.get_player(&pname)
    }

    pub fn get_current_name(&self) -> String {
        self.turnorder[self.current].clone()
    }

    pub fn get_next_name(&self) -> String {
        let index = (self.current + 1) % self.turnorder.len();
        self.turnorder[index].clone()
    }

    pub fn update_current(&mut self, i: usize) {
        self.current = i % self.turnorder.len();
    }

    pub fn get_player_pos(&self, name: String) -> usize {
        for i in 0..self.turnorder.len() {
            if self.turnorder[i] == name {
                return i
            }
        }

        // TODO:
        // Default
        return 0
    }

    pub fn set_next_player(&mut self) {
        self.current = (self.current + 1) % self.turnorder.len();
    }

    fn get_mut_team(&mut self, name: &str) -> &mut Team {
        self.teams.get_mut(name).expect(&format!("Could not find team with name: {name}"))
    }    

    pub fn get_team(&self, name: &str) -> &Team {
        self.teams.get(name).expect(&format!("Could not find team with name: {name}"))
    }

    pub fn get_team_copy(&self, name: &str) -> Team {
        self.teams.get(name).expect(&format!("Could not find team with name: {name}")).clone()
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
    pub fn move_q_cards<'a>(&'a mut self, q: usize, mut fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> Box<dyn FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a> {
        Box::new(
            move |cardsfromto: Vec<((LocationRef, usize), (LocationRef, usize))>| {
                use std::collections::HashMap;

                if cardsfromto.len() != q {
                    panic!("Player has to move {} Cards!", q)
                }

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
                        
                        // remove from the Location
                        let location = self.get_location(&loc);
                        location.borrow_mut().remove_card_at_index(index);

                        // Find destination info from original list
                        let (_, (to_loc, to_index)) = cardsfromto
                            .iter()
                            .find(|((f, i), _)| f == &loc && *i == index)
                            .unwrap();
                        moved_cards.push((card, to_loc.clone(), *to_index));
                    }
                }
            
                // Sort by destination index descending and insert
                moved_cards.sort_by(|a, b| b.2.cmp(&a.2));
                for (card, to_loc, index) in moved_cards {
                    let location = self.get_location(&to_loc);
                    println!("{}", location.borrow().name);
                    location.borrow_mut().add_card_index(card, index);
                }
            }
        )
    }

    // moving something bound means,
    // that after the cards have been moved,
    // they stay 'glued' together and you can reference all of the cards by one index
    // (in my opinion its very 'annoying' to implement and not an important feature, but i can be wrong)
    // pub fn move_q_cards_bound<'a>(&'a self, q: usize, mut fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> impl FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a{
    //        
    // }
    pub fn move_cardset(&mut self, fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) {
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

    

    pub fn deal_1_card<'a>(&'a mut self, fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> Box<(dyn FnOnce((LocationRef, LocationRef)) + 'a)> {
        Box::new(
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
        )
    }

    pub fn deal_q_cards<'a>(&'a mut self, q: usize, fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> Box<dyn FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a> {
        Box::new(
            move |cards: Vec<((LocationRef, usize), (LocationRef, usize))>| {
                let deal_cards: Vec<(LocationRef, LocationRef)> = cards
                    .iter()
                    .map(|card| (card.0.0.clone(), card.1.0.clone()))
                    .collect();
                if q > deal_cards.len() {
                    panic!("To few cards to deal!");
                }
                for _ in 0..q {
                    self.deal_1_card(fromcs.clone(), tocs.clone())(deal_cards[0].clone());
                }
            }
        )
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
                .apply_func(self,
                loc
                .borrow()
                .contents
                .clone());
        cards
    }
}
