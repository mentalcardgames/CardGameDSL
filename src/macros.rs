
// $gd = gamedata
macro_rules! player {
    ($gd:expr, $($n:expr), *) => {
        {
            $gd.add_players(vec![$($n), *])
        }
    }
}

macro_rules! team {
    ($gd:expr, $n:expr, ($($p:expr), *)) => {
        {
            $gd.add_team($n, vec![$($p), *]);
        }
    };
}

macro_rules! location_on {
    ($gd:expr, $location:literal, players: $($p:expr), *) => {
        {
            for p in vec![$($p),*] {
                $gd.add_loc_player($location, p);
            }
        }
    };

    ($gd:expr, $location:literal, team: $team:expr) => {
        {
            $gd.add_loc_team($location, $team);
        }
    };
    ($gd:expr, $location:literal, table) => {
        {
            $gd.add_loc_table($location);
        }
    };
}

macro_rules! card_on {
    (
        $gd:expr,
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
        use crate::ast::Component;
        use std::collections::HashMap;
        use std::collections::BTreeSet;
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

        // iterate over every player, team and table
        // then assign the cards to the correct location
        let locs = $gd.get_mut_locs($location).unwrap();
        let comp_card: Vec<Component> = all_cards.into_iter().map(|c| Component::CARD(c)).collect();
        for i in 0..locs.len() {
            locs[i].borrow_mut().contents.extend(comp_card.clone());
        }
    }};
}

macro_rules! precedence {
    (
        $gd:expr,
        $name:expr, // Name of the attribute for context
        ($($value:expr),* $(,)?)
        // TODO: add [key, value] Precedence!
    ) => {{
        use crate::ast::Precedence;
        use std::collections::HashMap;
        let mut precedence_map = HashMap::new();
        let mut index = 0;
        $(
            precedence_map.insert($value.to_string(), index);
            index += 1;
        )*
        // println!("Precedence for {}: {:?}", $name, precedence_map);
        $gd.add_precedence(Precedence { name: $name.to_string(), attributes: precedence_map});
    }};
}

macro_rules! pointmap {
    (
        $gd:expr,
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
        $gd.add_pointmap(PointMap { name: $pmapname.to_string(), entries: point_map});
        // Modify gamedata
        // $cgm.gamedata.add_pointmap(PointMap {
        //     name: format!("{}", stringify!($($name1),*)), // Handle multiple `$name1`
        //     entries: point_map.clone(), // Return a copy if needed
        // });
    }};
}

