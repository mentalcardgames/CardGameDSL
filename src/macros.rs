// $gd = gamedata
macro_rules! player {
    ($($n:literal), *) => {
        {
            use crate::ast::GameData;
            Box::new(
                |gd: &mut GameData| {
                    gd.add_players(vec![$($n), *])
                }
            )
        }
    }
}

macro_rules! team {
    ($n:expr, ($($p:expr), *)) => {
        {
            use crate::ast::GameData;
            Box::new(
                |gd: &mut GameData| {
                    gd.add_team($n, vec![$($p), *]);
                }
            )
        }
    };
}

/*
In what way can you get a location object?
- Using LocationRef and .get_location() (so like "hand" of player_ref!(...))
(i think that is the only way??????) 
*/
// macro_rules! location {
//     () => {
        
//     };
// }

macro_rules! location_on {
    ($location:literal, players: $($p:expr), *) => {
        {
            use crate::ast::GameData;
            Box::new(
                |gd: &mut GameData| {
                    for p in vec![$($p),*] {
                        gd.add_loc_player($location, p);
                    }
                }
            )
        }
    };

    ($location:literal, team: $team:expr) => {
        {
            use crate::ast::GameData;
            Box::new(
                |gd: &mut GameData| {
                    gd.add_loc_team($location, $team);
                }
            )
        }
    };
    ($location:literal, table) => {
        {
            use crate::ast::GameData;
            Box::new(
                |gd: &mut GameData| {
                    gd.add_loc_table($location);
                }
            )
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

        use crate::ast::GameData;
        Box::new(
            |gd: &mut GameData| {
                let mut keys_set: BTreeSet<String> = BTreeSet::new();

                // println!("Location: {}", $location.name);
        
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

                            let (first, rest) = attributes.split_first().expect("No Attributes to 'split on' in CardOn");
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

                // iterate over every player, team and table
                // then assign the cards to the correct location
                let locs = gd.get_mut_locs($location);
                for i in 0..locs.len() {
                    locs[i].borrow_mut().contents.extend(all_cards.clone());
                }
            }
       )
    }};
}

macro_rules! precedence {
    (
        $name:expr, // Name of the attribute for context
        ($($value:expr),* $(,)?)
        // TODO: add [key, value] Precedence!
    ) => {{
        use crate::ast::Precedence;
        use std::collections::HashMap;
        use crate::ast::GameData;

        Box::new(
            |gd: &mut GameData| {
                let mut precedence_map = HashMap::new();
                let mut index = 0;

                $(
                    precedence_map.insert($value.to_string(), index);
                    index += 1;
                )*

                gd.add_precedence(Precedence { name: $name.to_string(), attributes: precedence_map});
            }
        )
    }};
}

macro_rules! pointmap {
    (
        $pmapname:expr,

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
        use crate::ast::PointMap;

        use crate::ast::GameData;
        Box::new(
            |gd: &mut GameData| {
            
                let mut point_map: HashMap<String, Vec<i32>> = HashMap::new();
                // nested mapping
                $(
                    $(
                        $(
                            let entry = point_map.entry($key1.to_string()).or_insert_with(Vec::new);
                            $(
                                entry.push($value1);
                            )*
                        )*
                    )*
                )*

                // flat mapping
                $(
                    $(
                        let entry = point_map.entry($key2.to_string()).or_insert_with(Vec::new);
                        entry.push($value2);
                    )*
                )*

                // println!("Point map for {:?}", point_map);
                gd.add_pointmap(PointMap { name: $pmapname.to_string(), entries: point_map});
                // Modify gamedata
                // $cgm.gamedata.add_pointmap(PointMap {
                //     name: format!("{}", stringify!($($name1),*)), // Handle multiple `$name1`
                //     entries: point_map.clone(), // Return a copy if needed
                // });
            }
        )
    }};
}

macro_rules! turn_order {

    (random) => {{
        use rand::seq::SliceRandom;

        use crate::ast::GameData;
        Box::new(
            |gd: &mut GameData| {
                let mut turn_order: Vec<String> = gd.players.keys().cloned().collect();
                let mut rng = rand::thread_rng();
                turn_order.shuffle(&mut rng);
                gd.set_turnorder(turn_order);
            }
        )
    }};

    (($($pname:expr),*)) => {{
        use crate::ast::GameData;
        Box::new(
            |gd: &mut GameData| {
                gd.set_turnorder(vec![$(String::from($pname)),*]);
            }
        )
    }};

}

