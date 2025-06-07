use crate::model::rules::rule::Rule;

#[derive(Debug, Clone)]
pub struct Setup {
    pub setuprules: Vec<Rule>,
    pub str_repr: String,
}
impl Default for Setup {
    fn default() -> Self {
        Setup {
            setuprules: vec![],
            str_repr: String::from(""),
        }
    }
}