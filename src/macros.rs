macro_rules! player {
    ($cgm:expr, $($n:expr), *) => {
        {
            let player_names: Vec<String> = vec![$($n.to_string()), *];
            $cgm.gamedata.add_players(player_names)
        }
    }
}

macro_rules! team {
    ($cgm:expr, $n:expr, ($($p:expr), *)) => {
        {
            let player_names: Vec<String> = vec![$($p.to_string()), *];
            $cgm.gamedata.add_team($n.to_string(), player_names);
        }
    };
}

macro_rules! location_on {
    ($cgm:expr, $location:literal, players: $($p:expr), *) => {
        {
            let player_names: Vec<String> = vec![$($p.to_string()), *];
            for p in player_names {
                $cgm.gamedata.add_loc_player($location.to_string(), p)
            }
        }
    };

    ($cgm:expr, $location:literal, team: $team:expr) => {
        {
            $cgm.gamedata.add_loc_team($location.to_string(), $team.to_string())
        }
    };
    ($cgm:expr, $location:literal, table) => {
        {
            $cgm.gamedata.add_loc_table($location.to_string());
        }
    };
}

macro_rules! card_on {
    (
        $cgm:expr,
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
        let locs = $cgm.gamedata.get_mut_locs($location);
        let comp_card: Vec<Component> = all_cards.into_iter().map(|c| Component::CARD(c)).collect();
        for i in 0..locs.len() {
            locs[i].borrow_mut().contents.extend(comp_card.clone());
        }
    }};
}

macro_rules! precedence {
    (
        $cgm:expr,
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
        $cgm.gamedata.add_precedence(Precedence { name: $name.to_string(), attributes: precedence_map});
    }};
}

macro_rules! pointmap {
    (
        $cgm:expr,
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
        $cgm.gamedata.add_pointmap(PointMap { name: $pmapname.to_string(), entries: point_map});
        // Modify gamedata
        // $cgm.gamedata.add_pointmap(PointMap {
        //     name: format!("{}", stringify!($($name1),*)), // Handle multiple `$name1`
        //     entries: point_map.clone(), // Return a copy if needed
        // });
    }};
}

