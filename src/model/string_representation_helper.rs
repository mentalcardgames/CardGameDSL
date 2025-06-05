use crate::model::rules::rule::Rule;
use crate::model::base_types::g_string::GString;
use crate::model::base_types::g_int::GInt;



pub fn str_repr_locations(locs: Vec<&str>) -> String {
    let locnames: Vec<&str> = locs;
    let mut str_locs: String = String::from("(");

    for i in 0..locnames.len() {
        if i != locnames.len() - 1 {
            str_locs = format!("{}'{}',", str_locs, locnames[i]);
        } else {
            str_locs = format!("{}'{}')", str_locs, locnames[i]);
        }
    }

    return str_locs
}

pub fn str_repr_choose_rule(rules: &Vec<Rule>) -> String {
    let rules: Vec<String> = rules.iter().map(|r| r.get_str_repr()).collect();

    let mut string_rules = String::from("CHOOSE:");

    for i in 0..rules.len() {
        if i != rules.len() - 1 {
            string_rules = format!("{}\n{}\nOR:", string_rules, rules[i]);
        } else {
            string_rules = format!("{}\n{}\n", string_rules, rules[i]);
        }
    }

    return string_rules
}

pub fn str_repr_optional_rule(rules: &Vec<Rule>) -> String {
    let rules: Vec<String> = rules.iter().map(|r| r.get_str_repr()).collect();

    let mut string_rules = String::from("OPTIONAL:");

    for i in 0..rules.len() {
        string_rules = format!("{}\n{}\n", string_rules, rules[i]);
    }

    return string_rules
}

pub fn str_repr_if_rule(rules: &Vec<Rule>, b: String) -> String {
    let rules: Vec<String> = rules.iter().map(|r| r.get_str_repr()).collect();

    let mut string_rules = format!("IF {} THEN (", b);

    for i in 0..rules.len() {
        string_rules = format!("{}\n{}", string_rules, rules[i]);        
    }
    
    string_rules = format!("{}\n)", string_rules);        

    return string_rules
}

pub fn str_repr_rules(rules: &Vec<Rule>) -> String {
    let rules: Vec<String> = rules.iter().map(|r| r.get_str_repr()).collect();

    let mut string_rules = rules[0].clone();

    for i in 1..rules.len() {
        string_rules = format!("{}\n{}", string_rules, rules[i]);        
    }

    return string_rules
}

pub fn str_repr_intcollection(ints: &Vec<GInt>) -> String {
    let vec_str_ints: Vec<String> = ints.iter().map(|gint| gint.str_repr.clone()).collect();

    let mut string_ints = vec_str_ints[0].clone();

    for i in 1..vec_str_ints.len() {
        string_ints = format!("{}, {}", string_ints, vec_str_ints[i]);        
    }

    string_ints = format!("{})", string_ints);

    return string_ints
}

pub fn str_repr_stringcollection(strings: &Vec<GString>) -> String {
    let vec_str_strings: Vec<String> = strings.iter().map(|gstring| gstring.str_repr.clone()).collect();

    let mut string_strings = vec_str_strings[0].clone();

    for i in 1..vec_str_strings.len() {
        string_strings = format!("{}, {}", string_strings, vec_str_strings[i]);        
    }

    string_strings = format!("{})", string_strings);

    return string_strings
}