macro_rules! filter {
    /*
    How it works:
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
        use crate::ast::{Filter, GameData};
        use std::sync::Arc;

        Filter {
            func:
                Arc::new(
                    move |gd: &GameData, cards: Vec<Card>| -> Vec<Vec<Card>> {
                        let filter1 = filter!($($filter1)+);
                        let filter2 = filter!($($filter2)+);
                        
                        // Apply first filter, keep as Vec<Vec<Card>>
                        let filtered1 = filter1.apply_func(gd, cards.clone());

                        match $logical {
                            "and" => {
                                // Apply filter2 to each group individually, keep non-empty results
                                filtered1
                                    .into_iter()
                                    .flat_map(|group| {
                                        filter2.apply_func(gd, group)
                                            .into_iter()
                                            .filter(|g| !g.is_empty())
                                    })
                                    .collect()
                            }
                            "or" => {
                                let mut all_groups: Vec<Vec<Card>> = vec![];

                                // Collect all groups from both filters
                                for group in filter1.apply_func(gd, cards.clone()) {
                                    if !group.is_empty() && !all_groups.contains(&group) {
                                        all_groups.push(group);
                                    }
                                }
                                for group in filter2.apply_func(gd, cards) {
                                    if !group.is_empty() && !all_groups.contains(&group) {
                                        all_groups.push(group);
                                    }
                                }

                                all_groups
                            }
                            _ => panic!("Invalid logical operator: {}", $logical),
                        }
                    }
                ),
                str_repr: String::from("")
            }
    }};

    // Group by "same"
    (same $key:expr) => {{
        use crate::ast::{Filter, GameData};
        use std::sync::Arc;

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


        Filter {
            func: Arc::new(
                move |_: &GameData, cards: Vec<Card>| -> Vec<Vec<Card>> {
                    group_by_same(cards, $key)
                }
            ),
            str_repr: format!("SAME {}", $key)
        }
    }};

    // Group by "adjacent"
    (adjacent $key:literal using $precedence_map:literal) => {{
        use crate::ast::{Filter, GameData};
        use std::sync::Arc;
        
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
                        let last = current_group.last().expect("No 'last Group' in filter!(...)");
                        if let Some(last_value) = last {
                            // We now work directly with the values, not cloned ones.
                            if let (Some(last_index), Some(current_index)) = (
                                precedence_map.get(&(last_value).to_string()),
                                precedence_map.get(&(c_value).to_string()),
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

        Filter {
            func:
                Arc::new(
                    move |gd: &GameData, cards: Vec<Card>| -> Vec<Vec<Card>> {
                        let precedence_map = &gd
                            .get_precedence($precedence_map)
                            .attributes;
                        group_by_adjacent(cards, $key, precedence_map)
                    }
                ),
            str_repr: format!("ADJACENT {} USING {}", $key, $precedence_map)
        }
    }};


    (size $comparison:literal $size:expr) => {{
        use crate::ast::{Filter, GameData};
        use std::sync::Arc;

        Filter {
            func:
                Arc::new(
                    |_: &GameData, cards: Vec<Card>| -> Vec<Vec<Card>> {
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
                ),
            str_repr: format!("SIZE {} {}", $comparison, $size)
        }
    }};

    ($key:literal $comparison:literal $value:literal) => {{
        use crate::ast::{Filter, GameData};
        use std::sync::Arc;

        Filter {
            func:
                Arc::new(
                    move |_: &GameData, cards: Vec<Card>| {
                        match $comparison {
                            "==" => vec![cards.into_iter().filter(|card| card.attributes[$key] == $value).collect()],
                            "!=" => vec![cards.into_iter().filter(|card| card.attributes[$key] != $value).collect()],
                            _ => panic!("Invalid comparison operator: {}", $comparison),
                        }
                    }
                ),
            str_repr: format!("{} {} {}", $key, $comparison, $value)
        }
    }};

    ($comboname:literal) => {
        use crate::ast::{Filter, GameData};
        use std::sync::Arc;

        Filter {
            func:
                Arc::new(
                    move |gd: &GameData, cards: Vec<Card>| -> Vec<Vec<Card>> {
                        let cardcombo = gd.get_combo($comboname);
                        let cardfun: &Filter = &cardcombo.attributes;
                        cardfun.apply_func(gd, cards)
                    }
                ),
            str_repr: format!("{}", $comboname)
        }
    };

    (not $comboname:literal) => {{
        use crate::ast::{Filter, GameData};
        use std::sync::Arc;

        Filter {
            func:
                Arc::new(
                    move |gd: &GameData, cards| -> Vec<Vec<Card>> {
                        let cardcombo = gd.get_combo($comboname);
                        let cardfun: &Filter = &cardcombo.attributes;
                        let filtered_out: Vec<Card> = {
                            let mut seen = Vec::new();
                            for card in cardfun.apply_func(gd, cards.clone()).into_iter().flatten() {
                                if !seen.contains(&card) {
                                    seen.push(card);
                                }
                            }
                            seen
                        };
                
                        let remaining: Vec<Card> = cards
                            .into_iter()
                            .filter(|card| !filtered_out.contains(card))
                            .collect();
                
                        vec![remaining]
                    }
                ),
            str_repr: format!("NOT {}", $comboname)
        }
    }};
}

macro_rules! cardposition {
    ($locname:literal $int:literal) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname).get_card_set(gd);
                        let cards = card_map
                            .get(&LocationRef::Own($locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let card = cards
                            .get($int)
                            .expect(&format!("No Card at index {} in Location '{}' in CardSet", $int, $locname));
                        loc_card.insert(LocationRef::Own($locname.to_string()), 
                            vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("IN {} AT POSITION {}", $locname, $int)
        }
    }};

    ($locname:literal of player: $pref:expr, $int:literal) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname)(gd);
                        let cards = card_map
                            .get(&LocationRef::Player((*$pref)(gd).name, $locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let card = cards
                            .get($int)
                            .expect(&format!("No Card at index {} in Location '{}' in CardSet", $int, $locname));
                        loc_card.insert(LocationRef::Player((*$pref)(gd).name, $locname.to_string()), 
                            vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("IN {} OF {} AT POSITION {}", $locname, $pref.str_repr, $int)
        }
    }};

    ($locname:literal of team: $tref:expr, $int:literal) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname)(gd);
                        let cards = card_map
                            .get(&LocationRef::Team((*$tref)(gd).teamname.clone(), $locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let card = cards
                            .get($int)
                            .expect(&format!("No Card at index {} in Location '{}' in CardSet", $int, $locname));
                        loc_card.insert(LocationRef::Team((*$tref)(gd).teamname.clone(), $locname.to_string()), 
                            vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("IN {} OF {} AT POSITION {}", $locname, $tref.str_repr, $int)
        }
    }};



    ($locname:literal top) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname).get_card_set(gd);
                        let cards = card_map
                            .get(&LocationRef::Own($locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let card = cards
                            .get(0)
                            .expect(&format!("No Card at TOP in Location '{}' in CardSet", $locname));
                        loc_card.insert(LocationRef::Own($locname.to_string()), vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("TOP OF {}", $locname)
        }
    }};

    ($locname:literal of player: $pref:expr, top) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname)(gd);
                        let cards = card_map
                            .get(&LocationRef::Player(*$pref.name.clone(), $locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let card = cards
                            .get(0)
                            .expect(&format!("No Card at TOP in Location '{}' in CardSet", $locname));
                        loc_card.insert(LocationRef::Player(*$pref.name.clone(), $locname.to_string()), vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("TOP OF {} OF {}", $locname, $pref.str_repr)
        }
    }};

    ($locname:literal of team: $tref:expr, top) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {

                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname)(gd);
                        let cards = card_map
                            .get(&LocationRef::Team((*$tref)(gd).teamname.clone(), $locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let card = cards
                            .get(0)
                            .expect(&format!("No Card at TOP in Location '{}' in CardSet", $locname));
                        loc_card.insert(LocationRef::Team((*$tref)(gd).teamname.clone(), $locname.to_string()), vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("TOP OF {} OF {}", $locname, $tref.str_repr)
        }
    }};

    ($locname:literal bottom) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname).get_card_set(gd);
                        let cards = card_map
                            .get(&LocationRef::Own($locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let len = cards.len();
                        // TODO:
                        // That has to be handled later,
                        // because what if the location is empty???
                        let card = cards
                            .get(len - 1)
                            .expect(&format!("No Card at index {} in Location '{}' in CardSet", len - 1, $locname));
                        loc_card.insert(LocationRef::Own($locname.to_string()),
                            vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("BOTTOM OF {}", $locname)
        }
    }};

    ($locname:literal of player: $pref:expr, bottom) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($gd, $locname);
                        let cards = card_map
                            .get(&LocationRef::Player(*$pref.name.clone(), $locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let len = cards.len();
                        // TODO:
                        // That has to be handled later,
                        // because what if the location is empty???
                        let card = cards
                            .get(len - 1)
                            .expect(&format!("No Card at index {} in Location '{}' in CardSet", len - 1, $locname));
                        loc_card.insert(LocationRef::Player(*$pref.name.clone(), $locname.to_string()),
                            vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("BOTTOM OF {} OF {}", $locname, $pref.str_repr)
        }
    }};

    ($locname:literal of team: $tref:expr, bottom) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname)(gd);
                        let cards = card_map
                            .get(&LocationRef::Team((*$tref)(gd).teamname.clone(), $locname.to_string()))
                            .expect(&format!("No Location with name '{}' in Own(...) found in CardSet", $locname));
                        let len = cards.len();
                        // TODO:
                        // That has to be handled later,
                        // because what if the location is empty???
                        let card = cards
                            .get(len - 1)
                            .expect(&format!("No Card at index {} in Location '{}' in CardSet", len - 1, $locname));
                        loc_card.insert(LocationRef::Team((*$tref)(gd).teamname.clone(), $locname.to_string()),
                            vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("BOTTOM OF {} OF {}", $locname, $pref.str_repr)
        }
    }};

    (min of $cardset:tt using prec: $precname:literal) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let prec = gd.get_precedence($precname);
                        // First, collect all cards with their location and score
                        let mut scored_cards: Vec<(LocationRef, Card, usize)> = vec![];

                        for (loc, cards) in &$cardset.get_card_set(gd) {
                            for card in cards {
                                let score = prec.get_card_value_ref(card).expect(&format!("No value in PointMap for Card '{}'", card));
                                scored_cards.push((loc.clone(), card.clone(), score));
                            }
                        }

                        // Find the global minimum score
                        let min_score = scored_cards
                            .iter()
                            .map(|(_, _, score)| *score)
                            .min();

                        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

                        let min_val = min_score.expect(&format!("Found no Minimum in scored_cards '{:?}'", scored_cards));
                        for (loc, card, score) in scored_cards {
                            if score == min_val {
                                result.entry(loc).or_default().push(card);
                            }
                        }

                        result
                    }   
                ),
            str_repr: format!("MINIMUM USING PRECEDENCE {} OF {}", $precname, $cardset.str_repr)
        }
    }};

    (max of $cardset:tt using prec: $precname:literal) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let prec = gd.get_precedence($precname);
                        // Step 1: Gather all cards with their location and score
                        let mut scored_cards: Vec<(LocationRef, Card, usize)> = vec![];

                        for (loc, cards) in &$cardset.get_card_set(gd) {
                            for card in cards {
                                let score = prec.get_card_value_ref(card).expect(&format!("No value in PointMap for Card '{}'", card));
                                scored_cards.push((loc.clone(), card.clone(), score));
                            }
                        }

                        // Step 2: Find the global maximum score
                        let max_score = scored_cards
                            .iter()
                            .map(|(_, _, score)| *score)
                            .max();

                        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

                        let max_val = max_score.expect(&format!("Found no Maximum in scored_cards '{:?}'", scored_cards));
                        for (loc, card, score) in scored_cards {
                            if score == max_val {
                                result.entry(loc).or_default().push(card);
                            }
                        }

                        result
                    }
                ),
            str_repr: format!("MAXIMUM USING PRECEDENCE {} OF {}", $precname, $cardset.str_repr)
        }
    }};

    (min of $cardset:tt using pointmap: $pmname:literal) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let pointmap = gd.get_pointmap($pmname);
                        // First, collect all cards with their location and score
                        let mut scored_cards: Vec<(LocationRef, Card, i32)> = vec![];

                        for (loc, cards) in &$cardset.get_card_set(gd) {
                            for card in cards {
                                let score = pointmap.get_card_value_ref(card).expect(&format!("No value in PointMap for Card '{}'", card));
                                let min = score.iter().min().expect(&format!("Found no Minimum in score '{:?}' of card '{}'", score, card));
                                scored_cards.push((loc.clone(), card.clone(), *min));
                            }
                        }

                        // Find the global minimum score
                        let min_score = scored_cards
                            .iter()
                            .map(|(_, _, score)| *score)
                            .min();

                        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

                        let min_val = min_score.expect(&format!("Found no global Minimum in scored_cards '{:?}'", scored_cards));
                        for (loc, card, score) in scored_cards {
                            if score == min_val {
                                result.entry(loc).or_default().push(card);
                            }
                        }

                        result
                    }
                ),
            str_repr: format!("MINIMUM USING POINTMAP {} OF {}", $pmname, $cardset.str_repr)
        }
    }};

    (max of $cardset:tt using pointmap: $pmname:literal) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let pointmap = gd.get_pointmap($pmname);
                        // Step 1: Gather all cards with their location and score
                        let mut scored_cards: Vec<(LocationRef, Card, i32)> = vec![];

                        for (loc, cards) in &$cardset.get_card_set(gd) {
                            for card in cards {
                                let score = pointmap.get_card_value_ref(card).expect(&format!("No value in PointMap for Card '{}'", card));
                                let max = score.iter().max().expect(&format!("Found no Maximum in '{:?}' of card '{}'", score, card));
                                scored_cards.push((loc.clone(), card.clone(), *max));
                            }
                        }

                        // Step 2: Find the global maximum score
                        let max_score = scored_cards
                            .iter()
                            .map(|(_, _, score)| *score)
                            .max();

                        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

                        let max_val = max_score.expect(&format!("Found no global Maximum in scored_cards '{:?}''", scored_cards));
                        for (loc, card, score) in scored_cards {
                            if score == max_val {
                                result.entry(loc).or_default().push(card);
                            }
                        }

                        result
                    }
                ),
            str_repr: format!("MAXIMUM USING POINTMAP {} OF {}", $pmname, $cardset.str_repr)
        }
    }};

    // location OF player
    ($locname:literal of $pref:expr, $int:literal) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let pname = $pref.get_ref(gd).name;

                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                    
                        let card_map = cardset!($locname)(gd);

                        let i = $int(gd);
                        
                        let cards = card_map.get($locname).expect(&format!("No Location found with name '{}'", $locname));
                        let card = cards.get(i).expect(&format!("No card at index '{}' in Location '{}' of Player '{}'", i, $locname, pname));
                        loc_card.insert(LocationRef::Player(String::from(pname),
                            String::from($locname)),
                            vec![card.clone()]);
                    
                        loc_card
                    }
                ),
            str_repr: format!("{} OF {} AT ", $locname, $pref.str_repr)
        }
    }};

    ($locname:literal of $pref:expr, top) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        use crate::ast::LocationRef;

                        let pname = $pref.get_ref(gd).name;

                        let mut loc_card: HashMap<String, Vec<Card>> = HashMap::new();
                        let card_map = cardset!($locname)(gd);
                        let cards = card_map.get($locname).expect(&format!("No Location found with name '{}'", $locname));
                        let card = cards.get(0).expect(&format!("No card at index 0 in Location '{$locname}' of Player '{$pname}'", $locname, $pname));
                        loc_card.insert(LocationRef::Player(String::from($pname),
                            String::from($locname)),
                            vec![card.clone()]);

                        loc_card
                    }
                ),
            str_repr: format!("TOP OF {} OF {}", $locname, $pref.str_repr)
        }
    }};

    ($locname:literal of $pref:expr, bottom) => {{
        use crate::ast::{GameData, CardPosition};
        use std::sync::Arc;

        CardPosition {
            pos:
                Arc::new(
                    |gd: &GameData| {
                        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();

                        let pname = $pref.get_ref(gd).name;
                    
                        let card_map = cardset!($locname)(gd);
                        let cards = card_map.get($locname).expect(&format!("No Location found with name '{}'", $locname));
                        let len = cards.len();
                        let card = cards.get(len - 1).expect(&format!("No card at index '{}' in Location '{}' of Player '{}'", len - 1, $locname, $pname));
                        loc_card.insert(LocationRef::Player(String::from($pname),
                            String::from($locname)),
                            vec![card.clone()]); 
                    
                        loc_card
                    }
                ),
            str_repr: format!("BOTTOM OF {} OF {}", $locname, $pref.str_repr)
        }
    }};

    // TODO:
    // locations OF team
    // locations OF table
}

// TODO:
// location OF player
// location OF team
// location OF table
macro_rules! cardset {
    ($($locname:literal), *) => {{
        use crate::ast::{GameData, CardSet, LocationRef, Card, str_repr_locations};
        use std::sync::Arc;

        CardSet {
            set: Arc::new(|gd: &GameData| {
                    use std::collections::HashMap;

                    let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                    let locs: Vec<&str> =  vec![$($locname), *];
                    for loc in locs.iter() {
                        let loc_ref = LocationRef::Own(String::from(*loc));
                        let location_ref = gd.get_location(&loc_ref);
                        let location_borrow = location_ref.borrow(); // type: Ref<Location>
                        let cards: &Vec<Card> = location_borrow.get_cards_ref(); // now safe

                        loc_cards.insert(loc_ref, cards.clone());
                    }

                    loc_cards
                }),
            str_repr: format!("{}", str_repr_locations(vec![$($locname), *]))
        }
    }};

    ($($locname:literal), * of player: $pref:expr) => {{
        use crate::ast::{GameData, CardSet, str_repr_locations};
        use std::sync::Arc;

        CardSet {
            set: Arc::new(|gd: &GameData| {
                    use std::collections::HashMap;

                    let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                    let locs: Vec<&str> =  vec![$($locname), *];
                    for loc in locs.iter() {
                        let loc_ref = LocationRef::Player(($pref).get_ref(gd).name.clone(), String::from(*loc));
                        let location = gd.get_location(&loc_ref);
                        let cards = location.borrow().clone().get_cards();
                        loc_cards.insert(loc_ref, cards);
                    }

                    loc_cards
                }),
            str_repr: format!("{} OF {}", str_repr_locations(vec![$($locname), *]), $pref.str_repr)
        }
    }};

    ($($locname:literal), * of team: $tref:expr) => {{
        use crate::ast::{GameData, CardSet, str_repr_locations};
        use std::sync::Arc;

        CardSet {
            set: Arc::new(|gd: &GameData| {
                use std::collections::HashMap;

                let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                let locs: Vec<&str> =  vec![$($locname), *];
                for loc in locs.iter() {
                    let loc_ref = LocationRef::Team(($tref).get_ref(gd).name.clone(), String::from(*loc));
                    let location = gd.get_location(&loc_ref);
                    let cards = location.borrow().clone().get_cards();
                    loc_cards.insert(loc_ref, cards);
                }

                loc_cards
            }),
            str_repr: format!("{} OF {}", str_repr_locations(vec![$($locname), *]), $tref.str_repr)
        }
    }};
    
    // w = where
    ($($locname:literal), * w $f:tt) => {{
        use crate::ast::{GameData, CardSet, str_repr_locations};
        use std::sync::Arc;

        CardSet {
            set: Arc::new(|gd: &GameData| {
                use crate::ast::{LocationRef, Card};
                use std::collections::HashMap;

                let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                let locs: Vec<&str> = vec![$($locname),*];

                for loc in locs.iter() {
                    let location_ref = LocationRef::Own(loc.to_string());
                    let location = gd.get_location(&location_ref);
                    let cards = location.borrow().clone().get_cards(); // Assume this is Vec<Card> or &Vec<Card>
                    
                    // Clone only what's needed
                    let selected_cards: Vec<Card> = $f.apply_func(gd, cards.clone()).into_iter().flatten().collect();

                    // Filter original cards (need to clone here, or avoid re-binding `cards`)
                    let filtered: Vec<Card> = cards
                        .iter()
                        .filter(|card| selected_cards.contains(card))
                        .cloned()
                        .collect();

                    loc_cards.insert(location_ref, filtered);
                }

                loc_cards
            }),
            str_repr: format!("{} WITH {}", str_repr_locations(vec![$($locname), *]), $f.str_repr)
        }
    }};

    ($($locname:literal), * of player: $pref:expr, w $f:tt) => {{
        use crate::ast::{GameData, CardSet, str_repr_locations};
        use std::sync::Arc;

        CardSet {
            set: Arc::new(|gd: &GameData| {
                use crate::ast::LocationRef;
                use std::collections::HashMap;

                let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                let locs: Vec<&str> =  vec![$($locname), *];
                for loc in locs.iter() {
                    let l = gd.get_location(&LocationRef::Player(($pref).get_ref(gd).name.clone(), loc.to_string()));
                    let mut cards = l.borrow().clone().get_cards();
                    let fc: Vec<Card> = $f.apply_func(gd, cards.clone()).into_iter().flatten().collect();
                    cards = cards.into_iter().filter(|card| fc.contains(card)).collect();
                    loc_cards.insert(LocationRef::Player(($pref).get_ref(gd).name.clone(), loc.to_string()),
                        cards
                    );
                }

                loc_cards
            }),
            str_repr: format!("{} OF {} WITH {}", str_repr_locations(vec![$($locname), *]), $pref.str_repr, $f.str_repr)
        }
    }};

    ($($locname:literal), * of team: $tref:expr, w $f:tt) => {{
        use crate::ast::{GameData, CardSet, str_repr_locations};
        use std::sync::Arc;

        CardSet {
            set: Arc::new(|gd: &GameData| {
                use crate::ast::LocationRef;
                use std::collections::HashMap;

                let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                let locs: Vec<&str> =  vec![$($locname), *];
                for loc in locs.iter() {
                    let l = gd.get_location(&LocationRef::Team(($tref).get_ref(gd).name.clone(), loc.to_string()));
                    let mut cards = l.borrow().clone().get_cards();
                    let fc: Vec<Card> = $f.apply_func(gd, cards.clone()).into_iter().flatten().collect();
                    cards = cards.into_iter().filter(|card| fc.contains(card)).collect();
                    loc_cards.insert(LocationRef::Team(($tref).get_ref(gd).name.clone(), loc.to_string()),
                        cards
                    );
                }

                loc_cards
            }),
            str_repr: format!("{} OF {} WITH {}", str_repr_locations(vec![$($locname), *]), $tref.str_repr, $f.str_repr)
        }
    }};

    ($comboname:literal inn $($locname:literal), *) => {{
        use crate::ast::{GameData, CardSet, str_repr_locations};
        use std::sync::Arc;

        CardSet {
            set: Arc::new(|gd: &GameData| {
                use crate::ast::LocationRef;
                use std::collections::HashMap;

                let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                let locs: Vec<&str> =  vec![$($locname), *];
                for loc in locs.iter() {
                    let l = gd.get_location(&LocationRef::Own(loc.to_string()));
                    let mut cards = l.borrow().clone().get_cards();
                    let cardcombo = gd.get_combo($comboname);
                    let cardfun = &cardcombo.attributes;
                    let fc: Vec<Card> = cardfun.apply_func(gd, cards.clone()).into_iter().flatten().collect();
                    cards = cards.into_iter().filter(|card| fc.contains(card)).collect();
                    loc_cards.insert(LocationRef::Own(loc.to_string()),
                        cards
                    );
                }

                loc_cards
            }),
            str_repr: format!("{} IN {}", $comboname, str_repr_locations(vec![$($locname), *]))
        }
    }};

    (not $comboname:literal inn $($locname:literal), *) => {{
        use crate::ast::{GameData, CardSet, str_repr_locations};
        use std::sync::Arc;

        CardSet {
            set: Arc::new(|gd: &GameData| {
                use crate::ast::LocationRef;
                use std::collections::HashMap;

                let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
                let locs: Vec<&str> =  vec![$($locname), *];
                for loc in locs.iter() {
                    let l = gd.get_location(&LocationRef::Own(loc.to_string()));
                    let mut cards = l.borrow().clone().get_cards();
                    let cardcombo = gd.get_combo($comboname);
                    let cardfun = &cardcombo.attributes;
                    let fc: Vec<Card> = cardfun.apply_func(gd, cards.clone()).into_iter().flatten().collect();
                    cards = cards.into_iter().filter(|card| !fc.contains(card)).collect();
                    loc_cards.insert(LocationRef::Own(loc.to_string()),
                        cards
                    );
                }        

                loc_cards
            }),
            str_repr: format!("NOT {} IN {}", $comboname, str_repr_locations(vec![$($locname), *]))
        }
    }};

    ($cardpos:tt) => {{
        use crate::ast::{GameData, CardSet};
        use std::sync::Arc;
        
        CardSet {
            set: Arc::new(|gd: &GameData| {
                use crate::ast::LocationRef;

                let cardpos: HashMap<LocationRef, Vec<Card>> = $cardpos.get_card_position(gd); 
                cardpos
            }),
            str_repr: format!("{}", $cardpos.str_repr)
        }
    }};
}


macro_rules! combo {
    ($name:literal, $filter:expr) => {{
        use crate::ast::GameData;
        use std::sync::Arc;
        
        Arc::new(
            |gd: &mut GameData| {
                use crate::ast::{CardCombination};

                gd.add_cardcombination(
                    $name,
                    CardCombination {
                        name: String::from($name),
                        attributes: $filter, // Ensure Arc wrapping
                    }
                );
            }
        )
    }}
}


// TODO:
// needs to be CardGameModel considering that it needs information from ruleset
macro_rules! int {
    ($int:literal) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |_: &GameData| {
                    let i: i32 = $int;
                    i
                }
            ),
            str_repr: format!("{}", $int)
        }
    }};

    ($int1:expr, $op:literal, $int2:expr) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |gd: &GameData| {
                    let i1: i32 = $int1.get_value_i32(gd);
                    let i2: i32 = $int2.get_value_i32(gd);
                    match $op {
                        "+"   => (i1 + i2),
                        "-"   => (i1 - i2),
                        "*"   => (i1 * i2),
                        "//"  => (i1 / i2),
                        "mod" => (i1 % i2),
                        _ => {
                                println!("{} not defined", $op);
                                0
                            }
                    }
                }
            ),
            str_repr: format!("{} {} {}", $int1.str_repr, $op, $int2.str_repr)
        }
    }};

    ($intcol:expr, $int:tt) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |gd: &GameData| {
                    let index: usize = $int.get_value_usize(gd);
                    $intcol[index]    
                }   
            ),
            // TODO:
            // str_repr: format!("{} AT {}", $intcol, $int)
            str_repr: format!("INTCOLLECTION AT {}", $int.str_repr)
        }
    }};

    // size’ ’of’ [Collection] 
    (size of $col:expr) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |_: &GameData| {
                    $col.len()
                }
            ),
            str_repr: format!("SIZE OF {}", $col)
        }
    }};

    // ’sum’ ’of’ [IntCollection]
    (sum of $intcol:expr) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
            |_: &GameData| {
                    let intcol: Vec<i32> = $intcol;
                    intcol.iter().sum::<i32>()
                }
            ),
            // TODO:
            // str_repr: format!("SUM OF {}", $intcol)
            str_repr: format!("SUM OF INTCOLLECTION")
        }
    }};

    (sum of min $cardset:expr, using $pmname:literal) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |gd: &GameData| {
                    let pmap = &gd.get_pointmap($pmname);
                    
                    let mut sum = 0;

                    let cardset = $cardset.get_card_set(gd);

                    let mut cards = vec![];
                    for (_, cs) in cardset.iter() {
                        for c in cs {
                            cards.push(c);
                        }
                    }

                    for card in cards.iter() {
                        sum += pmap
                            .get_card_value_ref(card)
                            .expect(&format!("No value found in PointMap for Card '{}'", card))
                            .iter()
                            .min()
                            .expect(&format!("No Minimum found in PointMap for Card '{}'", card));
                    }

                    sum
                }
            ),
            str_repr: format!("SUM OF MINIMUM OF {} USING {}", $cardset.str_repr, $pmname)
        }
    }};

    (sum of max $cardset:expr, using $pmname:literal) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |gd: &GameData| {
                    let pmap = &gd.get_pointmap($pmname);
                    
                    let mut sum = 0;

                    let cardset = $cardset.get_card_set(gd);

                    let mut cards = vec![];
                    for (_, cs) in cardset.iter() {
                        for c in cs {
                            cards.push(c);
                        }
                    }


                    for card in cards.iter() {
                        sum += pmap
                            .get_card_value_ref(card)
                            .expect(&format!("No value found in PointMap for Card '{}'", card))
                            .iter()
                            .max()
                            .expect(&format!("No Minimum found in PointMap for Card '{}'", card));
                    }

                    sum
                }
            ),
            str_repr: format!("SUM OF MAXIMUM OF {} USING {}", $cardset.str_repr, $pmname)
        }
    }};

    
    (sum of $cardset:expr, using $pmname:literal gt $int:expr) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |gd: &GameData| {
                    /*
                    [
                        [i11, i12, ...],
                        [i21, i22, ...],
                        ...
                    ]

                    You can only choose 1 Value from each list.
                    Find the minimum of the sum of each chosen value
                    with a boundary: >= value.
                    */

                    fn dfs(
                        matrix: &Vec<Vec<i32>>,
                        row: usize,
                        current_sum: i32,
                        target: i32,
                        min_sum: &mut i32,
                    ) {
                        if row == matrix.len() {
                            if current_sum >= target {
                                *min_sum = (*min_sum).min(current_sum);
                            }
                            return;
                        }
                    
                        for &val in &matrix[row] {
                            // Prune if current sum already worse than best
                            if current_sum + val >= *min_sum {
                                continue;
                            }
                            dfs(matrix, row + 1, current_sum + val, target, min_sum);
                        }
                    }
                    
                    fn min_sum_greater_equal(matrix: Vec<Vec<i32>>, target: i32) -> Option<i32> {
                        let mut min_sum = i32::MAX;
                        dfs(&matrix, 0, 0, target, &mut min_sum);
                        if min_sum == i32::MAX {
                            None
                        } else {
                            Some(min_sum)
                        }
                    }        

                    let pmap = &gd.get_pointmap($pmname);

                    let target = $int.get_value_i32(gd);
                    
                    let mut matrix = vec![];

                    let cardset = $cardset.get_card_set(gd);

                    let mut cards = vec![];
                    for (_, cs) in cardset.iter() {
                        for c in cs {
                            cards.push(c);
                        }
                    }

                    for card in cards.iter() {
                        matrix.push(pmap.get_card_value_ref(&card).expect(&format!("No value found in PointMap for Card '{}'", card)));
                    }

                    min_sum_greater_equal(matrix, target).expect(&format!("Found no Solution for 'min_sum_greater_equal' with value '{}'", target))
                }
            ),
            str_repr: format!("SUM OF {} USING {} >= {}", $cardset.str_repr, $pmname, $int.str_repr)
        }
    }};

    (sum of $cardset:expr, using $pmname:literal lt $int:expr) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |gd: &GameData| {
                    /*
                    [
                        [i11, i12, ...],
                        [i21, i22, ...],
                        ...
                    ]

                    You can only choose 1 Value from each list.
                    Find the minimum of the sum of each chosen value
                    with a boundary: >= value.
                    */
                    
                    fn dfs(
                        matrix: &Vec<Vec<i32>>,
                        row: usize,
                        current_sum: i32,
                        target: i32,
                        min_sum: &mut i32,
                    ) {
                        if row == matrix.len() {
                            if current_sum >= target {
                                *min_sum = (*min_sum).min(current_sum);
                            }
                            return;
                        }
                    
                        for &val in &matrix[row] {
                            // Prune if current sum already worse than best
                            if current_sum + val >= *min_sum {
                                continue;
                            }
                            dfs(matrix, row + 1, current_sum + val, target, min_sum);
                        }
                    }
                    
                    fn min_sum_greater_equal(matrix: Vec<Vec<i32>>, target: i32) -> Option<i32> {
                        let mut min_sum = i32::MAX;
                        dfs(&matrix, 0, 0, target, &mut min_sum);
                        if min_sum == i32::MAX {
                            None
                        } else {
                            Some(min_sum)
                        }
                    }

                    fn negate_vec(vec: Vec<i32>) -> Vec<i32> {
                        vec.iter().map(|x| -x).collect()
                    }        

                    let pmap = &gd.get_pointmap($pmname);

                    // same problem just negate everything
                    let target = - $int.get_value_i32(gd);
                    
                    let cardset = $cardset.get_card_set(gd);

                    let mut cards = vec![];
                    for (_, cs) in cardset.iter() {
                        for c in cs {
                            cards.push(c);
                        }
                    }

                    let mut matrix = vec![];

                    for card in cards.iter() {
                        matrix.push(negate_vec(pmap.get_card_value_ref(&card).expect(&format!("No value found in PointMap for Card '{}'", card))));
                    }

                    - min_sum_greater_equal(matrix, target).expect(&format!("Found no Solution for 'min_sum_greater_equal' with value '{}'", target))
                }
            ),
            str_repr: format!("SUM OF {} USING {} <= {}", $cardset.str_repr, $pmname, $int.str_repr)
        }
    }};

    // TODO:
    // Int-Collection maybe vector of type int! so [int!(1), (int!(cardset(...))), ...]
    // (’min’ | ’max’) ’of’ [IntCollection] 
    (min of $intcol:expr) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |_: &GameData| {
                    *$intcol.iter().min().expect(&format!("No Minimum found in {:?}", $intcol))
                }
            ),
            // TODO:
            // str_repr: format!("MINIMUM OF {}", $intcol)
            str_repr: format!("MINIMUM OF INTCOLLECTION")
        }
    }};

    (max of $intcol:expr) => {{
        use crate::ast::{GameData, GInt};
        use std::sync::Arc;

        GInt {
            value: Arc::new(
                |_: &GameData| {
                    *$intcol.iter().max().expect(&format!("No Maximum found in {:?}", $intcol))
                }
            ),
            // TODO:
            // str_repr: format!("MAXIMUM OF {}", $intcol)
            str_repr: format!("MAXIMUM OF INTCOLLECTION")
        }
    }};

    // TODO: 
    // ’stageroundcounter’ | ’playroundcounter’
}

