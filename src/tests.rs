#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc};
    use crate::ast::{Card, CardGameModel, Component, LocationRef};

    fn init_model() -> CardGameModel {
        let mut cgm = CardGameModel::new("player_test");

        // Ensure the macro modifies the cgm instance
        player!(cgm, "Jimmy", "Kimmy", "Timmy");

        turn_order!(cgm, ("Jimmy", "Kimmy", "Timmy"));

        team! { cgm, "Team1", ("Jimmy", "Kimmy", "Timmy") };

        location_on!(cgm, "hand", players: "Jimmy", "Kimmy", "Timmy");

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

        precedence!(cgm, "Rank", ("2", "3", "4", "5", "A"));

        cgm
    }

    #[test]
    fn test_player_macro() {
        let cgm = init_model();

        assert_eq!(cgm.gamedata.players.len(), 3); // Ensure 3 players were added
        assert_eq!(cgm.gamedata.players.get("Jimmy").unwrap().name, "Jimmy");
        assert_eq!(cgm.gamedata.players.get("Jimmy").unwrap().name, "Jimmy");
        assert_eq!(cgm.gamedata.players.get("Timmy").unwrap().name, "Timmy");
    }

    #[test]
    fn test_team_macro() {
        let cgm = init_model();
        // Replace the below assertions with actual checks based on how `team!` works.
        // Example:
        assert!(cgm.gamedata.teams.get("Team1").unwrap().teamname == "Team1".to_string());
        assert!(cgm.gamedata.teams.get("Team1").unwrap().players == vec!["Jimmy", "Kimmy", "Timmy"]);
    }
    
    #[test]
    fn test_location_on() {
        let mut cgm = init_model();
        assert!(cgm.gamedata.players.get("Jimmy").unwrap().locations.len() == 1);

        team!(cgm, "t1", ("Kimmy", "Timmy"));
        location_on!(cgm, "teamloc", team: "t1");
        assert!(cgm.gamedata.teams.get("t1").unwrap().locations.len() == 1);

        location_on!(cgm, "stack", table);
        assert!(cgm.gamedata.table.locations.len() == 1);
    }

    #[test]
    fn test_card_on_macro() {
        let cgm = init_model();

        // Test cards
        assert!(cgm.gamedata.table.locations.get("stack").unwrap().borrow().contents.len() == 20);
    }

    #[test]
    fn test_precedence_macro() {
        let mut cgm = CardGameModel::new("precedence_test");

        precedence!(cgm, "rank", ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"));
        precedence!(cgm, "suite", ("Clubs", "Diamonds", "Hearts", "Spades"));

        // Test rank precedence
        assert!(cgm.gamedata.precedences.get("rank").unwrap().attributes.contains_key(&("2".to_string())));
        assert!(cgm.gamedata.precedences.get("rank").unwrap().attributes.contains_key(&("A".to_string())));

        // Test suite precedence
        assert!(cgm.gamedata.precedences.get("suite").unwrap().attributes.contains_key(&("Clubs".to_string())));
        assert!(cgm.gamedata.precedences.get("suite").unwrap().attributes.contains_key(&("Spades".to_string())));
    }

    #[test]
    fn test_pointmap_macro() {
        let mut cgm = CardGameModel::new("pointmap_test");

        pointmap!(
            cgm,
            "rank",
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
        assert!(cgm.gamedata.pointmaps["rank"].entries["2"].len() == 1);
        assert_eq!(cgm.gamedata.pointmaps["rank"].entries["2"], vec![2]);

        assert!(cgm.gamedata.pointmaps["rank"].entries["A"].len() == 2);
        assert_eq!(cgm.gamedata.pointmaps["rank"].entries["A"], vec![11, 1]);

        // Test rank points, flat mapping
        assert!(cgm.gamedata.pointmaps["rank"].entries["clubs"].len() == 1);
        assert_eq!(cgm.gamedata.pointmaps["rank"].entries["clubs"], vec![100]);        
    }

    #[test]
    fn test_turn_order_macro() {
        fn print_players(turnorder: Vec<String>) {
            println!("turnorder:");
            println!("================");
            for p in turnorder {
                println!("{}", p)
            }
            println!("================");
            println!();
        }

        let mut cgm = CardGameModel::new("turn_order_test");
        player!(cgm, "Jimmy", "Kimmy", "Timmy");
        turn_order!(cgm, random);
        print_players(cgm.gamedata.turnorder.clone());
        
        turn_order!(cgm, ("Timmy", "Jimmy", "Kimmy"));
        print_players(cgm.gamedata.turnorder.clone());
    }

    #[test]
    fn test_filter_macro() {
        use crate::ast::Component;
        let cgm = init_model();
        
        let cards: Vec<Card> = cgm.gamedata.table.locations["stack"].borrow().contents
            .iter()
            .filter_map(|c| {
                if let Component::CARD(card) = c {
                    Some(card.clone()) // Return the cloned card
                } else {
                    None // Filter out non-card components
                }
            })
            .collect();
        
        // for c in cards.iter() {
        //     println!("{}", c);
        // }

        // Filter for "same" rank
        let same_filter = filter!("Rank", "same");
        let filtered_cards = same_filter(cards.clone());
        // println!("{}", filtered_cards.len());
        // for f in filtered_cards.iter() {
        //     println!("Same rank cards: {:?}", f);
        // }

        // TODO: I dont know how to implement
        // More precisely I dont understand what it means
 
        // Filter for "distinct"
        // let distinct_filter = filter!("Rank", "distinct");
        // let filtered_cards = distinct_filter(cards.clone());
        // println!("{}", filtered_cards.len());
        // for f in filtered_cards.iter() {
        //     println!("Distinct rank cards: {:?}", f);
        // }
        
        // Filter for "adjacent" using precedence
        let adjacent_filter = filter!(
            cgm, 
            ("Rank" "adjacent" using "Rank"));
        let filtered_cards = adjacent_filter(cards.clone());
        // println!("Adjacent rank cards: {:?}\n", filtered_cards);


        // Higher and Lower makes NO sense
        // Filter for "higher" using precedence ("higher" is interpreted as "highest")
        // let higher_filter = filter!("rank", "higher" using cgm.gamedata.precedences["Rank"]);
        // let filtered_cards = higher_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Higher rank cards: {:?}", c);
        // }

        // Filter for "lower" using precedence ("lower" is interpreted as "lowest")
        // let lower_filter = filter!("rank", "lower" using cgm.gamedata.precedences["Rank"]);
        // let filtered_cards = lower_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Lower rank cards: {:?}", c);
        // }

        // Filter with Key == Value
        let bool_filter = filter!("Rank", "==", "3");
        let filtered_cards = bool_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Equal rank cards: {:?}", c);
        // }
    
        // Filter by size
        let size_filter = filter!(size, "==", 3);
        let filtered_cards = size_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("size cards == 3: {:?}", c);
        // }

        // Filter by size
        let size_filter = filter!(size, ">", 3);
        let filtered_cards = size_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("size cards > 3: {:?}", c);
        // }

        // Filter with Key != Value
        let bool_filter = filter!("Rank", "!=", "3");
        let filtered_cards = bool_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Not-Equal rank cards: {:?}", c);
        // }

        

    }

    #[test]
    fn test_combined_filter() {
        use crate::ast::Component;
        let cgm = init_model();
        
        let cards: Vec<Card> = cgm.gamedata.table.locations["stack"].borrow().contents
            .iter()
            .filter_map(|c| {
                if let Component::CARD(card) = c {
                    Some(card.clone()) // Return the cloned card
                } else {
                    None // Filter out non-card components
                }
            })
            .collect();
        
        // Combined filter
        let combined_filter = filter!(
            (cgm, ("Rank" "adjacent" using "Rank")), 
            ("and"), 
            ("Suite", "same")
        );        
        let filtered_cards = combined_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Combined-Filter (rank-adjacent, suite same): {:?}", c);
        // }

        let combined_filter = filter!(
            (cgm, ("Rank" "adjacent" using "Rank")),
            ("and"),
            (size, ">=", 3)
        );        
        let filtered_cards = combined_filter(cards.clone());
        // for c in filtered_cards {
        //     println!("Combined-Filter (adjacent, size >= 3): {:?}", c);
        // }

        // Testing more nested filter functions
        let combined_filter = filter!(
            ("Suite", "same"),
            ("and"),
            (size, ">=", 3)
        );        
        let filtered_cards = combined_filter(cards.clone());
        // println!("Combined-Filter (Suite same, size != 3): {:?}", filtered_cards);

        // // Testing more nested filter functions
        // // OR DOESNT WORK YET
        let combined_filter = filter!(
            ("Suite", "same"),
            ("or"),
            ("Rank", "same")
        );        
        let filtered_cards = combined_filter(cards.clone());
        println!("Combined-Filter (suite same, rank same): {:?}", filtered_cards);


        // this filter caused problems:
        let combined_filter = filter!(
            ("Suite", "same"),
            ("and"),
            ("Suite", "==", "Hearts")
        );
        let filtered_cards1 = combined_filter(cards.clone());

        let combined_filter = filter!(
            ("Suite", "==", "Hearts"),
            ("and"),
            ("Suite", "same")
        );        
        let filtered_cards2 = combined_filter(cards.clone());

        assert_eq!(filtered_cards1, filtered_cards2);

        let combined_filter = filter!(
            (size, "==", 3),
            ("and"),
            ("Suite", "==", "Hearts")
        );
        let filtered_cards = combined_filter(cards.clone());
        // println!("Combined-Filter (Suite same, size != 3): {:?}", filtered_cards);
    }

    #[test]
    fn test_combo() {
        let mut cgm = init_model();

        combo!(cgm, "all hearts", filter!(
            "Suite", "==", "Hearts"
        ));

        for c in cgm.gamedata
            .apply_combo("all hearts".to_string(),
                "stack".to_string())
            .iter() {
                for card in c {
                    println!("{}", card);
                }
        }
    }

    #[test]
    fn test_combo_filter() {
        let mut cgm = init_model();

        combo!(cgm, "all hearts", filter!(
            "Suite", "==", "Hearts"
        ));

        // let combo_filter = filter!(
        //     cgm, "all hearts"
        // );

        let cards: Vec<Card> = cgm.gamedata.table.locations["stack"].borrow().contents
            .iter()
            .filter_map(|c| {
                if let Component::CARD(card) = c {
                    Some(card.clone()) // Return the cloned card
                } else {
                    None // Filter out non-card components
                }
            })
            .collect();

        // let filtered_cards = combo_filter(cards.clone());
        // for cards in filtered_cards {
        //     for card in cards {
        //         println!("{}", card)
        //     }
        // }

        let combo_filter = filter!(
            cgm, not "all hearts"
        );

        let filtered_cards = combo_filter(cards.clone());
        for cards in filtered_cards {
            for card in cards {
                println!("{}", card)
            }
        }

    }

    fn print_cards(cards: Vec<Card>) {
        for c in cards {
            println!("{}", c)
        }
    }

    fn print_loc_cards(loc_cards: HashMap<LocationRef, Vec<Card>>) {
        for (locname, v) in loc_cards {
                println!("{}", locname);
                print_cards(v);
            }
    }

    #[test]
    fn test_cardset() {
        let mut cgm = init_model();

        card_on!(
            cgm,
            "hand",
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

        let a1 = cardset!(cgm, "stack");
        print_loc_cards(a1);
    
        let a2 = cardset!(cgm, "stack", "hand");
        print_loc_cards(a2);

        let b1 = cardset!(cgm, "stack" w (filter!("Suite", "==", "Hearts")));
        print_loc_cards(b1);
        
        let b2 = cardset!(cgm, "stack", "hand" w (filter!("Suite", "==", "Hearts")));
        print_loc_cards(b2);

        combo!(cgm, "all hearts", filter!(
            "Suite", "==", "Hearts"
        ));

        let d1 = cardset!(cgm, "all hearts" inn "stack");
        print_loc_cards(d1);

        let d2 = cardset!(cgm, "all hearts" inn "stack", "hand");
        print_loc_cards(d2);

        let e1 = cardset!(cgm, not "all hearts" inn "stack");
        print_loc_cards(e1);

        let e2 = cardset!(cgm, not "all hearts" inn "stack", "hand");
        print_loc_cards(e2);

        let f = cardset!(cgm, (cardposition!(cgm, "stack" 1)));
        print_loc_cards(f);

    }

    #[test]
    pub fn test_cardpos() {
        let mut cgm = init_model();

        card_on!(
            cgm,
            "hand",
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

        precedence!(cgm, "Rank", ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"));

        let card = cardposition!(cgm, "stack" 1);
        print_loc_cards(card);

        let card = cardposition!(cgm, "stack" top);
        print_loc_cards(card);

        let card = cardposition!(cgm, "stack" bottom);
        print_loc_cards(card);

        let card = cardposition!(cgm, min of (cardset!(cgm, "stack", "hand")) using prec: "Rank");
        print_loc_cards(card);
        
        let card = cardposition!(cgm, max of (cardset!(cgm, "stack", "hand")) using prec: "Rank");
        print_loc_cards(card);

        pointmap!(
            cgm,
            "Rank",
            nested: {  
                "Rank", (
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
            }
        );

        let card= cardposition!(cgm, min of (cardset!(cgm, "stack", "hand")) using pointmap: "Rank");
        print_loc_cards(card);
        
        let card = cardposition!(cgm, max of (cardset!(cgm, "stack")) using pointmap: "Rank");
        print_loc_cards(card);
        
        
    }

    #[test]
    fn test_move_card() {
        fn print_loc_hand(cgm: &CardGameModel) {
            let loc = cgm
                .gamedata
                .players
                .get(
                &cgm
                .gamedata
                .turnorder[cgm.gamedata.current])
                .unwrap()
                .locations
                .get("hand")
                .unwrap();

            println!("{}", loc.borrow());
        }
        let mut cgm = CardGameModel::new("SmallGame");

        player!(cgm, "P1", "P2");

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

        precedence!(cgm, "Rank", ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"));

        combo!(cgm, "all hearts", filter!(
            "Suite", "==", "Hearts"
        ));

        location_on!(cgm, "hand", players: "P1", "P2");

        turn_order!(cgm, random);

        // moveaction!(cgm, mv (cardset!(cgm, "stack")) to (cardset!(cgm, "hand")));

        // print_loc_hand(&cgm);

        // moveaction!(cgm, mv (cardset!(cgm, "hand")) to (cardset!(cgm, "stack")));

        // print_loc_hand(&cgm);
        
        // moveaction!(cgm, mv (cardset!(cgm, "all hearts" inn "stack")) to (cardset!(cgm, "hand")));
        
        // print_loc_hand(&cgm);

        // moveaction!(cgm, mv 1 from (cardset!(cgm, "stack")) to (cardset!(cgm, "hand")));

        // print_loc_hand(&cgm);

        // moveaction!(cgm, mv 1 from (cardset!(cgm, "stack")) to (cardset!(cgm, "hand")));

        // print_loc_hand(&cgm);

    }

    #[test]
    fn test_int() {
        let mut cgm = init_model();

        assert_eq!(int!(cgm, int!(cgm, 5, "mod", 3), "-", 1), 1);

        assert_eq!(int!(cgm, 3), 3);

        assert_eq!(
            int!(
                cgm,
                vec![1, 2, 3, 4],
                (int!(cgm, int!(cgm, 5, "mod", 3), "-", 1))
            ),
            2
        );

        assert_eq!(int!(cgm, sum of vec![1, 2, 3, 4]), 10);
        
        pointmap!(
            cgm,
            "Rank",
            nested: {  
                "Rank", (
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
            }
        );

        location_on!(cgm, "hand", players: "P1");

        card_on!(
            cgm,
            "hand",
            {
                Rank("A"),
                Suite("Hearts"),
                Color("Red")
            },
            {
                Rank("10"),
                Suite("Clubs"),
                Color("Black")
            }
        );

        assert_eq!(int!(cgm, sum of min (cardset!(cgm, "hand")), using "Rank"), 11);

        assert_eq!(int!(cgm, sum of max (cardset!(cgm, "hand")), using "Rank"), 21);

        assert_eq!(int!(cgm, sum of (cardset!(cgm, "hand")), using "Rank" gt 20), 21);

        assert_eq!(int!(cgm, sum of (cardset!(cgm, "hand")), using "Rank" lt 15), 11);

        assert_eq!(int!(cgm, max of vec![1, 2, 3, 4]), 4);

        assert_eq!(int!(cgm, min of vec![1, 2, 3, 4]), 1);
    }

    #[test]
    fn test_string() {
        let mut cgm = init_model();

        assert_eq!(string!("banana"), "banana");

        assert_eq!(string!("Rank" of cardposition!(cgm, "stack" top)), "2");

        assert_eq!(string!(vec!["a", "b", "c", "d"], 1), "b");
    }

    #[test]
    fn test_bool() {
        let mut cgm = init_model();

        assert_eq!(bool!(
            string:
            string!("Rank" of cardposition!(cgm, "stack" top)),
            "!=",
            string!("Rank" of cardposition!(cgm, "stack" bottom))),
        true);
        
        assert_eq!(bool!(
            string:
            string!("Rank" of cardposition!(cgm, "stack" top)),
            "!=",
            string!("Rank" of cardposition!(cgm, "stack" top))),
        false);

        pointmap!(
            cgm,
            "Rank",
            nested: {  
                "Rank", (
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
            }
        );

        location_on!(cgm, "hand", players: "P1");

        card_on!(
            cgm,
            "hand",
            {
                Rank("A"),
                Suite("Hearts"),
                Color("Red")
            },
            {
                Rank("10"),
                Suite("Clubs"),
                Color("Black")
            }
        );

        assert_eq!(bool!(
            int:
            int!(cgm, sum of min (cardset!(cgm, "hand")), using "Rank"),
            "==",
            int!(cgm, sum of (cardset!(cgm, "hand")), using "Rank" lt 15)),
        true);


        assert_eq!(bool!(
            cardset:
            cardset!(cgm, "hand"), "==", cardset!(cgm, "hand")),
            true
        );

        assert_eq!(bool!(
            cardset:
            cardset!(cgm, "hand"), "==", cardset!(cgm, "stack")),
            false
        );

        location_on!(cgm, "hand_empty", players: "P1", "p2");

        assert_eq!(bool!(
            cardset!(cgm, "hand_empty"), is empty),
            true
        );

        assert_eq!(bool!(
            cardset!(cgm, "hand"), is not empty),
            true
        );
        
        // player and team tests
        assert_eq!(
            bool!(
                pt:
                player_ref!(cgm, owner of cardposition!(cgm, "hand" top)),
                "==",
                player_ref!(cgm, owner of cardposition!(cgm, "hand" top))
            ),
            true
        );
        
        assert_eq!(
            bool!(
                pt:
                player_ref!(cgm, current),
                "!=",
                player_ref!(cgm, previous)
            ),
            true
        );

        assert_eq!(
            bool!(
                pt:
                player_ref!(cgm, previous),
                "!=",
                player_ref!(cgm, next)
            ),
            true
        );

        assert_eq!(
            bool!(
                pt:
                player_ref!(cgm, turnorder 1),
                "!=",
                player_ref!(cgm, next)
            ),
            true
        );
        
        assert_eq!(
            bool!(
                pt:
                team_ref!(cgm, "Team1"),
                "==",
                team_ref!(cgm, team of player_ref!(cgm, current))
            ),
            true
        );
        


    }

}
