use crate::model::location::location_ref::{LocationRef};

pub enum PlayOutput<'a> {
    Move(Box<dyn FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a>),
    MoveCS(()),
    EndAction,
}
