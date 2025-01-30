macro_rules! player {
    ($($n:expr), *) => {
        {
            use crate::ast::Player;
            use std::rc::Rc;
            use std::cell::RefCell;

            let player_names: Vec<String> = vec![$($n.to_string()), *];
            let players: Vec<Rc<RefCell<Player>>> = player_names.iter().map(|x| Rc::new(RefCell::new(Player::new(x.to_string())))).collect();
            players
        }
    }
}

macro_rules! team {
    ($n:expr, ($($p:expr), *)) => {
        {
            use crate::ast::Team;
            use crate::ast::Player;
            use std::cell::RefCell;
            use std::rc::Rc;

            let player_names: Vec<String> = vec![$($p.to_string()), *];
            let name = $n.to_string();
            let players: Vec<Rc<RefCell<Player>>> = player_names.iter().map(|x| Rc::new(RefCell::new(Player::new(x.to_string())))).collect();
            let team = Team::new(name, players);
            team
        }
    };
}

macro_rules! location_on {
    ($location:literal, players: $players:expr) => {
        {
            use std::cell::RefCell;
            use std::rc::Rc;
            let players: Vec<Rc<RefCell<Player>>> = $players;
            for player in players.iter() {
                player.borrow_mut().add_location($location.to_string());
            }
        }
    };

    ($location:literal, team: $team:expr) => {
        {
            use crate::ast::Team;
            let team: &mut Team = $team;
            (*team).add_location($location.to_string());
        }
    };
    ($location:literal, table: $table:expr) => {
        {
            let table = $table;
            table.add_location($location.to_string());
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

        // iterate over every player, team and table!
        // then assign the cards to the correct location!

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

// OR DOESNT WORK YET!
macro_rules! filter {
    /*
    How it (should) works:
        We want to compute the all legal moves ("all" in a kind of way).
        For that we compute all possible "legal-moves".
        We then get a Vector of playable moves, so
        a Vec<Vec<Card>>.
        Example:
            [1, 1, 2, 3, 3]
            => rank same -> [[1, 1], [3, 3]]
        
        If we now get a second filter we apply it on every Vector in playabler moves:
        Example:
            [(1, Black), (1, Red), (2, Red), (3, Black), (3, Black)]
            => rank same -> [[(1, Black), (1, Red)], [(3, Black), (3, Black]]
            => suite same -> [[(3, Black), (3, Black]]
        
        We see that we do NOT change the type of the input.
        In other words we can apply this scheme indifferently many times
        without changing the structure.

        We also need to diminish unecassary "explosion" of the Vector size.
        For Example:
            We have this Vec<Card>:
                [1, 1, 1, 1, 1, 1]
            If we would now filter for (rank same),
            then we could have all different kinds of playable moves:
            => [[1,1], [1, 1, 1], ...]

            This is a very big expansion (2^n)!.
            We also do NOT want to lose information!
            Therefore we compute S in {s | s not in S },
            so every Set of playable moves that is not a
            Subset of a different playable move.

        However, we still need to distinguish between the other
        attributes that are not mentioned in the filter, because
        they might be needed in a later filter.
        If we do NOT do that:
        For example:
            [(1, Black), (1, Red), (2, Red), (3, Black), (3, Black), (3, Red)]
            => rank adjacent -> [[(1, Black), (2, Red), (3, Black)]]
            => suite same -> size > 2 -> [[]]
        
        If we do that:
        For example:
            [(1, Black), (1, Red), (2, Red), (3, Black), (3, Black), (3, Red)]
            => rank adjacent -> [
                                    [(1, Black), (2, Red), (3, Black)],
                                    [(1, Red),   (2, Red), (3, Black)],
                                    [(1, Black), (2, Red), (3, Red)],
                                    [(1, Red),   (2, Red), (3, Red)],
                                ]
            => suite same -> size > 2 -> [[(1, Red),   (2, Red), (3, Red)]]
        We can see by the example that the playable moves, after (rank adjacent),
        are all no subset of eachother!
        So we still keep all of the information!

    We say that a filter is FALSE,
    whem the returned Vector is empty!

    It is TRUE otherwise!
    */

    // Combine filters with "and" or "or" for Vec<Vec<Card>> results
    (($($filter1:tt)+), ($logical:literal), ($($filter2:tt)+)) => {{
        // Recursive Call of filter!
        let filter1 = filter!($($filter1)+);
        let filter2 = filter!($($filter2)+);
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            // Apply first filter
            let filtered1 = filter1(cards.clone());

            // Wrap the result of filter1 into Vec<Vec<Card>> if it's Vec<Card>
            let filtered1 = if let Some(group) = filtered1.into_iter().next() {
                vec![group]
            } else {
                vec![] // If it's an empty result, return Vec<Vec<Card>>
            };

            // Apply the second filter to the result of the first filter
            let filtered2 = filter2(filtered1.clone().into_iter().flatten().collect::<Vec<Card>>());
            
            match filtered2 {
                // If filtered2 returns Vec<Vec<Card>>, handle the logic
                filtered2_groups if filtered2_groups.iter().all(|x| !x.is_empty()) => {
                    match $logical {
                        "and" => filtered1.into_iter()
                            .filter(|group| filtered2_groups.contains(group))
                            .collect(),
                        "or" => {
                            let mut result = filtered1;
                            for group in filtered2_groups {
                                if !result.contains(&group) {
                                    result.push(group);
                                }
                            }
                            result
                        },
                        _ => panic!("Invalid logical operator: {}", $logical),
                    }
                }
                // Handle error for type mismatch
                _ => panic!("Filter type mismatch"),
            }
        }
    }};

    // Group by "same"
    ($key:expr, "same") => {{
        fn group_by_same(cards: Vec<Card>, key: &str) -> Vec<Vec<Card>> {
            use std::collections::HashMap;
            let mut groups: HashMap<String, Vec<Card>> = HashMap::new();
            for card in cards {
                if let Some(value) = card.attributes.get(key) {
                    groups.entry(value.clone())
                        .or_insert_with(Vec::new)
                        .push(card);
                }
            }
            groups.into_values()
                .filter(|group| group.len() > 1) // Only keep groups with more than one card
                .collect()
        }

        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            group_by_same(cards, $key)
        }
    }};

    // Group by "adjacent"
    ($key:expr, "adjacent" using $precedence_map:expr) => {{
        use std::collections::HashMap;
        fn group_by_adjacent(cards: Vec<Card>, key: &str, precedence_map: &HashMap<String, usize>) -> Vec<Vec<Card>> {
            let mut sorted_cards: Vec<Card> = cards.clone().into_iter()
                .filter(|card| card.attributes.contains_key(key))
                .collect();
            sorted_cards.sort_by_key(|card| {
                card.attributes.get(key)
                    .and_then(|value| precedence_map.get(value))
                    .cloned()
                    .unwrap_or(usize::MAX)
            });
    
            let mut groups = Vec::new();
            let mut current_group = Vec::new();
            for card in sorted_cards {
                let c_value = card.attributes.get(key).cloned();  // Use `cloned` to avoid double cloning.

                if let Some(c_value) = c_value {
                    if current_group.is_empty() {
                        // If current_group is empty, we add the first value.
                        current_group.push(Some(c_value));
                    } else {
                        let last = current_group.last().unwrap();
                        if let Some(last_value) = last {
                            // We now work directly with the values, not cloned ones.
                            if let (Some(last_index), Some(current_index)) = (
                                precedence_map.get(&($key.to_string() + last_value).to_string()),
                                precedence_map.get(&($key.to_string() + &c_value).to_string()),
                            ) {
                                if *current_index == *last_index + 1 {
                                    current_group.push(Some(c_value));
                                    continue;  // If it's part of the same group, continue.
                                }
                                // We are looking for all adjacent indexes, so we skip over same ones!
                                if *current_index == *last_index {
                                    continue;
                                }
                            }
                        }

                        // End of current group, push it and reset current group.
                        groups.push(current_group);
                        current_group = vec![];
                    }
                }
            }

            if !current_group.is_empty() {
                groups.push(current_group);
            }

            fn group_by_same(cards: Vec<Card>, key: &str) -> HashMap<String, Vec<Card>> {
                use std::collections::HashMap;
                let mut groups: HashMap<String, Vec<Card>> = HashMap::new();
                for card in cards {
                    if let Some(value) = card.attributes.get(key) {
                        groups.entry(value.clone())
                            .or_insert_with(Vec::new)
                            .push(card);
                    }
                }
                return groups;
            }

            /// Generates all combinations by switching cards within the adjacency group.
            fn generate_combinations_by_switching(
                adjacency_groups: Vec<Vec<String>>,
                value_to_cards: HashMap<String, Vec<Card>>,
            ) -> Vec<Vec<Card>> {
                let mut result = Vec::new();

                // Process each adjacency group
                for group in adjacency_groups {
                    // Collect vectors of card options for each value in the group
                    let card_options: Vec<Vec<Card>> = group
                        .iter()
                        .filter_map(|value| value_to_cards.get(value))
                        .cloned()
                        .collect();

                    // Use a recursive function to compute the cartesian product for this group
                    let mut current_combination = Vec::new();
                    cartesian_product(&card_options, 0, &mut current_combination, &mut result);
                }

                result
            }

            /// Recursive function to compute the cartesian product of card options.
            fn cartesian_product(
                card_options: &[Vec<Card>],
                index: usize,
                current_combination: &mut Vec<Card>,
                result: &mut Vec<Vec<Card>>,
            ) {
                if index == card_options.len() {
                    result.push(current_combination.clone());
                    return;
                }

                for card in &card_options[index] {
                    current_combination.push(card.clone());
                    cartesian_product(card_options, index + 1, current_combination, result);
                    current_combination.pop();
                }
            }
            
            let same_values: HashMap<String, Vec<Card>> = group_by_same(cards, $key);
            
            let result = generate_combinations_by_switching(
                groups
                    .into_iter()
                    .map(|inner_vec| {
                        inner_vec
                            .into_iter()
                            .map(|opt| opt.map(|s| s.to_string()).unwrap_or_else(|| "".to_string()))
                        .collect()
                    })
                    .collect(),
                same_values);
            
            return result;
        }
            

        let precedence_map_ref = &$precedence_map;
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            group_by_adjacent(cards, $key, precedence_map_ref)
        }
    }};

    
    // Additional filters like "size" remain unchanged
    (size, $comparison:literal, $size:expr) => {{
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            match $comparison {
                "==" => {
                    if cards.len() == $size {return vec![cards]}
                    else {return vec![]}
                },
                "!=" => {
                    if cards.len() != $size {return vec![cards]}
                    else {return vec![]}
                },
                "<" => {
                    if cards.len() < $size {return vec![cards]}
                    else {return vec![]}
                },
                ">" => {
                    if cards.len() > $size {return vec![cards]}
                    else {return vec![]}
                },
                "<=" => {
                    if cards.len() <= $size {return vec![cards]}
                    else {return vec![]}
                },
                ">=" => {
                    if cards.len() >= $size {return vec![cards]}
                    else {return vec![]}
                },
                _ => panic!("Invalid comparison operator: {}", $comparison),
            }
        }
    }};

    // Additional filters like "size" remain unchanged
    ($key:literal, $comparison:literal, $value:literal) => {{
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            match $comparison {
                "==" => vec![cards.into_iter().filter(|card| card.attributes[$key] == $value).collect()],
                "!=" => vec![cards.into_iter().filter(|card| card.attributes[$key] != $value).collect()],
                _ => panic!("Invalid comparison operator: {}", $comparison),
            }
        }
    }};


}

macro_rules! combo {
    ($name:literal, "where", $filter:tt) => {
        
    };
}


macro_rules! Setup {
    ($cplayer:tt, ($cteam:tt)?, cturnorder:tt) => {

    }
}