macro_rules! turn_order {

    ($cgm:expr, random) => {{
        use rand::seq::SliceRandom;

        // DO NOT CLONE THE REFERENCE
        let mut turn_order: Vec<String> = $cgm.gamedata.players.keys().cloned().collect();
        let mut rng = rand::thread_rng();
        turn_order.shuffle(&mut rng);
        $cgm.gamedata.set_turnorder(turn_order);
    }};

    ($cgm:expr, ($($pname:expr),*)) => {{
        $cgm.gamedata.set_turnorder(vec![$($pname.to_string()),*]);
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
    ($cgm:expr, ($key:literal "adjacent" using $precedence_map:literal)) => {{
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
            
        // TODO: Make it safe (bc of unwrap() use)
        let precedence_map = &$cgm
            .gamedata
            .precedences
            .get($precedence_map)
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

    ($cgm:expr, $comboname:literal) => {
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            use std::ops::Deref;
            let cardfun: CardFunction = $cgm
                .gamedata
                .cardcombinations
                .get($comboname)
                .unwrap()
                .attributes
                .clone();
            cardfun.deref()(cards)
        }
    };

    ($cgm:expr, not $comboname:literal) => {{
        move |cards: Vec<Card>| -> Vec<Vec<Card>> {
            use std::ops::Deref;
    
            let cardfun = $cgm
                .gamedata
                .cardcombinations
                .get($comboname)
                .unwrap()
                .attributes
                .clone();
    
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
    ($cgm:expr, $locname:literal $int:literal) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
    
        let card_map = cardset!($cgm, $locname);
        
        if let Some(cards) = card_map.get(&LocationRef::Own($locname.to_string())) {
            if let Some(card) = cards.get($int) {
                loc_card.insert(LocationRef::Own($locname.to_string()), 
                vec![card.clone()]);
            }
        }
    
        loc_card
    }};

    ($cgm:expr, $locname:literal top) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
    
        let card_map = cardset!($cgm, $locname);
        
        if let Some(cards) = card_map.get(&LocationRef::Own($locname.to_string())) {
            if let Some(card) = cards.get(0) {
                loc_card.insert(LocationRef::Own($locname.to_string()), vec![card.clone()]);
            }
        }
    
        loc_card
    }};

    ($cgm:expr, $locname:literal bottom) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
    
        let card_map = cardset!($cgm, $locname);
        
        if let Some(cards) = card_map.get(&LocationRef::Own($locname.to_string())) {
            let len = cards.len();
            if let Some(card) = cards.get(len - 1) {
                loc_card.insert(LocationRef::Own($locname.to_string()),
                    vec![card.clone()]);
            }
        }
    
        loc_card
    }};

    ($cgm:expr, min of $cardset:tt using prec: $precname:literal) => {{
        use crate::ast::LocationRef;

        let prec = $cgm.gamedata.precedences.get($precname).unwrap();

        // First, collect all cards with their location and score
        let mut scored_cards: Vec<(LocationRef, Card, usize)> = vec![];

        for (loc, cards) in &$cardset {
            for card in cards {
                if let Some(score) = prec.get_card_value_ref(card) {
                    scored_cards.push((loc.clone(), card.clone(), score));
                }
            }
        }

        // Find the global minimum score
        let min_score = scored_cards
            .iter()
            .map(|(_, _, score)| *score)
            .min();

        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        if let Some(min_val) = min_score {
            for (loc, card, score) in scored_cards {
                if score == min_val {
                    result.entry(loc).or_default().push(card);
                }
            }
        }

        result
    }};

    ($cgm:expr, max of $cardset:tt using prec: $precname:literal) => {{
        use crate::ast::LocationRef;

        let prec = $cgm.gamedata.precedences.get($precname).unwrap();

        // Step 1: Gather all cards with their location and score
        let mut scored_cards: Vec<(LocationRef, Card, usize)> = vec![];

        for (loc, cards) in &$cardset {
            for card in cards {
                if let Some(score) = prec.get_card_value_ref(card) {
                    scored_cards.push((loc.clone(), card.clone(), score));
                }
            }
        }

        // Step 2: Find the global maximum score
        let max_score = scored_cards
            .iter()
            .map(|(_, _, score)| *score)
            .max();

        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        if let Some(max_val) = max_score {
            for (loc, card, score) in scored_cards {
                if score == max_val {
                    result.entry(loc).or_default().push(card);
                }
            }
        }

        result
    }};

    ($cgm:expr, min of $cardset:tt using pointmap: $pmname:literal) => {{
        use crate::ast::LocationRef;

        let pointmap = $cgm.gamedata.pointmaps.get($pmname).unwrap();

        // First, collect all cards with their location and score
        let mut scored_cards: Vec<(LocationRef, Card, i32)> = vec![];

        for (loc, cards) in &$cardset {
            for card in cards {
                if let Some(score) = pointmap.get_card_value_ref(card) {
                    scored_cards.push((loc.clone(), card.clone(), *score.iter().min().unwrap()));
                }
            }
        }

        // Find the global minimum score
        let min_score = scored_cards
            .iter()
            .map(|(_, _, score)| *score)
            .min();

        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        if let Some(min_val) = min_score {
            for (loc, card, score) in scored_cards {
                if score == min_val {
                    result.entry(loc).or_default().push(card);
                }
            }
        }

        result
    }};

    ($cgm:expr, max of $cardset:tt using pointmap: $pmname:literal) => {{
        use crate::ast::LocationRef;

        let pointmap = $cgm.gamedata.pointmaps.get($pmname).unwrap();

        // Step 1: Gather all cards with their location and score
        let mut scored_cards: Vec<(LocationRef, Card, i32)> = vec![];

        for (loc, cards) in &$cardset {
            for card in cards {
                if let Some(score) = pointmap.get_card_value_ref(card) {
                    scored_cards.push((loc.clone(), card.clone(), *score.iter().max().unwrap()));
                }
            }
        }

        // Step 2: Find the global maximum score
        let max_score = scored_cards
            .iter()
            .map(|(_, _, score)| *score)
            .max();

        let mut result: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        if let Some(max_val) = max_score {
            for (loc, card, score) in scored_cards {
                if score == max_val {
                    result.entry(loc).or_default().push(card);
                }
            }
        }

        result
    }};

    // location OF player
    ($cgm:expr, $locname:literal of $pname:literal $int:literal) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
    
        let card_map = cardset!($cgm, $locname);
        
        if let Some(cards) = card_map.get($locname) {
            if let Some(card) = cards.get($int) {
                loc_card.insert(LocationRef::Player($pname.to_string(),
                    $locname.to_string()),
                    vec![card.clone()]);
            }
        }
    
        loc_card
    }};

    ($cgm:expr, $locname:literal of $pname:literal top) => {{
        use crate::ast::LocationRef;

        let mut loc_card: HashMap<String, Vec<Card>> = HashMap::new();
    
        let card_map = cardset!($cgm, $locname);
        
        if let Some(cards) = card_map.get($locname) {
            if let Some(card) = cards.get(0) {
                loc_card.insert(LocationRef::Player($pname.to_string(),
                    $locname.to_string()),
                    vec![card.clone()]);
            }
        }
    
        loc_card
    }};

    ($cgm:expr, $locname:literal of $pname:literal bottom) => {{
        let mut loc_card: HashMap<LocationRef, Vec<Card>> = HashMap::new();
    
        let card_map = cardset!($cgm, $locname);
        
        if let Some(cards) = card_map.get($locname) {
            let len = cards.len();
            if let Some(card) = cards.get(len - 1) {
                loc_card.insert(LocationRef::Player($pname.to_string(),
                    $locname.to_string()),
                    vec![card.clone()]);
            }
        }
    
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
    ($cgm:expr, $($locname:literal), *) => {{
        use std::collections::HashMap;

        let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        let locs: Vec<&str> =  vec![$($locname), *];

        for loc in locs.iter() {
            let loc_ref = LocationRef::Own(loc.to_string());
    
            if let Some(location) = $cgm.gamedata.get_location(&loc_ref) {
                let cards = location.borrow().get_cards_ref();
                loc_cards.insert(loc_ref, cards);
            } else {
                eprintln!("⚠️ Location '{}' (Own) not found!", loc);
            }
        }

        loc_cards
    }};
    
    // w = where
    ($cgm:expr, $($locname:literal), * w $f:tt) => {{
        use crate::ast::LocationRef;

        use std::collections::HashMap;

        let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        let locs: Vec<&str> =  vec![$($locname), *];

        for loc in locs.iter() {
            let mut cards = $cgm.gamedata
                .get_location(&LocationRef::Own(loc.to_string()))
                .unwrap()
                .borrow()
                .get_cards_ref();

            let fc: Vec<Card> = $f(cards.clone()).into_iter().flatten().collect();

            cards = cards.into_iter().filter(|card| fc.contains(card)).collect();

            loc_cards.insert(LocationRef::Own(loc.to_string()),
                cards
            );
        }

        loc_cards
    }};

    ($cgm:expr, $comboname:literal inn $($locname:literal), *) => {{
        use crate::ast::LocationRef;

        use std::collections::HashMap;

        let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        let locs: Vec<&str> =  vec![$($locname), *];

        for loc in locs.iter() {
            let mut cards = $cgm.gamedata
                .get_location(&LocationRef::Own(loc.to_string()))
                .unwrap()
                .borrow()
                .get_cards_ref();

            let cardfun = $cgm
                    .gamedata
                    .cardcombinations
                    .get($comboname)
                    .unwrap()
                    .attributes
                    .clone();
            
            let fc: Vec<Card> = cardfun(cards.clone()).into_iter().flatten().collect();
    
            cards = cards.into_iter().filter(|card| fc.contains(card)).collect();

            loc_cards.insert(LocationRef::Own(loc.to_string()),
                cards
            );
        }

        loc_cards
    }};

    ($cgm:expr, not $comboname:literal inn $($locname:literal), *) => {{
        use crate::ast::LocationRef;

        use std::collections::HashMap;

        let mut loc_cards: HashMap<LocationRef, Vec<Card>> = HashMap::new();

        let locs: Vec<&str> =  vec![$($locname), *];

        for loc in locs.iter() {
            let mut cards = $cgm.gamedata
                .get_location(&LocationRef::Own(loc.to_string()))
                .unwrap()
                .borrow()
                .get_cards_ref();

            let cardfun = $cgm
                    .gamedata
                    .cardcombinations
                    .get($comboname)
                    .unwrap()
                    .attributes
                    .clone();
            
            let fc: Vec<Card> = cardfun(cards.clone()).into_iter().flatten().collect();
    
            cards = cards.into_iter().filter(|card| !fc.contains(card)).collect();

            loc_cards.insert(LocationRef::Own(loc.to_string()),
                cards
            );
        }

        loc_cards
    }};

    ($cgm:expr, $cardpos:tt) => {{
        use crate::ast::LocationRef;

        let cardpos: HashMap<LocationRef, Vec<Card>> = $cardpos; 
        cardpos
    }};
}


macro_rules! combo {
    ($cgm:expr, $name:literal, $filter:expr) => {
        use crate::ast::{CardFunction, CardCombination};

        $cgm.gamedata.add_cardcombination(
            $name.to_string(),
            CardCombination {
                name: $name.to_string(),
                attributes: CardFunction::new(Rc::new($filter)), // Ensure Arc wrapping
            }
        );
    };
}

macro_rules! endcondition {
    ($cgm:expr, until $bool:literal) => {

    };

    ($cgm:expr, until $bool:literal and $reps:tt) => {

    };

    ($cgm:expr, until $bool:literal or $reps:tt) => {

    };

    ($reps:expr) => {

    };

    (until end) => {

    };
}

macro_rules! condition {
    // bool with cards and player-location
    ($cgm:expr, $filter:tt of $locname:literal) => {{
        let playername = $cgm.gamedata.turnorder[$cgm.gamedata.current].clone();
        let cards = $cgm
                    .gamedata
                    .players
                    .get_mut(
                        &playername
                    )
                    .unwrap()
                    .get_location($locname)
                    .unwrap()
                    .borrow()
                    .clone()
                    .get_cards();

        !$filter(cards).is_empty()
    }};
    // TODO:
    // condition for arbitrary things!
}

// ActionRule → FlipAction |ShuffleAction | MoveAction | MemoryAction | CycleAction |
//              OutAction | EndAction | DemAction
// macro_rules! actionrule {
//     () => {
        
//     };
// }

/*
The defintion is switched in the Thesis.
It has to be a mistake and should look like this:


// Group is in no rules required
Group → Group (’of’ ([Player] | PlayerCollection))?

CardSet → ([Location] | LocationCollection) (’where’ Filter)? |
            (’not’)? [Combo] ’in’ ([Location] | LocationCollection) |
            CardPosition

CardPosition → Location (Int | ’Top’ | ’Bottom’) |
                (’min’ | ’max’) ’of’ CardSet ’using’ ([Precedence] | [PointMap])

*/



macro_rules! moveaction {
    // ClassicMove → ’move’ (Quantity (’from’)?)? CardSet Status (’bound’)? ’to’ CardSet
    ($cgm:expr, mv $q:literal from $fromcs:tt to $tocs:tt) => {{
        
    }};
    
    ($cgm:expr, mv $q:literal from $fromcs:tt bound to $tocs:tt) => {{
        
    }};

    ($cgm:expr, mv $q:literal $fromcs:tt to $tocs:tt) => {{
        
    }};
    
    ($cgm:expr, mv $q:literal $fromcs:tt bound to $tocs:tt) => {{
        
    }};

    ($cgm:expr, mv $fromcs:tt to $tocs:tt) => {{
        
    }};
    
    ($cgm:expr, mv $fromcs:tt bound to $tocs:tt) => {{
        
    }};

    // DealMove → ’deal’ (Quantity (’from’)? )? CardSet Status ’bound’? ’to’ CardSet
    ($cgm:expr, deal $q:literal from $fromcs:tt to $tocs:tt) => {{
        
    }};
    
    ($cgm:expr, deal $q:literal from $fromcs:tt bound to $tocs:tt) => {{
        
    }};

    ($cgm:expr, deal $q:literal $fromcs:tt to $tocs:tt) => {{
        
    }};
    
    ($cgm:expr, deal $q:literal $fromcs:tt bound to $tocs:tt) => {{
        
    }};

    ($cgm:expr, deal $fromcs:tt to $tocs:tt) => {{
        
    }};
    
    ($cgm:expr, deal $fromcs:tt bound to $tocs:tt) => {{
        
    }};

    // ExchangeMove → ’exchange’ (Quantity (’from’)?)? CardSet ’with’ CardSet
    ($cgm:expr, exchange $q:literal from $fromcs:tt with $tocs:tt) => {{
        
    }};

    ($cgm:expr, exchange $q:literal $fromcs:tt with $tocs:tt) => {{
        
    }};

    ($cgm:expr, exchange $fromcs:tt with $tocs:tt) => {{
        
    }};
}

// seq-stage
macro_rules! seqstage {
    ($cgm:expr, stage $stage:literal ffor current, $endcond:expr) => {

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





