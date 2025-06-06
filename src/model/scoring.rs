use crate::model::rules::rule::Rule;

#[derive(Debug, Clone)]
pub struct Scoring {
    pub scoringrules: Vec<Rule>,
    pub str_repr: String,
}
impl Default for Scoring {
    fn default() -> Self {
        Scoring {
            scoringrules: vec![],
            str_repr: String::from(""),
        }
    }
}
