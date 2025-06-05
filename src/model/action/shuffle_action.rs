use crate::model::card::card_set::CardSet;
use crate::model::card::card::Card;
use crate::model::location::location_ref::LocationRef;
use crate::model::location::location::Location;
use crate::model::card_game_model::{CardGameModel};

pub struct ShuffleAction {
    pub cardset: CardSet,
}
impl std::fmt::Debug for ShuffleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action::ShuffleAction(<closure>)")
    }
}
impl Clone for ShuffleAction {
    fn clone(&self) -> Self {
        ShuffleAction {
            cardset: self.cardset.clone(),
        }
    }
}
impl ShuffleAction {
    pub fn shuffle(&mut self, cgm: &mut CardGameModel) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        use std::collections::HashMap;
        use std::rc::Rc;
        use std::cell::RefCell;

        // Get the cardset for this shuffle
        let cardset: HashMap<LocationRef, Vec<Card>> = (self.cardset).get_card_set(&cgm.gamedata);

        for (loc_ref, cards_to_shuffle) in cardset.iter() {
            let location: &Rc<RefCell<Location>> = cgm.gamedata.get_location(loc_ref);
            let mut loc = location.borrow_mut();

            // Get mutable reference to location contents
            let contents = &mut loc.contents;

            // Find the indices of the cards to shuffle in the contents list
            let indices: Vec<usize> = contents
                .iter()
                .enumerate()
                .filter_map(|(i, card)| {
                    if cards_to_shuffle.contains(card) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();

            // Shuffle the indices logically by shuffling the corresponding cards
            let mut selected_cards: Vec<Card> = indices.iter().map(|&i| contents[i].clone()).collect();

            // Shuffle the cards randomly (ensure you have a reproducible RNG if needed)
            let mut rng = thread_rng();
            selected_cards.shuffle(&mut rng);

            // Put the shuffled cards back into the original indices
            for (i, &idx) in indices.iter().enumerate() {
                contents[idx] = selected_cards[i].clone();
            }
        }
    }
}