/*
String → ID | [Key] ’of’ CardPosition | [StringCollection] Int |
    [Key] ’of’ CardPosition
*/
macro_rules! string {
    ($id:literal) => {{
        use crate::ast::{GameData, GString};
        use std::sync::Arc;

        GString {
            string: Arc::new(|_: &GameData| {
                    String::from($id)
                }
            ),
            str_repr: format!("{}", $id)
        }
    }};

    // Problem:
    // there are multiple minima and maxima,
    // so it is not always one card (but should be maybe)
    // let map: HashMap<LocationRef, Vec<Card>> = $cardpos;
    ($key:literal of $cardpos:expr) => {{
        use crate::ast::{GameData, GString};
        use std::sync::Arc;

        GString {
            string: Arc::new(
                |gd: &GameData| {
                    let map = $cardpos.get_card_position(gd);
                    let card = map.iter().next().map(|(_, v)| v[0].clone()).expect("HashMap is empty");

                    String::from(
                        card.clone().attributes.get($key).expect(&format!("No Attribute found with Key '{}' in Card '{}'", $key, card)).clone()
                    )
                }
            ),
            str_repr: format!("{} OF {}", $key, $cardpos.str_repr)
        }
    }};

    ($stringcol:expr, $int:expr) => {{
        use crate::ast::{GameData, GString};
        use std::sync::Arc;

        GString {
            string: Arc::new(
                |gd: &GameData| {
                    let index = $int.get_value_usize(gd);
                    String::from($stringcol[index])
                }
            ),
            // TODO:
            // str_repr: format!("{} AT {}", $stringcol, $int.str_repr)
            str_repr: format!("STRINGCOLLECTION AT {}", $int.str_repr)
        }
    }}; 
}

