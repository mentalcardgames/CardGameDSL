use crate::model::rules::condition::{Condition};
use crate::model::rules::rule::{Rule};


#[derive(Debug, Clone)]
pub struct ConditionalCase {
    pub condition: Condition,
    pub rules: Vec<Rule>,
    pub str_repr: String,
}