macro_rules! turn_order {

    ($gd:expr, random) => {{
        use rand::seq::SliceRandom;

        // DO NOT CLONE THE REFERENCE
        let mut turn_order: Vec<String> = $gd.players.keys().cloned().collect();
        let mut rng = rand::thread_rng();
        turn_order.shuffle(&mut rng);
        $gd.set_turnorder(turn_order);
    }};

    ($gd:expr, ($($pname:expr),*)) => {{
        $gd.set_turnorder(vec![$(String::from($pname)),*]);
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
        let filter1 = filter!($($filter1)+);
        let filter2 = filter!($($filter2)+);
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            // Apply first filter, keep as Vec<Vec<Card>>
            let filtered1 = filter1(cards.clone());

            match $logical {
                "and" => {
                    // Apply filter2 to each group individually, keep non-empty results
                    filtered1
                        .into_iter()
                        .flat_map(|group| {
                            filter2(group)
                                .into_iter()
                                .filter(|g| !g.is_empty())
                        })
                        .collect()
                }
                "or" => {
                    let mut all_groups: Vec<Vec<Card>> = vec![];

                    // Collect all groups from both filters
                    for group in filter1(cards.clone()) {
                        if !group.is_empty() && !all_groups.contains(&group) {
                            all_groups.push(group);
                        }
                    }
                    for group in filter2(cards) {
                        if !group.is_empty() && !all_groups.contains(&group) {
                            all_groups.push(group);
                        }
                    }

                    all_groups
                }
                _ => panic!("Invalid logical operator: {}", $logical),
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
    ($gd:expr, ($key:literal "adjacent" using $precedence_map:literal)) => {{
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
            
        let precedence_map = &$gd
            .get_precedence($precedence_map)
            .unwrap()
            .attributes;

        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            group_by_adjacent(cards, $key, precedence_map)
        }
    }};


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

    ($key:literal, $comparison:literal, $value:literal) => {{
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            match $comparison {
                "==" => vec![cards.into_iter().filter(|card| card.attributes[$key] == $value).collect()],
                "!=" => vec![cards.into_iter().filter(|card| card.attributes[$key] != $value).collect()],
                _ => panic!("Invalid comparison operator: {}", $comparison),
            }
        }
    }};

    ($gd:expr, $comboname:literal) => {
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            use std::ops::Deref;

            let cardcombo = $gd.get_combo($comboname).unwrap();
            let cardfun: &CardFunction = &cardcombo.attributes;
            cardfun.deref()(cards)
        }
    };

    ($gd:expr, not $comboname:literal) => {{
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            use std::ops::Deref;

            let cardcombo = $gd.get_combo($comboname).unwrap();
            let cardfun: &CardFunction = &cardcombo.attributes;
            let filtered_out: Vec<Card> = {
                let mut seen = Vec::new();
                for card in cardfun.deref()(cards.clone()).into_iter().flatten() {
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
    }};
}

macro_rules! cardposition {
    ($gd:expr, $locname:literal $int:literal) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
        let card_map = cardset!($gd, $locname);
        let cards = card_map.get(&LocationRef::Own($locname.to_string())).unwrap();
        let card = cards.get($int).unwrap();
        loc_card.insert(LocationRef::Own($locname.to_string()), 
            vec![card.clone()]);
    
        loc_card
    }};

    ($gd:expr, $locname:literal top) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
        let card_map = cardset!($gd, $locname);
        let cards = card_map.get(&LocationRef::Own($locname.to_string())).unwrap();
        let card = cards.get(0).unwrap();
        loc_card.insert(LocationRef::Own($locname.to_string()), vec![card.clone()]);
    
        loc_card
    }};

    ($gd:expr, $locname:literal bottom) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
        let card_map = cardset!($gd, $locname);
        let cards = card_map.get(&LocationRef::Own($locname.to_string())).unwrap();
        let len = cards.len();
        // TODO:
        // That has to be handled later,
        // because what if the location is empty???
        let card = cards.get(len - 1).unwrap();
        loc_card.insert(LocationRef::Own($locname.to_string()),
            vec![card.clone()]);
    
        loc_card
    }};

    ($gd:expr, min of $cardset:tt using prec: $precname:literal) => {{
        use crate::ast::LocationRef;

        let prec = $gd.get_precedence($precname).unwrap();
        // First, collect all cards with their location and score
        let mut scored_cards: Vec<(LocationRef, Card, usize)> = vec![];

        for (loc, cards) in &$cardset {
            for card in cards {
                let score = prec.get_card_value_ref(card).unwrap();
                scored_cards.push((loc.clone(), card.clone(), score));
            }
        }

        // Find the global minimum score
        let min_score = scored_cards
            .iter()
            .map(|(_, _, score)| *score)
            .min();

        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        let min_val = min_score.unwrap();
        for (loc, card, score) in scored_cards {
            if score == min_val {
                result.entry(loc).or_default().push(card);
            }
        }

        result        
    }};

    ($gd:expr, max of $cardset:tt using prec: $precname:literal) => {{
        use crate::ast::LocationRef;

        let prec = $gd.get_precedence($precname).unwrap();
        // Step 1: Gather all cards with their location and score
        let mut scored_cards: Vec<(LocationRef, Card, usize)> = vec![];

        for (loc, cards) in &$cardset {
            for card in cards {
                let score = prec.get_card_value_ref(card).unwrap();
                scored_cards.push((loc.clone(), card.clone(), score));
            }
        }

        // Step 2: Find the global maximum score
        let max_score = scored_cards
            .iter()
            .map(|(_, _, score)| *score)
            .max();

        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        let max_val = max_score.unwrap();
        for (loc, card, score) in scored_cards {
            if score == max_val {
                result.entry(loc).or_default().push(card);
            }
        }

        result
    }};

    ($gd:expr, min of $cardset:tt using pointmap: $pmname:literal) => {{
        use crate::ast::LocationRef;

        let pointmap = $gd.get_pointmap($pmname).unwrap();
        // First, collect all cards with their location and score
        let mut scored_cards: Vec<(LocationRef, Card, i32)> = vec![];

        for (loc, cards) in &$cardset {
            for card in cards {
                let score = pointmap.get_card_value_ref(card).unwrap();
                let min = score.iter().min().unwrap();
                scored_cards.push((loc.clone(), card.clone(), *min));
            }
        }

        // Find the global minimum score
        let min_score = scored_cards
            .iter()
            .map(|(_, _, score)| *score)
            .min();

        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        let min_val = min_score.unwrap();
        for (loc, card, score) in scored_cards {
            if score == min_val {
                result.entry(loc).or_default().push(card);
            }
        }

        result
    }};

    ($gd:expr, max of $cardset:tt using pointmap: $pmname:literal) => {{
        use crate::ast::LocationRef;

        let pointmap = $gd.get_pointmap($pmname).unwrap();
        // Step 1: Gather all cards with their location and score
        let mut scored_cards: Vec<(LocationRef, Card, i32)> = vec![];

        for (loc, cards) in &$cardset {
            for card in cards {
                let score = pointmap.get_card_value_ref(card).unwrap();
                let max = score.iter().max().unwrap();
                scored_cards.push((loc.clone(), card.clone(), *max));
            }
        }

        // Step 2: Find the global maximum score
        let max_score = scored_cards
            .iter()
            .map(|(_, _, score)| *score)
            .max();

        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        let max_val = max_score.unwrap();
        for (loc, card, score) in scored_cards {
            if score == max_val {
                result.entry(loc).or_default().push(card);
            }
        }

        result
    }};

    // location OF player
    ($gd:expr, $locname:literal of $pname:literal $int:literal) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
    
        let card_map = cardset!($gd, $locname);
        
        let cards = card_map.get($locname).unwrap();
        let card = cards.get($int).unwrap();
        loc_card.insert(LocationRef::Player(String::from($pname),
            String::from($locname)),
            vec![card.clone()]);
    
        loc_card
    }};

    ($gd:expr, $locname:literal of $pname:literal top) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<String, Vec<Card>> = HashMap::new();
        let card_map = cardset!($gd, $locname);
        let cards = card_map.get($locname).unwrap();
        let card = cards.get(0).unwrap()
        loc_card.insert(LocationRef::Player(String::from($pname),
            String::from($locname)),
            vec![card.clone()]);

        loc_card
    }};

    ($gd:expr, $locname:literal of $pname:literal bottom) => {{
        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
    
        let card_map = cardset!($gd, $locname);
        let cards = card_map.get($locname).unrwap();
        let len = cards.len();
        let card = cards.get(len - 1).unwrap();
        loc_card.insert(LocationRef::Player(String::from($pname),
            String::from($locname)),
            vec![card.clone()]); 
    
        loc_card
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
    ($gd:expr, $($locname:literal), *) => {{
        use std::collections::HashMap;

        let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
        let locs: Vec<&str> =  vec![$($locname), *];
        for loc in locs.iter() {
            let loc_ref = LocationRef::Own(String::from(*loc));
            let location = $gd.get_location(&loc_ref).unwrap();
            let cards = location.borrow().clone().get_cards();
            loc_cards.insert(loc_ref, cards);
        }

        loc_cards
    }};
    
    // w = where
    ($gd:expr, $($locname:literal), * w $f:tt) => {{
        use crate::ast::LocationRef;
        use std::collections::HashMap;

        let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
        let locs: Vec<&str> =  vec![$($locname), *];
        for loc in locs.iter() {
            let l = $gd.get_location(&LocationRef::Own(loc.to_string())).unwrap();
            let mut cards = l.borrow().get_cards_ref();
            let fc: Vec<Card> = $f(cards.clone()).into_iter().flatten().collect();
            cards = cards.into_iter().filter(|card| fc.contains(card)).collect();
            loc_cards.insert(LocationRef::Own(loc.to_string()),
                cards
            );
        }

        loc_cards
    }};

    ($gd:expr, $comboname:literal inn $($locname:literal), *) => {{
        use crate::ast::LocationRef;
        use std::collections::HashMap;

        let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
        let locs: Vec<&str> =  vec![$($locname), *];
        for loc in locs.iter() {
            let l = $gd.get_location(&LocationRef::Own(loc.to_string())).unwrap();
            let mut cards = l.borrow().get_cards_ref();
            let cardcombo = $gd.get_combo($comboname).unwrap();
            let cardfun = &cardcombo.attributes;
            let fc: Vec<Card> = cardfun(cards.clone()).into_iter().flatten().collect();
            cards = cards.into_iter().filter(|card| fc.contains(card)).collect();
            loc_cards.insert(LocationRef::Own(loc.to_string()),
                cards
            );
        }

        loc_cards
    }};

    ($gd:expr, not $comboname:literal inn $($locname:literal), *) => {{
        use crate::ast::LocationRef;
        use std::collections::HashMap;

        let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();
        let locs: Vec<&str> =  vec![$($locname), *];
        for loc in locs.iter() {
            let l = $gd.get_location(&LocationRef::Own(loc.to_string())).unwrap();
            let mut cards = l.borrow().get_cards_ref();
            let cardcombo = $gd.get_combo($comboname).unwrap();
            let cardfun = &cardcombo.attributes;
            let fc: Vec<Card> = cardfun(cards.clone()).into_iter().flatten().collect();
            cards = cards.into_iter().filter(|card| !fc.contains(card)).collect();
            loc_cards.insert(LocationRef::Own(loc.to_string()),
                cards
            );
        }        

        loc_cards
    }};

    ($cardpos:tt) => {{
        use crate::ast::LocationRef;

        let cardpos: HashMap<LocationRef, Vec<Card>> = ($cardpos); 
        cardpos
    }};
}


macro_rules! combo {
    ($gd:expr, $name:literal, $filter:expr) => {
        use crate::ast::{CardFunction, CardCombination};

        $gd.add_cardcombination(
            $name,
            CardCombination {
                name: String::from($name),
                attributes: CardFunction::new(Rc::new($filter)), // Ensure Arc wrapping
            }
        );
    };
}


/*
This is needed for Condition:

Int → INT | ’(’ Int (’+’ | ’-’ | ’*’ | ’//’ | ’mod’) Int ’)’ |
    [IntCollection] Int | size’ ’of’ [Collection] |
    ’sum’ ’of’ ([IntCollection] | CardSet ’using’ [PointMap]) |
    (’min’ | ’max’) ’of’ [IntCollection] |
    ’stageroundcounter’ | ’playroundcounter’

TODO:
Implement the Types above!
(maybe call them IntCond, StringCond, BoolCond or something like that,
because it is confusing if we call tehm Int, String, Bool)

*/


macro_rules! int {
    ($int:literal) => {{
        let i: i32 = $int;
        i
    }};

    ($int1:expr, $op:literal, $int2:expr) => {{
        let i1: i32 = $int1;
        let i2: i32 = $int2;
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
    }};

    ($intcol:expr, $int:tt) => {{
        let index: usize = $int as usize;
        $intcol[index]        
    }};

    // size’ ’of’ [Collection] 
    (size of $col:expr) => {{
        $col.len()
    }};

    // ’sum’ ’of’ [IntCollection]
    (sum of $intcol:expr) => {{
        let intcol: Vec<i32> = $intcol;
        intcol.iter().sum::<i32>()
    }};

    ($gd:expr, sum of min $cardset:expr, using $pmname:literal) => {{
        let pmap = &$gd.pointmaps.get($pmname).unwrap();
        
        let mut sum = 0;

        let cardset = $cardset;

        let mut cards = vec![];
        for (_, cs) in cardset.iter() {
            for c in cs {
                cards.push(c);
            }
        }

        for card in cards.iter() {
            // let values = pmap.get_card_value_ref(card).unwrap();
            // let min_value = values.iter().min().unwrap();
            // sum += min_value;
            sum += pmap.get_card_value_ref(card).unwrap().iter().min().unwrap();
        }

        sum
    }};

    ($gd:expr, sum of max $cardset:expr, using $pmname:literal) => {{
        let pmap = &$gd.pointmaps.get($pmname).unwrap();
        
        let mut sum = 0;

        let cardset = $cardset;

        let mut cards = vec![];
        for (_, cs) in cardset.iter() {
            for c in cs {
                cards.push(c);
            }
        }


        for card in cards.iter() {
            sum += pmap.get_card_value_ref(&card).unwrap().iter().max().unwrap();
        }

        sum
    }};

    
    ($gd:expr, sum of $cardset:expr, using $pmname:literal gt $int:expr) => {{
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

        let pmap = &$gd.pointmaps.get($pmname).unwrap();

        let target = $int;
        
        let mut matrix = vec![];

        let cardset = $cardset;

        let mut cards = vec![];
        for (_, cs) in cardset.iter() {
            for c in cs {
                cards.push(c);
            }
        }

        for card in cards.iter() {
            matrix.push(pmap.get_card_value_ref(&card).unwrap());
        }

        min_sum_greater_equal(matrix, target).unwrap()
    }};

    ($gd:expr, sum of $cardset:expr, using $pmname:literal lt $int:expr) => {{
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

        let pmap = &$gd.pointmaps.get($pmname).unwrap();

        // same problem just negate everything
        let target = - $int;
        
        let cardset = $cardset;

        let mut cards = vec![];
        for (_, cs) in cardset.iter() {
            for c in cs {
                cards.push(c);
            }
        }

        let mut matrix = vec![];

        for card in cards.iter() {
            matrix.push(negate_vec(pmap.get_card_value_ref(&card).unwrap()));
        }

        - min_sum_greater_equal(matrix, target).unwrap()
    }};

    // (’min’ | ’max’) ’of’ [IntCollection] 
    (min of $intcol:expr) => {{
        *$intcol.iter().min().unwrap()
    }};

    (max of $intcol:expr) => {{
        *$intcol.iter().max().unwrap()
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
        $id
    }};

    // Problem:
    // there are multiple minima and maxima,
    // so it is not always one card (but should be maybe)
    // let map: HashMap<LocationRef, Vec<Card>> = $cardpos;
    ($key:literal of $cardpos:expr) => {{
        use std::collections::HashMap;

        let map = $cardpos;
        let card = map.iter().next().map(|(_, v)| v[0].clone()).unwrap();

        card.clone().attributes.get($key).unwrap()
    }};

    ($stringcol:expr, $int:expr) => {{
        let index = $int as usize;
        $stringcol[index]
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
        
        match $op {
            "==" => $string1 == $string2,
            "!=" => $string1 != $string2,
            _    => {
                        println!("Unknown Operator!");
                        false
                    }
        }
    }};

    (int: $int1:expr, $op:literal, $int2:expr) => {{
        match $op {
            "==" => $int1 == $int2,
            "!=" => $int1 != $int2,
            "<"  => $int1 < $int2,
            ">"  => $int1 > $int2,
            "<=" => $int1 <= $int2,
            ">=" => $int1 >= $int2,
            _    => {
                        println!("Unknown Operator!");
                        false
                    }
        }
    }};

    // CardSet (’==’ | ’!=’) CardSet
    (cardset: $cs1:expr, $op:literal, $cs2:expr) => {{
        fn eq(
            cs1: HashMap<LocationRef, Vec<Card>>,
            cs2: HashMap<LocationRef, Vec<Card>>,
        ) -> bool {
            let cards1: Vec<&Card> = cs1.values().flatten().collect();
            let cards2: Vec<&Card> = cs2.values().flatten().collect();
        
            cards1 == cards2
        }


        match $op {
            "==" => eq($cs1, $cs2),
            "!=" => !eq($cs1, $cs2),
            _    => {
                        println!("Unknown Operator!");
                        false
                    }
        }
    }};

    // CardSet ’is’ (’not’)? ’empty’
    ($cs:expr, is empty) => {{
        let mut isempty = true;
        for (_, v) in $cs.iter() {
            if !v.is_empty() {
                isempty = false;
                break;
            }
        }

        isempty
    }};

    ($cs:expr, is not empty) => {{
        let mut isnotempty = false;
        for (_, v) in $cs.iter() {
            if !v.is_empty() {
                isnotempty = true;
                break;
            }
        }

        isnotempty
    }};

    // Player == Player and Team == Team
    (pt: $ref1:expr, $op:literal, $ref2:expr) => {{
        use std::ptr;

        match $op {
            "==" => ptr::eq($ref1, $ref2),
            "!=" => !ptr::eq($ref1, $ref2),
            _    => {
                        println!("Unknown Operator!");
                        false
                    }
        }
    }};

    // ’(’ Bool (’and’ | ’or’) Bool ’)’ 
    ($b1:expr, $op:literal, $b2:expr) => {{
        match $op {
            "and" => $b1 && $b2,
            "or"  => $b1 || $b2,
            _     => {
                        println!("Unknown Operator!");
                        false
                    }
        }
    }};

    // ’not’ ’(’ Bool ’)’
    (not $b1:expr) => {{
        !$b1
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
    ($gd:expr, $pname:literal) => {{
        $gd.players.get($pname).unwrap()
    }};

    ($gd:expr, current) => {{
        let current = $gd.current as usize;
        let pname   = &$gd.turnorder[current];
        $gd.players.get(pname).unwrap()
    }};

    ($gd:expr, next) => {{
        let current = $gd.current as i32;
        let next    = ((current + 1) % ($gd.turnorder.len() as i32)) as usize;
        let pname   = &$gd.turnorder[next];
        $gd.players.get(pname).unwrap()
    }};

    ($gd:expr, previous) => {{
        let current = $gd.current as i32;
        let len = $gd.turnorder.len() as i32;
        let previous    = ((current - 1 + len) % len) as usize;
        let pname   = &$gd.turnorder[previous];
        $gd.players.get(pname).unwrap()
    }};

    // If we have teams or no teams at all then we have multiple competitors
    // makes not a lot of sense
    // ($cgm:expr, competitor) => {{
    //     $cgm.playertoteam
    //     $cgm.gamedata.players.get(pname).unwrap()
    // }};
    
    ($gd:expr, turnorder $int:expr) => {{
        let i       = $int as i32;
        let len = $gd.turnorder.len() as i32;
        let index   = ((i - 1 + len) % len) as usize;
        let pname   = &$gd.turnorder[index];
        $gd.players.get(pname).unwrap()
    }};

    // ’owner’ ’of’ CardPosition
    ($gd:expr, owner of $cardpos:expr) => {{
        let map = $cardpos;
        let i     = $gd.current as usize;
        let pname = &$gd.turnorder[i];
        let locowner: LocationRef = map.iter().next().map(|(k, _)| k.clone()).unwrap();
        match locowner {
            LocationRef::Own(_)       => $gd.players.get(pname).unwrap(),
            LocationRef::Player(player, _) => $gd.players.get(&player).unwrap(),
            _                             => {
                println!("No owner found!");
                // Placeholder for player return (return current if not found)
                $gd.players.get(pname).unwrap()
            }  
            // We try to find one player so we ignore teams
            // LocationRef::Team(tname, _) => $cgm.gamedata.players.get(pname).unwrap(),
            // LocationRef::Table(pname) => $cgm.gamedata.players.get(pname).unwrap(),
        }
    }}

    // TODO:
    // ’owner’ ’of’ (’highest’ | ’lowest’) [Memory]
    
}

// Team → TeamName | ’team’ ’of’ [Player]
macro_rules! team_ref {
    ($gd:expr, $tname:literal) => {{
        $gd.teams.get($tname).unwrap()
    }};

    ($gd:expr, team of $pref:expr) => {{
        use crate::ast::Player;
        let player: &Player = $pref;
        let pname: &str = &player.name;
        let tname = $gd.playertoteam.get(pname).unwrap();
        $gd.teams.get(tname).unwrap()
    }};
}


// ActionRule → FlipAction |ShuffleAction | MoveAction | MemoryAction | CycleAction |
//              OutAction | EndAction | DemAction
// macro_rules! actionrule {
//     () => {
        
//     };
// }


macro_rules! moveaction {
    // ClassicMove → ’move’ (Quantity (’from’)?)? CardSet Status (’bound’)? ’to’ CardSet
    // move X from <from> to <to>
    ($cgm:expr, mv $q:literal from $fromcs:tt to $tocs:tt) => {{
    }};

    // move X from <from> bound to <to>
    ($cgm:expr, mv $q:literal from $fromcs:tt bound to $tocs:tt) => {{
    }};

    // move X <from> to <to> (implicit "from")
    ($cgm:expr, mv $q:literal $fromcs:tt to $tocs:tt) => {{
    }};

    // move X <from> bound to <to> (implicit "from")
    ($cgm:expr, mv $q:literal $fromcs:tt bound to $tocs:tt) => {{
    }};

    ($cgm:expr, mv $fromcs:tt to $tocs:tt) => {{
        for (from_locref, cards) in $fromcs.into_iter() {
            let _: Vec<Card> = cards;
            if let Some(from_loc) = $cgm.gamedata.get_location(&from_locref).cloned() {
                for (to_locref, _) in &$tocs {
                    if let Some(to_loc) = $cgm.gamedata.get_location(to_locref).cloned() {
                        from_loc.borrow_mut().move_cards(&mut to_loc.borrow_mut(), &cards);
                    } else {
                        println!("Target location {:?} not found!", to_locref);
                    }
                    break; // Only move to one destination per source
                }
            } else {
                println!("Source location {:?} not found!", from_locref);
            }
        }
    }};
    
    ($cgm:expr, mv $fromcs:tt bound to $tocs:tt) => {{

    }};

    // DealMove → ’deal’ (Quantity (’from’)? )? CardSet Status ’bound’? ’to’ CardSet
    ($cgm:expr, deal $q:literal from $fromcs:tt to $tocs:tt) => {{
        let mut counter = $q;
        // get the top card of the from-card-set
        let fromcs_vec: Vec<(LocationRef, Vec<Card>)> = $fromcs.into_iter().collect();
        let toloc_ref: LocationRef = $tocs.iter().next().map(|(k, _)| k.clone()).unwrap();
        if let Some(toloc) = &$cgm.gamedata.get_location(&toloc_ref).cloned() {
            for (loc_ref, fromcards) in fromcs_vec.iter() {
                if let Some(fromloc) = &$cgm.gamedata.get_location(&loc_ref).cloned() {
                    for i in 0..counter {
                        if i == fromcards.len() {
                            break;
                        }
                        // TODO:
                        // Handle error
                        let _ = fromloc.borrow_mut().move_card_index(
                            &mut toloc.borrow_mut(),
                            0
                        );

                        counter -= 1;
                        
                        if counter == 0 {
                            break;
                        }
                    }
                } else {
                    println!("Target location {:?} not found!", loc_ref);
                }
                if counter == 0 {
                    break;
                }
            }
        } else {
            println!("Target location {:?} not found!", toloc_ref);
        }
    }};
    
    ($cgm:expr, deal $q:literal from $fromcs:tt bound to $tocs:tt) => {{
        
    }};

    ($cgm:expr, deal $fromcs:tt to $tocs:tt) => {{
        moveaction!($cgm, mv $fromcs to $tocs);
    }};
    
    ($cgm:expr, deal $fromcs:tt bound to $tocs:tt) => {{
        moveaction!($cgm, mv $fromcs bound to $tocs);
    }};

    // ExchangeMove → ’exchange’ (Quantity (’from’)?)? CardSet ’with’ CardSet
    ($cgm:expr, exchange $q:literal from $fromcs:tt with $tocs:tt) => {{
        
    }};

    ($cgm:expr, exchange $q:literal $fromcs:tt with $tocs:tt) => {{
        
    }};

    ($cgm:expr, exchange $fromcs:tt with $tocs:tt) => {{
        
    }};
}


// ’until’ Bool ((’and’ | ’or’) Repetitions)? | Repetitions | ’until’ ’end’
macro_rules! endcondition {
    ($cgm:expr, until $bool:literal) => {
        // I would say until the bool is false
        $bool
    };

    // Where do we save the repitions?
    ($cgm:expr, until $bool:literal and $reps:tt) => {

    };

    // Where do we save the repitions?
    ($cgm:expr, until $bool:literal or $reps:tt) => {

    };

    // Where do i save the repitions?
    ($reps:expr) => {

    };

    (until end) => {

    };
}

// seq-stage
// SeqStage -> ’Stage’ Stage ’for’ [Player] EndCondition ’:’ (’create’ SetupRule | PlayRule |
//      ScoringRule)+ ’}’
macro_rules! seqstage {
    ($cgm:expr, stage $stage:literal ffor $pref:expr, $endcond:expr,
        create (($setuprule:expr), * ($playrule:expr), * ($scoringrule:expr) *)*) => {

    };
}



macro_rules! condrule {
    (conditional: (case: $bool:tt? ($rule:tt+))+) => {

    }
}


macro_rules! ifrule {
    (iff $bool:tt? then $rule:tt+) => {

    }
}

macro_rules! oprule {
    (optional: $rule:tt) => {

    }
}

macro_rules! choicerule {
    (choose: $prule1:tt ($(or: $prule2:tt),*)) => {


    }
}

macro_rules! triggerrule {
    (trigger: $prule:tt) => {

    }
}


/*
ScoringRule → ScoreRule | WinnerRule
ScoreRule → ’score’ Int (’to’ [Memory])? ’of’ ([PlayerName] | PlayerCollection)
WinnerRule → ’winner’ ’is’ ([PlayerName] | PlayerCollection) | (’lowest’ | ’highest’) (’Score’
    | ’Position’ | [Memory])
*/


