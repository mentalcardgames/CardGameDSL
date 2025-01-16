macro_rules! player {
    ($($n:expr), *) => {
        {
            use crate::ast::Player;
            use std::rc::Rc;

            let player_names: Vec<String> = vec![$($n.to_string()), *];
            let players: Vec<Rc<Player>> = player_names.iter().map(|x| Rc::new(Player::new(x.to_string()))).collect();
            players
        }
    }
}

macro_rules! team {
    ($n:expr, ($($p:expr), *)) => {
        {
            use crate::ast::Team;
            use crate::ast::Player;
            use std::rc::Rc;

            let player_names: Vec<String> = vec![$($p.to_string()), *];
            let name = $n.to_string();
            let players: Vec<Rc<Player>> = player_names.iter().map(|x| Rc::new(Player::new(x.to_string()))).collect();
            let team = Team::new(name, players);
            team
        }
    };
}


// ChatGPT generated check later for mistakes!
macro_rules! card_on {
    (
        $location:expr,
        $(
            {
                $(
                    $attkey:ident($($attvalue:expr),* $(,)?)
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        {
        use crate::ast::Card;
        use std::collections::HashMap;
        use std::collections::BTreeSet;
        let mut keys_set: BTreeSet<String> = BTreeSet::new();

        println!("Location: {}", $location.name);

        let mut all_cards = Vec::new();
        let mut all_keys = Vec::new();
        let mut all_values = Vec::new();
        // Process each group of attributes
        $(
            {
                $(
                    let key = stringify!($attkey).to_string();
                    if !keys_set.contains(&key) {
                        keys_set.insert(key.clone());
                        all_keys.push(key);
                    }
                )*

                let mut cards = Vec::new();

                // Collect all attributes into a vector of vectors
                let attributes = vec![
                    $(
                        vec![$($attvalue.to_string()),*],
                    )*
                ];

                // Generate Cartesian product for this group
                fn cartesian_product(
                    attributes: &[Vec<String>],
                    current: Vec<String>,
                    results: &mut Vec<Vec<String>>
                ) {
                    if attributes.is_empty() {
                        results.push(current);
                        return;
                    }

                    let (first, rest) = attributes.split_first().unwrap();
                    for value in first {
                        let mut next = current.clone();
                        next.push(value.clone());
                        cartesian_product(rest, next, results);
                    }
                }

                let mut results = Vec::new();
                cartesian_product(&attributes, Vec::new(), &mut results);

                for combination in results {
                    cards.push(combination);
                }
                all_values.extend(cards);
            }
        )*
        for i in 0..all_values.len() {
            let mut attr: HashMap<String, String> = HashMap::new();
            for j in 0..all_values[i].len() {
                attr.insert(all_keys[j].clone(), all_values[i][j].clone());
            }
            all_cards.push(Card::new(attr));
        }

        all_cards
    }};
}

// ChatGPT look over later
macro_rules! precedence {
    (
        $name:expr, // Name of the attribute for context
        ($($value:expr),* $(,)?)
    ) => {{
        use std::collections::HashMap;
        let mut precedence_map = HashMap::new();
        let mut index = 0;
        $(
            precedence_map.insert($value.to_string(), index);
            index += 1;
        )*
        println!("Precedence for {}: {:?}", $name, precedence_map);
        precedence_map
    }};
}

macro_rules! pointmap {
    (

        // nested mapping
        $(
            nested: { 
                $($name1:expr,
                    ($($key1:expr => [$($value1:expr),*] ),* $(,)?)
                ),* $(,)? 
            }
        ),* $(,)?

        // flat mapping
        $(
            list: { 
                $(
                    ($name2:expr, $key2:expr) => [$value2:expr]
                ),* $(,)? 
            }
        ),* $(,)?

    ) => {{
        use std::collections::HashMap;
        let mut point_map: HashMap<String, Vec<i32>> = HashMap::new();

        // nested mapping
        $(
            $(
                $(
                    let key = format!("{}{}", $name1, $key1);
                    let entry = point_map.entry(key).or_insert_with(Vec::new);
                    $(
                        entry.push($value1);
                    )*
                )*
            )*
        )*

        // flat mapping
        $(
            $(
                let key = format!("{}{}", $name2, $key2);
                let entry = point_map.entry(key).or_insert_with(Vec::new);
                entry.push($value2);
            )*
        )*

        println!("Point map for {:?}", point_map);
        point_map
    }};
}

macro_rules! turn_order {
    (($($player:expr),*), random) => {{
        use rand::seq::SliceRandom;
        let mut players: Vec<String> = vec![$($player.to_string()),*];
        let mut rng = rand::thread_rng();
        players.shuffle(&mut rng);
        players
    }};
    (($($player:expr),*)) => {{
        let players: Vec<String> = vec![$($player.to_string()),*];
        players
    }};
}