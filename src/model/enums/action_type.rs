#[derive(Debug, Clone)]
pub enum ActionType {
    None,
    EndAction,
    ChooseAction,
    MoveAction,
    DealAction,
    MoveCardSetAction,
    TriggerAction,
    OptionalAction,
}