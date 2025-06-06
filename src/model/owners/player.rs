use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use crate::model::location::location::{Location};



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