/*
// Bool == Condition (kind of)
Bool → String (’==’ | ’!=’) String | Int (’==’ | ’!=’ | ’<’ | ’>’ | ’<=’ | ’>=’) Int
    CardSet (’==’ | ’!=’) CardSet | CardSet ’is’ (’not’)? ’empty’ |
    Player (’==’ | ’!=’) Player | Team (’==’ | ’!=’) Team |
    ’(’ Bool (’and’ | ’or’) Bool ’)’ | ’not’ ’(’ Bool ’)’ |
    ([Player] | PlayerCollection) ’out’ ’of’ ([Stage] | ’stage’ | ’play’ | ’game’)
*/
macro_rules! bool {
    (string: $string1:expr, $op:literal, $string2:expr) => {{
        use crate::ast::{CardGameModel, GBool};
        use std::sync::Arc;

        GBool {
            value:
                Arc::new(
                    |cgm: &CardGameModel| {
                        match $op {
                            "!=" => $string1.get_string(&cgm.gamedata) != $string2.get_string(&cgm.gamedata),
                            "==" => $string1.get_string(&cgm.gamedata) == $string2.get_string(&cgm.gamedata),
                            _    => {
                                        println!("Unknown Operator!");
                                        false
                                    }
                        }
                    }
                ),
            str_repr: format!("{} {} {}", $string1.str_repr, $op, $string2.str_repr)
        }
    }};

    (int: $int1:expr, $op:literal, $int2:expr) => {{
        use crate::ast::{CardGameModel, GBool};
        use std::sync::Arc;

        GBool {
            value:
                Arc::new(
                    |cgm: &CardGameModel| {
                        match $op {
                            "==" => $int1.get_value_i32(&cgm.gamedata) == $int2.get_value_i32(&cgm.gamedata),
                            "!=" => $int1.get_value_i32(&cgm.gamedata) != $int2.get_value_i32(&cgm.gamedata),
                            "<"  => $int1.get_value_i32(&cgm.gamedata) <  $int2.get_value_i32(&cgm.gamedata),
                            ">"  => $int1.get_value_i32(&cgm.gamedata) >  $int2.get_value_i32(&cgm.gamedata),
                            "<=" => $int1.get_value_i32(&cgm.gamedata) <= $int2.get_value_i32(&cgm.gamedata),
                            ">=" => $int1.get_value_i32(&cgm.gamedata) >= $int2.get_value_i32(&cgm.gamedata),
                            _    => {
                                        println!("Unknown Operator!");
                                        false
                                    }
                        }
                    }
                ),
            str_repr: format!("{} {} {}", $int1.str_repr, $op, $int2.str_repr)
        }
    }};

    // CardSet (’==’ | ’!=’) CardSet
    (cardset: $cs1:expr, $op:literal, $cs2:expr) => {{
        use crate::ast::{CardGameModel, GBool};
        use std::sync::Arc;

        GBool {
            value:
                Arc::new(
                    |cgm: &CardGameModel| {
                        fn eq(
                            cs1: HashMap<LocationRef, Vec<Card>>,
                            cs2: HashMap<LocationRef, Vec<Card>>,
                        ) -> bool {
                            let cards1: Vec<&Card> = cs1.values().flatten().collect();
                            let cards2: Vec<&Card> = cs2.values().flatten().collect();
                        
                            cards1 == cards2
                        }


                        match $op {
                            "==" =>  eq(($cs1).get_card_set(&cgm.gamedata), ($cs2).get_card_set(&cgm.gamedata)),
                            "!=" => !eq(($cs1).get_card_set(&cgm.gamedata), ($cs2).get_card_set(&cgm.gamedata)),
                            _    => {
                                        println!("Unknown Operator!");
                                        false
                                    }
                        }
                    }
                ),
            str_repr: format!("{} {} {}", $cs1.str_repr, $op, $cs2.str_repr)
        }
    }};

    // CardSet ’is’ (’not’)? ’empty’
    ($cs:expr, is empty) => {{
        use crate::ast::{CardGameModel, GBool};
        use std::sync::Arc;

        GBool {
            value:
                Arc::new(
                    |cgm: &CardGameModel| {
                        let mut isempty = true;
                        for (_, v) in $cs.get_card_set(&cgm.gamedata).iter() {
                            if !v.is_empty() {
                                isempty = false;
                                break;
                            }
                        }

                        isempty
                    }
                ),
            str_repr: format!("{} IS EMPTY", $cs.str_repr)
        }
    }};

    ($cs:expr, is not empty) => {{
        use crate::ast::{CardGameModel, GBool};
        use std::sync::Arc;

        GBool {
            value:
                Arc::new(
                    |cgm: &CardGameModel| {
                        let mut isnotempty = false;
                        for (_, v) in $cs.get_card_set(&cgm.gamedata).iter() {
                            if !v.is_empty() {
                                isnotempty = true;
                                break;
                            }
                        }

                        isnotempty
                    }
                ),
            str_repr: format!("{} IS NOT EMPTY", $cs.str_repr)
        }
    }};

    // Player == Player and Team == Team
    (pt: $ref1:expr, $op:literal, $ref2:expr) => {{
        use crate::ast::{CardGameModel, GBool};
        use std::sync::Arc;

        GBool {
            value:
                Arc::new(
                    |cgm: &CardGameModel| {
                        match $op {
                            "==" => ($ref1).get_ref(&cgm.gamedata) == ($ref2).get_ref(&cgm.gamedata),
                            "!=" => ($ref1).get_ref(&cgm.gamedata) != ($ref2).get_ref(&cgm.gamedata),
                            _    => {
                                        println!("Unknown Operator!");
                                        false
                                    }
                        }
                    }
                ),
            str_repr: format!("{} {} {}", $ref1.str_repr, $op, $ref2.str_repr)
        }
    }};

    // ’(’ Bool (’and’ | ’or’) Bool ’)’ 
    ($b1:expr, $op:literal, $b2:expr) => {{
        use crate::ast::{CardGameModel, GBool};
        use std::sync::Arc;

        GBool {
            value:
                Arc::new(
                    |cgm: &CardGameModel| {
                        match $op {
                            "and" => $b1(cgm) && $b2(cgm),
                            "or"  => $b1(cgm) || $b2(cgm),
                            _     => {
                                        println!("Unknown Operator!");
                                        false
                                    }
                        }
                    }
                ),
            str_repr: format!("{} {} {}", $b1.str_rep, $op, $b2.str_rep)
        }
    }};

    // ’not’ ’(’ Bool ’)’
    (not $b:expr) => {{
        use crate::ast::{CardGameModel, GBool};
        use std::sync::Arc;

        GBool {
            value:
            Arc::new(
                |cgm: &CardGameModel| {
                    !$b(cgm)
                }
            ),
            str_repr: format!("NOT {}", $b1.str_rep)
        }
    }};

    // TODO:
    // ([Player] | PlayerCollection) ’out’ ’of’ ([Stage] | ’stage’ | ’play’ | ’game’)
    //  
    // () => {{

    // }};
}

