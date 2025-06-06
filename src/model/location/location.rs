use crate::model::card::card::{Card};


#[derive(Debug, Clone)]
pub struct Location {
    // TODO:
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

