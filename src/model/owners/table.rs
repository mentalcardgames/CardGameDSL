use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::model::location::location::{Location};

#[derive(Debug, Clone)]
pub struct Table {
    pub locations: HashMap<String, Rc<RefCell<Location>>>,
}
impl Table {
    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname))));
    }
}