macro_rules! player_ref {
    // Player → PlayerName | ’current’ | ’next’ | ’previous’ | ’competitor’ | ’Turnorder’
    //      Int | ’owner’ ’of’ (CardPosition | (’highest’ | ’lowest’) [Memory])
    ($pname:literal) => {{
        use crate::ast::{GameData, RefPlayer};
        use std::sync::Arc;

        RefPlayer {
            player: Arc::new(|gd: &GameData| gd.get_player_copy($pname)),
            str_repr: format!("{}", $pname)
        }
    }};

    (current) => {{
        use crate::ast::{GameData, RefPlayer};
        use std::sync::Arc;

        RefPlayer {
            player: 
                Arc::new(|gd: &GameData| {
                    let current = gd.current as usize;
                    let pname   = &gd.turnorder[current];
                    gd.get_player_copy(pname)
                }),
            str_repr: format!("CURRENT PLAYER")
        }
    }};

    (next) => {{
        use crate::ast::{GameData, RefPlayer};
        use std::sync::Arc;

        RefPlayer {
            player: Arc::new(|gd: &GameData| {
                    let current = gd.current as i32;
                    let next    = ((current + 1) % (gd.turnorder.len() as i32)) as usize;
                    let pname   = &gd.turnorder[next];
                    gd.get_player_copy(pname)
                }),
            str_repr: format!("NEXT PLAYER")
        }
    }};

    (previous) => {{
        use crate::ast::{GameData, RefPlayer};
        use std::sync::Arc;

        RefPlayer {
            player: Arc::new(|gd: &GameData| {
                    let current = gd.current as i32;
                    let len = gd.turnorder.len() as i32;
                    let previous    = ((current - 1 + len) % len) as usize;
                    let pname   = &gd.turnorder[previous];
                    gd.get_player_copy(pname)
                }),
            str_repr: format!("PREVIOUS PLAYER")
        }
    }};

    // If we have teams or no teams at all then we have multiple competitors
    // makes not a lot of sense
    // ($gd:expr, competitor) => {{
    //     $gd.playertoteam
    //     $gd.get_player(pname)
    // }};
    
    (turnorder $int:expr) => {{
        use crate::ast::{GameData, RefPlayer};
        use std::sync::Arc;

        RefPlayer {
            player: Arc::new(|gd: &GameData| {
                    let i       = $int.get_value_i32(gd) as i32;
                    let len = gd.turnorder.len() as i32;
                    let index   = ((i - 1 + len) % len) as usize;
                    let pname   = &gd.turnorder[index];
                    gd.get_player_copy(pname)
                }),
            str_repr: format!("{}'S PLAYER", $int.str_repr)
        }
    }};

    // ’owner’ ’of’ CardPosition
    (owner of $cardpos:expr) => {{
        use crate::ast::{GameData, RefPlayer};
        use std::sync::Arc;

        RefPlayer {
            player: Arc::new(|gd: &GameData| {
                    let map = $cardpos.get_card_position(gd);
                    let i     = gd.current as usize;
                    let pname = &gd.turnorder[i];
                    let locowner: LocationRef = map.iter().next().map(|(k, _)| k.clone()).expect("CardPosition is empty");
                    match locowner {
                        LocationRef::Own(_)       => gd.get_player_copy(pname),
                        LocationRef::Player(player, _) => gd.get_player_copy(&player),
                        _                             => {
                            println!("No owner found!");
                            // Placeholder for player return (return current if not found)
                            gd.get_player_copy("")
                        }  
                        // We try to find one player so we ignore teams
                        // LocationRef::Team(tname, _) => $gd.get_player(pname),
                        // LocationRef::Table(pname) => $gd.get_player(pname),
                    }
                }),
            str_repr: format!("OWNER OF {}", $cardpos.str_repr)
        }
    }}

    // TODO:
    // ’owner’ ’of’ (’highest’ | ’lowest’) [Memory]
    
}

