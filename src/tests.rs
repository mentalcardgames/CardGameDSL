#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::ast::{Card, CardGameModel, Player};

    #[derive(Debug)]
    struct Location {
        name: String,
    }
    

    #[test]
    fn test_player_macro() {
        let mut cgm = CardGameModel::new("player_test");

        // Ensure the macro modifies the cgm instance
        player!(cgm, "Jimmy", "Kimmy", "Timmy");

        assert_eq!(cgm.gamedata.players.len(), 3); // Ensure 3 players were added
        assert_eq!(cgm.gamedata.players[0].name, "Jimmy");
        assert_eq!(cgm.gamedata.players[1].name, "Jimmy");
        assert_eq!(cgm.gamedata.players[2].name, "Timmy");
    }

    #[test]
    fn test_team_macro() {
        let mut cgm = CardGameModel::new("team_test");
        player!(cgm, "Jimmy", "Kimmy", "Timmy");
        team! { cgm, "Team1", ("Jimmy", "Kimmy", "Timmy") };
        // Replace the below assertions with actual checks based on how `team!` works.
        // Example:
        assert!(cgm.gamedata.teams[0].teamname == "Team1".to_string());
        assert!((*cgm.gamedata.teams[0].players[0].borrow_mut()).name == "Jimmy".to_string());
    }
    
    #[test]
    fn test_location_on() {
        let mut cgm = CardGameModel::new("location_on_test");
        // Ensure the macro modifies the cgm instance
        player!(cgm, "Jimmy", "Kimmy", "Timmy");
        location_on!(cgm, "hand", players: "Jimmy", "Kimmy");
        assert!(cgm.gamedata.players[0].locations.len() == 1);

        team!(cgm, "t1", ("Kimmy", "Timmy"));
        location_on!(cgm, "teamloc", team: "t1");
        assert!(cgm.gamedata.teams[0].locations.len() == 1);

        location_on!(cgm, "stack", table);
        assert!(cgm.gamedata.table.locations.len() == 1);
    }

    #[test]
    fn test_card_on_macro() {
        let mut cgm = CardGameModel::new("cards_on_test");

        location_on!(cgm, "stack", table);

        card_on!(
            cgm,
            "stack",
            {
                Rank("2", "3", "4", "5", "A"),
                Suite("Diamond", "Hearts"),
                Color("Red")
            },
            {
                Rank("2", "3", "4", "5", "A"),
                Suite("Spades", "Clubs"),
                Color("Black")
            }
        );

        // Test cards
        assert!(cgm.gamedata.table.locations.get("stack").unwrap().contents.len() == 20);
    }

    #[test]
    fn test_precedence_macro() {
        let mut cgm = CardGameModel::new("precedence_test");

        precedence!(cgm, "rank", ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"));
        precedence!(cgm, "suite", ("Clubs", "Diamonds", "Hearts", "Spades"));

        // Test rank precedence
        assert!(cgm.gamedata.precedences.get("rank").unwrap().attributes.contains_key(&("rank2".to_string())));
        assert!(cgm.gamedata.precedences.get("rank").unwrap().attributes.contains_key(&("rankA".to_string())));

        // Test suite precedence
        assert!(cgm.gamedata.precedences.get("suite").unwrap().attributes.contains_key(&("suiteClubs".to_string())));
        assert!(cgm.gamedata.precedences.get("suite").unwrap().attributes.contains_key(&("suiteSpades".to_string())));
    }

    #[test]
    fn test_pointmap_macro() {
        let mut cgm = CardGameModel::new("pointmap_test");

        let rank_points = pointmap!(
            cgm,
            nested: {  
                "rank", (
                "2" => [2],
                "3" => [3],
                "4" => [4],
                "5" => [5],
                "6" => [6],
                "7" => [7],
                "8" => [8],
                "9" => [9],
                "10" => [10],
                "J" => [10],
                "Q" => [10],
                "K" => [10],
                "A" => [11, 1]
                )
            },
            list: {  
                ("suite", "clubs") => [100],
            },
        );

        // Test rank points, nested mapping
        assert!(rank_points.contains_key("rank2"));
        assert_eq!(rank_points["rank2"], vec![2]);

        assert!(rank_points.contains_key("rankA"));
        assert_eq!(rank_points["rankA"], vec![11, 1]);

        // Test rank points, flat mapping
        assert!(rank_points.contains_key("suiteclubs"));
        assert_eq!(rank_points["suiteclubs"], vec![100]);        
    }

    #[test]
    fn test_turn_order_macro() {

        let p1 = Player::new("Jimmy".to_string());
        let p2 = Player::new("Timmy".to_string());
        let p3 = Player::new("Kimmy".to_string());
        let p4 = Player::new("Mimmy".to_string());

        let rcp1 = Rc::new(p1);
        let rcp2 = Rc::new(p2);
        let rcp3 = Rc::new(p3);
        let rcp4 = Rc::new(p4);

        let order: Vec<String> = turn_order!(
            (rcp1.clone(),rcp2.clone(), rcp3.clone(), rcp4.clone())
        )
            .iter()
            .map(|x| x.name.clone())
            .collect();

        let random_order: Vec<String> = turn_order!(
            (rcp1.clone(), rcp2.clone(), rcp3.clone(), rcp4.clone()),
            random
        )
            .iter()
            .map(|x| x.name.clone())
            .collect();

        // Test fixed turn order
        assert_eq!(
            order,
            vec![rcp1.name.clone(), rcp2.name.clone(), rcp3.name.clone(), rcp4.name.clone()]
        );

        // Test random turn order
        assert_eq!(random_order.len(), 4);
        assert!(random_order.contains(&rcp1.name));
        assert!(random_order.contains(&rcp1.name));
    }

    #[test]
    fn test_filter_macro(){
        // Sample cards
        let cards = vec![
            Card::new([("rank".to_string(), "2".to_string()), ("suite".to_string(), "spades".to_string())].into()),
            Card::new([("rank".to_string(), "3".to_string()), ("suite".to_string(), "spades".to_string())].into()),
            Card::new([("rank".to_string(), "3".to_string()), ("suite".to_string(), "hearts".to_string())].into()),
            Card::new([("rank".to_string(), "3".to_string()), ("suite".to_string(), "diamonds".to_string())].into()),
            Card::new([("rank".to_string(), "4".to_string()), ("suite".to_string(), "spades".to_string())].into()),
            Card::new([("rank".to_string(), "4".to_string()), ("suite".to_string(), "clubs".to_string())].into()),
            Card::new([("rank".to_string(), "6".to_string()), ("suite".to_string(), "clubs".to_string())].into()),
        ];

        // Precedence map for "rank"
        // let rank_precedence = precedence!("rank", ("2", "3", "4", "5", "6"));

        // Filter for "same" rank
        let same_filter = filter!("rank", "same");
        let filtered_cards = same_filter(cards.clone());
        println!("{}", filtered_cards.len());
        for f in filtered_cards.iter() {
            println!("Same rank cards: {:?}", f);
        }

        // Filter for "distinct"
        // let distinct_filter = filter!("rank", "distinct");
        // let filtered_cards = distinct_filter(cards.clone());
        // println!("{}", filtered_cards.len());
        // for f in filtered_cards.iter() {
            // println!("Distinct rank cards: {:?}", f);
        // }
        
        // Filter for "adjacent" using precedence
        // let adjacent_filter = filter!("rank", "adjacent" using rank_precedence);
        // let filtered_cards = adjacent_filter(cards.clone());
        // println!("Adjacent rank cards: {:?}\n", filtered_cards);

        // Filter for "higher" using precedence ("higher" is interpreted as "highest")
        // let higher_filter = filter!("rank", "higher" using rank_precedence);
        // let filtered_cards = higher_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Higher rank cards: {:?}", c);
        // }

        // Filter for "lower" using precedence ("lower" is interpreted as "lowest")
        // let lower_filter = filter!("rank", "lower" using rank_precedence);
        // let filtered_cards = lower_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Lower rank cards: {:?}", c);
        // }

        // Filter with Key == Value
        let bool_filter = filter!("rank", "==", "3");
        let filtered_cards = bool_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("Equal rank cards: {:?}", c);
        }
    
        // Filter by size
        let size_filter = filter!(size, "==", 3);
        let filtered_cards = size_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("size cards == 3: {:?}", c);
        }

        // Filter by size
        let size_filter = filter!(size, ">", 3);
        let filtered_cards = size_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("size cards > 3: {:?}", c);
        }

        // Filter with Key != Value
        let bool_filter = filter!("rank", "!=", "3");
        let filtered_cards = bool_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("Not-Equal rank cards: {:?}", c);
        }

        // Combined filter
        // let combined_filter = filter!(
        //     ("rank", "adjacent" using rank_precedence), 
        //     ("and"), 
        //     ("suite", "same")
        // );        
        // let filtered_cards = combined_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Combined-Filter (rank-adjacent, suite same): {:?}", c);
        // }

        // let combined_filter = filter!(
        //     ("rank", "adjacent" using rank_precedence),
        //     ("and"),
        //     (size, ">=", 3)
        // );        
        // let filtered_cards = combined_filter(cards.clone());
        // for c in filtered_cards {
        //     println!("Combined-Filter (adjacent, size >= 3): {:?}", c);
        // }

        // Testing more nested filter functions
        let combined_filter = filter!(
            ("suite", "same"),
            ("and"),
            (size, ">=", 3)
        );        
        let filtered_cards = combined_filter(cards.clone());
        println!("Combined-Filter (suite same, size != 3): {:?}", filtered_cards);

        // Testing more nested filter functions
        // OR DOESNT WORK YET
        let combined_filter = filter!(
            ("suite", "same"),
            ("or"),
            ("rank", "same")
        );        
        let filtered_cards = combined_filter(cards.clone());
        println!("Combined-Filter (suite same, rank same): {:?}", filtered_cards);

    }

    #[test]
    fn test_location() {
        // // test for location on players
        // let players = player!("Jimmy", "Timmy");
        // let p_clone = players.clone();
        // location_on!("test_location1", players: p_clone);
        // for player in players.iter() {
        //     player.borrow_mut().show_locations();
        // }

        // // test for location on team
        // let mut team = team!("T1", ("Jimmy", "Timmy"));
        // location_on!("test_location2", team: &mut team);
        // team.show_locations();

        // // table not implemented yet
    }
}
