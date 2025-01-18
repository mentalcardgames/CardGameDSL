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

macro_rules! precedence {
    (
        $name:expr, // Name of the attribute for context
        ($($value:expr),* $(,)?)
        // TODO: add [key, value] Precedence!
    ) => {{
        use std::collections::HashMap;
        let mut precedence_map = HashMap::new();
        let mut index = 0;
        $(
            // TODO: might be overworked later
            precedence_map.insert($name.to_string() + &$value.to_string(), index);
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
        use crate::ast::Player;
        use std::rc::Rc;
        let mut players: Vec<Rc<Player>> = vec![$($player),*];
        let mut rng = rand::thread_rng();
        players.shuffle(&mut rng);
        players
    }};
    (($($player:expr),*)) => {{
        use crate::ast::Player;
        use std::rc::Rc;
        let players: Vec<Rc<Player>> = vec![$($player),*];
        players
    }};
}

macro_rules! filter {
    // Combined filters with "and" or "or"
    (($($filter1:tt)+), ($logical:literal), ($($filter2:tt)+)) => {{
        let filter1 = filter!($($filter1)+);
        let filter2 = filter!($($filter2)+);
        move |cards: Vec<Card>| {
            match $logical {
                "and" => {
                    let intermediate = filter1(cards.clone());
                    filter2(intermediate)
                },
                "or" => {
                    let mut result = filter1(cards.clone());
                    let mut other_result = filter2(cards);
                    result.append(&mut other_result);
                    result
                },
                _ => panic!("Invalid logical operator: {}", $logical),
            }
        }
    }};
    
    // Filter for key with "same" or "distinct" values
    ($key:expr, $condition:literal) => {{
        move |cards: Vec<Card>| {
            match $condition {
                "same" => {
                    if cards.is_empty() {
                        return vec![];
                    }
                    // we want all cards with the same key
                    // Ex.: rank same -> (rank, ...), (rank, ...), (rank, ...), (rank, ...)
                    use std::collections::{HashMap};

                    let mut groups: HashMap<String, Vec<Card>> = HashMap::new();
                    
                    // Iterate over references to cards to avoid consuming the original cards
                    for card in cards.iter() {
                        // Get the value of the attribute for this card
                        if let Some(value) = card.attributes.get($key) {
                            // Insert the card into the appropriate group based on its attribute value
                            groups.entry(value.clone()) // Use the attribute value as the key
                                .or_insert_with(Vec::new) // If no group exists, initialize a new Vec
                                .push(card.clone()); // Add the card to the group
                        }
                    }

                    // Now, we return the groups as Vec<Vec<Card>>
                    let result: Vec<Vec<Card>> = groups.into_iter().map(|(_, group)| group).collect();
                    result
                }
                // TODO:
                "distinct" => {
                    use std::collections::{HashMap};

                    let mut groups: HashMap<String, Vec<Card>> = HashMap::new();
                    
                    // Iterate over references to cards to avoid consuming the original cards
                    for card in cards.iter() {
                        // Get the value of the attribute for this card
                        if let Some(value) = card.attributes.get($key) {
                            // Insert the card into the appropriate group based on its attribute value
                            groups.entry(value.clone()) // Use the attribute value as the key
                                .or_insert_with(Vec::new) // If no group exists, initialize a new Vec
                                .push(card.clone()); // Add the card to the group
                        }
                    }

                    // Now, we return the groups as Vec<Vec<Card>>
                    let result: Vec<Vec<Card>> = groups.into_iter().map(|(_, group)| group).collect();
                    result
                }
                _ => panic!("Invalid condition: {}", $condition),
            }
        }
    }};

    // Filter for key with "adjacent", "higher", "lower" using precedence
    ($key:expr, $condition:literal using $precedence_map:expr) => {{
        let precedence_map_ref = &$precedence_map;
        move |cards: Vec<Card>| {
            cards
                .iter()
                .filter(|card| {
                    if let Some(current_value) = card.attributes.get($key) {
                        if let Some(current_index) = precedence_map_ref.get(&(String::from($key) + current_value)) {
                            match $condition {
                                "adjacent" => {
                                    return cards.iter().any(|other| {
                                        if let Some(other_value) = other.attributes.get($key) {
                                            if let Some(other_index) =
                                                precedence_map_ref.get(&(String::from($key) + other_value))
                                            {
                                                return (*current_index as i32 - *other_index as i32).abs() == 1;
                                            }
                                        }
                                        false
                                    });
                                }
                                // TODO:
                                "higher" => {
                                    return cards.iter().any(|other| {
                                        if let Some(other_value) = other.attributes.get($key) {
                                            if let Some(other_index) =
                                                precedence_map_ref.get(&(String::from($key) + other_value))
                                            {
                                                return current_index > other_index;
                                            }
                                        }
                                        false
                                    });
                                }
                                // TODO:
                                "lower" => {
                                    return cards.iter().any(|other| {
                                        if let Some(other_value) = other.attributes.get($key) {
                                            if let Some(other_index) =
                                                precedence_map_ref.get(&(String::from($key) + other_value))
                                            {
                                                return current_index < other_index;
                                            }
                                        }
                                        false
                                    });
                                }
                                _ => panic!("Invalid condition: {}", $condition),
                            }
                        }
                    }
                    false
                })
                .map(|card| card.clone())
                .collect::<Vec<_>>()
        }
    }};

    // Filter by size
    (size, $comparison:literal, $size:expr) => {{
        move |cards: Vec<Card>| {
            match $comparison {
                "==" => cards.len() == $size,
                "!=" => cards.len() != $size,
                "<" => cards.len() < $size,
                ">" => cards.len() > $size,
                "<=" => cards.len() <= $size,
                ">=" => cards.len() >= $size,
                _ => panic!("Invalid comparison operator: {}", $comparison),
            }
        }
    }};

    // Filter by key with "==" or "!=" string values
    ($key:expr, $comparison:literal, $value:expr) => {{
        move |cards: Vec<Card>| {
            cards
                .into_iter()
                .filter(|card| match $comparison {
                    "==" => card.attributes.get($key) == Some(&$value.to_string()),
                    "!=" => card.attributes.get($key) != Some(&$value.to_string()),
                    _ => panic!("Invalid comparison operator: {}", $comparison),
                })
                .collect::<Vec<Card>>()
        }
    }};
}