// Team → TeamName | ’team’ ’of’ [Player]
macro_rules! team_ref {
    ($tname:literal) => {{
        use crate::ast::{GameData, RefTeam};
        use std::sync::Arc;

        RefTeam {
            team:
                Arc::new(|gd: &GameData| {
                    gd.get_team_copy($tname)
                }),
            str_repr: format!("{}", $tname)
        }
    }};

    (team of $pref:expr) => {{
        use crate::ast::{GameData, RefTeam};
        use std::sync::Arc;

        RefTeam {
            team:
                Arc::new(|gd: &GameData| {
                    use crate::ast::Player;
                    let player: Player = ($pref).get_ref(gd);
                    let pname: &str = &player.name;
                    let tname = gd.playertoteam.get(pname).expect(&format!("No Player with name: {} in 'playertoteam'", pname));
                    gd.get_team_copy(tname)
                }),
            str_repr: format!("TEAM OF {}", $pref.str_repr)
        }
    }};
}


// ActionRule → FlipAction |ShuffleAction | MoveAction | MemoryAction | CycleAction |
//              OutAction | EndAction | DemAction

// TODO:
// Status


// ’until’ Bool ((’and’ | ’or’) Repetitions)? | Repetitions | ’until’ ’end’
macro_rules! endcondition {
    (until $b:expr) => {{
        use crate::ast::{CardGameModel, EndCondition};
        use std::sync::Arc;

        EndCondition {
            condition: Arc::new(
                |cgm: &CardGameModel, _: usize| {
                    $b.get_value(cgm)
                }
            ),
            str_repr: format!("{}", $b.str_repr)
        }
    }};

    (until $b:literal and $int:expr, times) => {{
        use crate::ast::{CardGameModel, EndCondition};
        use std::sync::Arc;

        EndCondition {
            condition: Arc::new(
                |cgm: &CardGameModel, rep: usize| {
                    $b.get_value(cgm) && rep == $int(&cgm.gamedata)
                }
            ),
            str_repr: format!("UNTIL {} AND {} TIMES", $b.str_repr, $int.str_repr)
        }
    }};

    (until $b:literal or $int:expr, times) => {{
        use crate::ast::{CardGameModel, EndCondition};
        use std::sync::Arc;

        EndCondition {
            condition: Arc::new(
                |cgm: &CardGameModel, rep: usize| {
                    $b.get_value(cgm) || rep == $int(&cgm.gamedata)
                }
            ),
            str_repr: format!("UNTIL {} OR {} TIMES", $b.str_repr, $int.str_repr)
        }
    }};

    (until $b:expr, and once) => {{
        use crate::ast::{CardGameModel, EndCondition};
        use std::sync::Arc;

        EndCondition {
            condition: Arc::new(
                |cgm: &CardGameModel, rep: usize| {
                    $b.get_value(cgm) && rep == 1
                }
            ),
            str_repr: format!("UNTIL {} AND ONCE", $b.str_repr)
        }
    }};

    (until $b:expr, or once) => {{
        use crate::ast::{CardGameModel, EndCondition};
        use std::sync::Arc;

        EndCondition {
            condition: Arc::new(
                |cgm: &CardGameModel, rep: usize| {
                    $b.get_value(cgm) || rep == 1
                }
            ),
            str_repr: format!("UNTIL {} OR ONCE", $b.str_repr)
        }
    }};

    ($int:expr, times) => {{
        use crate::ast::{EndCondition};
        use std::sync::Arc;
        EndCondition {
            condition: Arc::new(
                |cgm: &CardGameModel, rep: usize| {
                    rep == ($int(&cgm.gamedata)as usize)
                }
            ),
            str_repr: format!("{} TIMES", $int.str_repr)
        }
    }};

    (once) => {{
        use crate::ast::{EndCondition};
        use std::sync::Arc;
        EndCondition {
            condition: Arc::new(
                |_: &CardGameModel, rep: usize| {
                    rep == 1
                }
            ),
            str_repr: format!("ONCE")
        }
    }};

    // Stage is only being ended by other factors, like:
    // all players are out of stage, winner is declared in Stage, etc...
    (untilend) => {{
        use crate::ast::{EndCondition};
        use std::sync::Arc;

        EndCondition {
            condition: Arc::new(|_: &CardGameModel, _: usize| {
                false
            }),
            str_repr: format!("UNTIL END")
        }
    }};
}

