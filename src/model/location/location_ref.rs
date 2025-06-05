use std::fmt;


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
