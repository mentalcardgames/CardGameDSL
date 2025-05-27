use crate::ast::CardGameModel;
use std::collections::HashMap;

// Work on String interpreter for setup will be discard later

pub fn run() {
    let mut cgm = CardGameModel::new("NewGame");

    let mut keywords: HashMap<&str, fn(&[&str])> = HashMap::new();
    keywords.insert("Player", keyword_player);
    keywords.insert("Turnorder", keyword_turnorder);
    keywords.insert("Location", keyword_location);
    

    string_interpreter("Open")
}


fn string_interpreter(input: &str) {
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        
        for (&keyword, &keyword_function) in &keywords {
            if let Some(pos) = tokens.iter().position(|&t| == keyword) {
                let args = tokens[pos + 1..];
                keyword_function(args);
                break;
            }
        }
    }
}

fn keyword_player (args: &[&str]) {
    player!(args)(&mut cgm.gamedata);
}

fn keyword_turnorder (args: &[&str]) {
    turn_order!((args))(&mut cgm.gamedata); 
}

fn keyword_location (args: &[&str]) {
    location_on!("stack", table)(&mut cgm.gamedata);
}