// seq-stage
// SeqStage -> ’Stage’ Stage ’for’ [Player] EndCondition ’:’ (’create’ SetupRule | PlayRule |
//      ScoringRule)+ ’}’
macro_rules! stage {
    (
        stage $stage_name:literal $pref:expr, $end_cond:expr,
        substages: ($($stage:expr),* )
        rules: ( $( $rule:expr ),* )
        $(,)?
    ) => {{
        use crate::ast::Stage;
        |cgm: &mut CardGameModel| {
            let mut stage = Stage::new($stage_name);
            stage.set_player_reference($pref);
            stage.add_end_condition($end_cond);

            $(
                stage.add_sub_stage($stage);
            )*

            // $(
            //     stage.add_setup_rule($setup_rule);
            // )*

            // $(
            //     stage.add_play_rule($play_rule);
            // )*

            // $(
            //     stage.add_scoring_rule($scoring_rule);
            // )*

            $(
                stage.add_rule($rule);
            )*

            cgm.ruleset.play.add_stage(stage);
        }
    }};
}

// substage (Maybe change later if possible)
macro_rules! substage {
    (
        stage $stage_name:literal $pref:expr, $end_cond:expr,
        substages: ($($stage:expr),* )
        rules: ( $( $rule:expr ),* )
        $(,)?
    ) => {{
        use crate::ast::Stage;
        let mut stage = Stage::new($stage_name);
        stage.set_player_reference($pref);
        stage.add_end_condition($end_cond);

        $(
            let sub = $stage(cgm);
            stage.add_sub_stage(sub);
        )*

        // $(
        //     stage.add_setup_rule($setup_rule);
        // )*

        // $(
        //     stage.add_play_rule($play_rule);
        // )*

        // $(
        //     stage.add_scoring_rule($scoring_rule);
        // )*

        $(
            stage.add_rule($rule);
        )*

        stage
    }};
}

macro_rules! condrule {
    (
        (conditional:
            $(
                (case: $b:tt ( $($rule:tt)+ ))
            )+
        )
    ) => {{
        use crate::ast::{Condition, Rule, ConditionalRule, ConditionalCase, str_repr_rules};

        let mut string_conditional = format!("conditional:\n");

        ConditionalRule {
            condcases: vec![
                $(
                    let vec = vec![$($rule),+];

                    let repr = format!("case {}:\n{}", $b.str_repr, str_repr_rules(&vec));

                    ConditionalCase {
                        condition: Condition { condition: $b},
                        str_repr: repr,
                        rules: vec,
                    }

                    string_conditional = format!("{}{}", string_conditional, repr)
                ),+
            ],
            str_repr: string_conditional,
        }
    }};
}

macro_rules! ifrule {
    (iff $b:tt then $( $rule:expr ),* ) => {{
        use crate::ast::{Rule, IfRule, PlayRule, Condition, str_repr_if_rule};

        let vec = vec![$($rule),*];

        Rule::PLAYRULE(PlayRule::IFRULE(
            IfRule {
                condition: Condition {
                    condition: $b,
                    str_repr: $b.str_repr
                },
                str_repr: str_repr_if_rule(&vec, $b.str_repr),
                rules: vec,
            }
        ))
    }}
}

macro_rules! oprule {
    (optional: $( $rule:expr ),*) => {{
        use crate::ast::{OptionalRule, str_repr_optional_rule};

        let vec = vec![$($rule),*],

        Rule::PLAYRULE(PlayRule::OPTIONALRULE(
            OptionalRule {
                str_repr: str_repr_optional_rule(&vec),
                rules: vec,
            }
        ))
    }}
}

macro_rules! chooserule {
    (choose: $prule1:tt $(or: $prule2:tt)*) => {{
        use crate::ast::{Rule, PlayRule, ChooseRule, str_repr_choose_rule};
        
        let mut vec = vec![$prule1];
        $(
            vec.push($prule2);
        )*

        Rule::PLAYRULE(PlayRule::CHOOSERULE(
            ChooseRule {
                str_repr: str_repr_choose_rule(&vec),
                rules: vec,
            }
        ))
    }}
}

macro_rules! triggerrule {
    (trigger: $( $rule:expr ),*) => {
        use crate::ast::{TriggerRule, str_repr_rules};

        let vec = vec![$($rule),*];

        Rule::PLAYRULE(PlayRule::CHOOSERULE(
            TriggerRule {
                str_repr: format!("TRIGGER:\n{}", str_repr_rules(&vec)),
                rules: vec,
            }
        ))
    }
}

macro_rules! actionrule {
    // ------------------------------------------------------------------------
    // MOVEACTION
    // ==========
    // TODO:
    // Status
    (mv $fromcs:tt to $tocs:tt) => {{
        use crate::ast::{CardGameModel, TMoveCardSet, Action, MoveCSAction, ActionRule, Rule, PlayRule};
        use std::sync::Arc;

        // Create a boxed closure.
        let move_cardset_closure: TMoveCardSet = Arc::new(
            move |cgm: &mut CardGameModel| {

                let fromcs = $fromcs.get_card_set(&cgm.gamedata);
                let tocs = $tocs.get_card_set(&cgm.gamedata);

                cgm.gamedata.move_cardset(fromcs, tocs);

            }
        );

        Rule::PLAYRULE(PlayRule::ACTIONRULE(ActionRule {
            action: Action::MoveCardSet(MoveCSAction { action: move_cardset_closure }),
            str_repr: format!("MOVE {} TO {}", $fromcs.str_repr, $tocs.str_repr)
        }))
    }};
    
    // ClassicMove → ’move’ (Quantity (’from’)?)? CardSet Status (’bound’)? ’to’ CardSet
    // move X from <from> to <to>
    (mv $q:literal from $fromcs:tt to $tocs:tt) => {{
        // use crate::ast::{CardGameModel, GameData, TMoveCards};
        use crate::ast::{CardGameModel, TMoveCards, Action, MoveAction, ActionRule, Rule, PlayRule};
        use std::sync::Arc;

        // Create a boxed closure that takes a mutable GameData.
        let move_cards_closure: TMoveCards = 
            Arc::new(
                move |cgm: &mut CardGameModel| {
                     // Capture the literal q.
                    let q_value = $q;

                    let fromcs = $fromcs.get_card_set(&cgm.gamedata);
                    let tocs = $tocs.get_card_set(&cgm.gamedata);

                    cgm.gamedata.move_q_cards(q_value, fromcs, tocs)
                }
            );

        Rule::PLAYRULE(PlayRule::ACTIONRULE(ActionRule {
            action: Action::Move(MoveAction { action: move_cards_closure }),
            str_repr: format!("MOVE {} CARDS FROM {} TO {}", $q, $fromcs.str_repr, $tocs.str_repr)
        }))
    }};

    (deal $q:literal from $fromcs:tt to $tocs:tt) => {
        {
            use crate::ast::{CardGameModel, TMoveCards, Action, DealAction, ActionRule, PlayRule, Rule};
            use std::sync::Arc;

            // Create a boxed closure that takes a mutable GameData.
            let deal_cards_closure: TMoveCards = Arc::new(
                move |cgm: &mut CardGameModel| {
                    // Capture the literal q.
                    let q_value = $q;

                    let fromcs = $fromcs.get_card_set(&cgm.gamedata);
                    let tocs = $tocs.get_card_set(&cgm.gamedata);

                    cgm.gamedata.deal_q_cards(q_value, fromcs, tocs)
                },
            );
            Rule::PLAYRULE(PlayRule::ACTIONRULE(ActionRule{
                action: Action::Deal(DealAction { action: deal_cards_closure }),
                str_repr: format!("DEAL {} CARDS FROM {} TO {}", $q, $fromcs.str_repr, $tocs.str_repr)
            }))
        }
    };
    
    (deal $fromcs:tt to $tocs:tt) => {{
        use crate::ast::{CardGameModel, TMoveCardSet, Action, MoveCSAction, ActionRule, Rule, PlayRule};
        use std::sync::Arc;

        // Create a boxed closure.
        let move_cardset_closure: TMoveCardSet = Arc::new(
            move |cgm: &mut CardGameModel| {

                let fromcs = $fromcs.get_card_set(&cgm.gamedata);
                let tocs = $tocs.get_card_set(&cgm.gamedata);

                cgm.gamedata.move_cardset(fromcs, tocs);

            }
        );

        Rule::PLAYRULE(PlayRule::ACTIONRULE(ActionRule {
            action: Action::MoveCardSet(MoveCSAction { action: move_cardset_closure }),
            str_repr: format!("DEAL {} TO {}", $fromcs.str_repr, $tocs.str_repr)
        }))
    }};
    
    // TODO:
    // ExchangeMove → ’exchange’ (Quantity (’from’)?)? CardSet ’with’ CardSet
    ($cgm:expr, exchange $q:literal from $fromcs:tt with $tocs:tt) => {{
        
    }};

    ($cgm:expr, exchange $q:literal $fromcs:tt with $tocs:tt) => {{
        
    }};

    ($cgm:expr, exchange $fromcs:tt with $tocs:tt) => {{
        
    }};
}

