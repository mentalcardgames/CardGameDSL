#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::ast::{Card, Player};

    #[derive(Debug)]
    struct Location {
        name: String,
    }
    

    #[test]
    fn test_player_macro() {
        let players = player! { "Jimmy", "Jimmy", "Timmy" };

        // Replace the below assertions with actual checks based on how `player!` works.
        // Example:
        assert!(players[0].name == "Jimmy".to_string());
        assert!(players[1].name == "Jimmy".to_string());
        assert!(players[2].name == "Timmy".to_string());
    }

    #[test]
    fn test_team_macro() {
        let team = team! { "Team1", ("Jimmy", "Jimmy", "Timmy") };

        // Replace the below assertions with actual checks based on how `team!` works.
        // Example:
        assert!(team.teamname == "Team1".to_string());
        assert!(team.players[0].name == "Jimmy".to_string());
    }

    #[test]
    fn test_card_on_macro() {
        let location = Location {
            name: "Deck".to_string(),
        };

        let cards = card_on!(
            location,
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
        assert!(cards.len() > 0);
        // for card in cards {
        //     println!("{}", card); // Debugging output
        // }
    }

    #[test]
    fn test_precedence_macro() {
        let rank_precedence = precedence!("rank", ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"));
        let suite_precedence = precedence!("suite", ("Clubs", "Diamonds", "Hearts", "Spades"));

        // Test rank precedence
        assert!(rank_precedence.contains_key(&("rank2".to_string())));
        assert!(rank_precedence.contains_key(&("rankA".to_string())));

        // Test suite precedence
        assert!(suite_precedence.contains_key(&("suiteClubs".to_string())));
        assert!(suite_precedence.contains_key(&("suiteSpades".to_string())));
    }

    #[test]
    fn test_pointmap_macro() {
        let rank_points = pointmap!(
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
            Card::new([("rank".to_string(), "2".to_string())].into()),
            Card::new([("rank".to_string(), "3".to_string())].into()),
            Card::new([("rank".to_string(), "3".to_string())].into()),
            Card::new([("rank".to_string(), "3".to_string())].into()),
            Card::new([("rank".to_string(), "4".to_string())].into()),
            Card::new([("rank".to_string(), "4".to_string())].into()),
            Card::new([("suite".to_string(), "black".to_string())].into()),
        ];

        // Precedence map for "rank"
        let rank_precedence = precedence!("rank", ("2", "3", "4", "5", "6"));

        // Filter for "same" rank
        let same_filter = filter!("rank", "same");
        let filtered_cards = same_filter(cards.clone());
        println!("{}", filtered_cards.len());
        for f in filtered_cards.iter() {
            println!("Same rank cards: {:?}", f);
        }

        // Filter for "same" rank
        let distinct_filter = filter!("rank", "distinct");
        let filtered_cards = distinct_filter(cards.clone());
        println!("{}", filtered_cards.len());
        for f in filtered_cards.iter() {
            println!("Distinct rank cards: {:?}", f);
        }
        
        // Filter for "adjacent" using precedence
        let adjacent_filter = filter!("rank", "adjacent" using rank_precedence);
        let filtered_cards = adjacent_filter(cards.clone());
        println!("Adjacent rank cards: {:?}", filtered_cards);

        // Filter for "higher" using precedence ("higher" is interpreted as "highest")
        let higher_filter = filter!("rank", "higher" using rank_precedence);
        let filtered_cards = higher_filter(cards.clone());
        println!("Higher rank cards: {:?}", filtered_cards);

        // Filter for "lower" using precedence ("lower" is interpreted as "lowest")
        let lower_filter = filter!("rank", "lower" using rank_precedence);
        let filtered_cards = lower_filter(cards.clone());
        println!("Lower rank cards: {:?}", filtered_cards);

        // Filter with Key == Value
        let bool_filter = filter!("rank", "==", "3");
        let filtered_cards = bool_filter(cards.clone());
        println!("Equal rank cards: {:?}", filtered_cards);

        // Filter by size
        let size_filter = filter!(size, "==", 3);
        let filtered_cards = size_filter(cards.clone());
        println!("size cards == 3: {:?}", filtered_cards);

        // Filter by size
        let size_filter = filter!(size, ">", 3);
        let filtered_cards = size_filter(cards.clone());
        println!("size cards > 3: {:?}", filtered_cards);
        
        // Filter with Key != Value
        let bool_filter = filter!("rank", "!=", "3");
        let filtered_cards = bool_filter(cards.clone());
        println!("Not-Equal rank cards: {:?}", filtered_cards);

        // Combined filter
        let combined_filter = filter!(
            ("rank", "adjacent" using rank_precedence), 
            ("and"), 
            ("rank", "!=", "4")
        );        
        let filtered_cards = combined_filter(cards.clone());
        println!("Combined-Filter rank cards: {:?}", filtered_cards);
    }
}
