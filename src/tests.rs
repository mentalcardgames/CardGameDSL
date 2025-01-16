#[cfg(test)]
mod tests {
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
        assert!(rank_precedence.contains_key(&("2".to_string())));
        assert!(rank_precedence.contains_key(&("A".to_string())));

        // Test suite precedence
        assert!(suite_precedence.contains_key(&("Clubs".to_string())));
        assert!(suite_precedence.contains_key(&("Spades".to_string())));
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
        let order = turn_order!(("1", "2", "3", "4"));
        let random_order = turn_order!(("1", "2", "3", "4"), random);

        // Test fixed turn order
        assert_eq!(order, vec!["1", "2", "3", "4"]);

        // Test random turn order
        assert_eq!(random_order.len(), 4);
        assert!(random_order.contains(&("1".to_string())));
        assert!(random_order.contains(&("4".to_string())));
    }
}