// EndAction → ’end’ (’turn’ | ’stage’ | ’play’ | ’game’ ’with’ ’winner’ ([Player] | PlayerCol-
// lection))
macro_rules! endaction {
    (end turn) => {{
        use crate::ast::{Rule, PlayRule, Action, ActionRule};

        Rule::PLAYRULE(PlayRule::ACTIONRULE(
            ActionRule { 
                action: Action::EndTurn,
                str_repr: format!("END TURN"),
            }
        ))
    }};

    (end stage) => {{
        use crate::ast::{Rule, PlayRule, Action, ActionRule};

        Rule::PLAYRULE(PlayRule::ACTIONRULE(
            ActionRule {
                action: Action::EndStage,
                str_repr: format!("END TURN"),
            }
        ))
    }};
    
    (end play) => {{
        use crate::ast::CardGameModel;

        Rule::PLAYRULE(PlayRule::ACTIONRULE(
            ActionRule {
                action: Action::EndPlay
                str_repr: format!("END TURN"),
            }
        ))
    }};

    (end game with winner $pref:expr) => {{
        use crate::ast::CardGameModel;
        
        Rule::PLAYRULE(PlayRule::ACTIONRULE(
            ActionRule {
                action: Action::EndGame
                str_repr: format!("END TURN"),
            }
        ))
    }};

    // TODO:
    // PlayerCollection
    // (end game with winner $pref:expr) => {{
    //     use crate::ast::CardGameModel;
    //     Arc::new(
    //         |cgm: &CardGameModel| {
    //             // do something
    //         }
    //     )
    // }};    
}

macro_rules! cycleaction {
    (cycle to $pref:expr) => {{
        use crate::ast::{CycleAction, Rule, PlayRule, ActionRule, Action};    
    
        Rule::PLAYRULE(PlayRule::ACTIONRULE(ActionRule {
            action: Action::CycleAction(
                CycleAction {
                    pref: $pref
                }
            ),
            str_repr: format!("CYCLE TO {}", $pref.str_repr)
        }))
    }};
}

macro_rules! shuffleaction {
    (shuffle $cs:expr) => {{
        use crate::ast::{Rule, PlayRule, ActionRule, Action, ShuffleAction};

        Rule::PLAYRULE(PlayRule::ACTIONRULE(ActionRule {
            action: Action::ShuffleAction(
                ShuffleAction {
                    cardset: $cs
                }
            ),
            str_repr: format!("SHUFFLE {}", $cs.str_repr)
            })
        )
    }};
}

/*
OutAction → ’set’ ([Player] | PlayerCollection) ’out’ ’of’ ( ’stage’ | ’play’ | ’game’ (’suc-
cessful’ | ’fail’))
*/
macro_rules! outaction {
    // TODO:
    (set $pref:expr, out of stage) => {{
        use crate::ast::{Rule, PlayRule, Action, ActionRule, OutAction, OutOf};

        Rule::PLAYRULE(PlayRule::ACTIONRULE(
            ActionRule { action: Action::OutAction(
                    OutAction {
                        pref: $pref,
                        outof: OutOf::Stage,
                    }
                ),
                str_repr: format!("SET {} OUT OF STAGE", $pref.str_repr)
            }
        ))
    }};

    // TODO:
    (set $pref:expr, out of play) => {{
        use crate::ast::{Rule, PlayRule, Action, ActionRule, OutAction, OutOf};

        Rule::PLAYRULE(PlayRule::ACTIONRULE(
            ActionRule { action: Action::OutAction(
                    OutAction {
                        pref: $pref,
                        outof: OutOf::Play,
                    }
                ),
                str_repr: format!("SET {} OUT OF PLAY", $pref.str_repr)
            }
        ))
    }};

    // TODO:
    (set $pref:expr, out of game successful) => {{
        use crate::ast::{Rule, PlayRule, Action, ActionRule, OutAction, OutOf};

        Rule::PLAYRULE(PlayRule::ACTIONRULE(
            ActionRule { action: Action::OutAction(
                    OutAction {
                        pref: $pref,
                        outof: OutOf::GameSuccessful,
                    }
                ),
                str_repr: format!("SET {} OUT OF GAME SUCCESSFUL", $pref.str_repr)
            }
        ))
    }};

    // TODO:
    (set $pref:expr, out of game fail) => {{
        use crate::ast::{Rule, PlayRule, Action, ActionRule, OutAction, OutOf};

        Rule::PLAYRULE(PlayRule::ACTIONRULE(
            ActionRule { action: Action::OutAction(
                    OutAction {
                        pref: $pref
                        outof: OutOf::GameFail,
                    }
                ),
                str_repr: format!("SET {} OUT OF GAME FAIL", $pref.str_repr)
            }
        ))
    }};
}


/*
ScoringRule → ScoreRule | WinnerRule
ScoreRule → ’score’ Int (’to’ [Memory])? ’of’ ([PlayerName] | PlayerCollection)
WinnerRule → ’winner’ ’is’ ([PlayerName] | PlayerCollection) | (’lowest’ | ’highest’) (’Score’
    | ’Position’ | [Memory])
*/
macro_rules! scoringrule {
    // ScoreRule → ’score’ Int ’of’ [Player]
    (set score $int:expr, of $pref:expr) => {{
        use crate::ast::{Rule, ScoringRule, ScoreRule};

        Rule::SCORINGRULE(
            ScoringRule::Score(ScoreRule {
            set: true,
            score: $int,
            pref: $pref,
            str_repr: format!("SET SCORE {} OF {}", $int.str_repr, $pref.str_repr)
        }))
    }};

    (add score $int:expr, of $pref:expr) => {{
        use crate::ast::{Rule, ScoringRule, ScoreRule};

        Rule::SCORINGRULE(
            ScoringRule::Score(ScoreRule {
            set: false,
            score: $int,
            pref: $pref,
            str_repr: format!("ADD SCORE {} OF {}", $int.str_repr, $pref.str_repr)
        }))
    }};
}

macro_rules! winnerrule {
    (winner is $pref:expr) => {{
        use crate::ast::{Rule, ScoringRule, WinnerRule};

        Rule::SCORINGRULE(
            ScoringRule::Winner(WinnerRule {
            winner: $pref,
            str_repr: format!("WINNER IS {}", $pref.str_repr)
        }))
    }};

    (lowest score) => {{
        use crate::ast::{Rule, ScoringRule, WinnerRule};
        use std::sync::Arc;

        Rule::SCORINGRULE(
            ScoringRule::Winner(
                WinnerRule {
            winner: Arc::new(
                |cgm: &CardGameModel| {
                    let mut lowest_scoring_player: Option<(&String, i32)> = None;

                    for p in cgm.gamedata.turnorder.iter() {
                        let player = cgm.gamedata.get_player(p);
                        let score = player.score;

                        match lowest_scoring_player {
                            Some((_, min_score)) if score < min_score => {
                                lowest_scoring_player = Some((p, score));
                            }
                            None => {
                                lowest_scoring_player = Some((p, score));
                            }
                            _ => {}
                        }
                    }

                    if let Some((player_name, score)) = lowest_scoring_player {
                        println!("Player with lowest score: {} (score: {})", player_name, score);
                        cgm.gamedata.get_player(player_name)
                    } else {
                        // TODO:
                        // Default value (if there is no Player with highest score)
                        cgm.gamedata.get_current()
                    }
                }
            ),
            str_repr: String::from(""),
        })),
            str_repr: format!("WINNER IS THE PLAYER WITH THE LOWEST SCORE")
    }};

    (highest score) => {{
        use crate::ast::{Rule, ScoringRule, WinnerRule};
        use std::sync::Arc;

        Rule::SCORINGRULE(
            ScoringRule::Winner(
                WinnerRule {
            winner: Arc::new(
                |cgm: &CardGameModel| {
                    let mut highest_scoring_player: Option<(&String, i32)> = None;

                    for p in cgm.gamedata.turnorder.iter() {
                        let player = cgm.gamedata.get_player(p);
                        let score = player.score;

                        match highest_scoring_player {
                            Some((_, max_score)) if score > max_score => {
                                highest_scoring_player = Some((p, score));
                            }
                            None => {
                                highest_scoring_player = Some((p, score));
                            }
                            _ => {}
                        }
                    }

                    if let Some((player_name, score)) = highest_scoring_player {
                        println!("Player with highest score: {} (score: {})", player_name, score);
                        cgm.gamedata.get_player(player_name)
                    } else {
                        // TODO:
                        // Default value (if there is no Player with highest score)
                        cgm.gamedata.get_current()
                    }
                }
            ),
            str_repr: format!("WINNER IS THE PLAYER WITH THE HIGHEST SCORE")
        }))
    }};

    (highest score lt $int:expr) => {{
        use crate::ast::{Rule, ScoringRule, WinnerRule};
        use std::sync::Arc;

        Rule::SCORINGRULE(
            ScoringRule::Winner(
                WinnerRule {
            winner: Arc::new(
                |cgm: &CardGameModel| {
                    let mut highest_scoring_player: Option<(&String, i32)> = None;

                    let boundary = $int.get_value_i32(&cgm.gamedata);

                    for p in cgm.gamedata.turnorder.iter() {
                        let player = cgm.gamedata.get_player(p);
                        let score = player.score;

                        match highest_scoring_player {
                            Some((_, max_score)) if score > max_score && score <= boundary => {
                                highest_scoring_player = Some((p, score));
                            }
                            None if score <= boundary => {
                                highest_scoring_player = Some((p, score));
                            }
                            _ => {}
                        }
                    }

                    if let Some((player_name, score)) = highest_scoring_player {
                        println!("Player with highest score: {} (score: {})", player_name, score);
                        cgm.gamedata.get_player(player_name)
                    } else {
                        println!("NO PLAYER WON!");
                        // TODO:
                        // Default value (if there is no Player with highest score)
                        cgm.gamedata.get_current()
                    }
                }
            ),
            str_repr: format!("WINNER IS THE PLAYER WITH THE HIGHEST SCORE SMALLER THAN {}", $int.str_repr)
        }))
    }};
}




