use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::model::location::location::{Location};

#[derive(Debug, Clone)]
pub struct Team {
    pub name: String,
    pub players: Vec<String>,
    pub locations: HashMap<String, Rc<RefCell<Location>>>,
}
impl Team {
    pub fn new(name: String, players: Vec<String>) -> Team {
        Team {
            name: name,
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
            println!("Player {}: locname={}", self.name, k.to_string())
        }
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.players == other.players
    }
}