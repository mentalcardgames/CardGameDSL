#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc};
    use crate::ast::{Card, CardGameModel, LocationRef, TDealCards};

    fn init_model() -> CardGameModel {
        let mut cgm = CardGameModel::new("player_test");

        // Ensure the macro modifies the cgm instance
        player!("Jimmy", "Kimmy", "Timmy")(&mut cgm.gamedata);

        turn_order!(("Jimmy", "Kimmy", "Timmy"))(&mut cgm.gamedata);

        team!("Team1", ("Jimmy", "Kimmy", "Timmy"))(&mut cgm.gamedata);

        location_on!("hand", players: "Jimmy", "Kimmy", "Timmy")(&mut cgm.gamedata);

        location_on!("stack", table)(&mut cgm.gamedata);

        card_on!(
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
        )(&mut cgm.gamedata);

        precedence!("Rank", ("2", "3", "4", "5", "A"))(&mut cgm.gamedata);

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

        team!("t1", ("Kimmy", "Timmy"))(&mut cgm.gamedata);
        location_on!("teamloc", team: "t1")(&mut cgm.gamedata);
        assert!(cgm.gamedata.teams.get("t1").unwrap().locations.len() == 1);

        location_on!("stack", table)(&mut cgm.gamedata);
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

        precedence!("rank", ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"))(&mut cgm.gamedata);
        precedence!("suite", ("Clubs", "Diamonds", "Hearts", "Spades"))(&mut cgm.gamedata);

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
        )(&mut cgm.gamedata);

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
        player!("Jimmy", "Kimmy", "Timmy")((&mut cgm.gamedata));
        turn_order!(random)((&mut cgm.gamedata));
        print_players(cgm.gamedata.turnorder.clone());
        
        turn_order!(("Timmy", "Jimmy", "Kimmy"))((&mut cgm.gamedata));
        print_players(cgm.gamedata.turnorder.clone());
    }

    #[test]
    fn test_filter_macro() {
        let cgm = init_model();
        
        let cards: &Vec<Card> = &cgm.gamedata.table.locations["stack"].borrow().contents;
        
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
            &cgm.gamedata, 
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
        let cgm = init_model();
        
        let cards: &Vec<Card> = &cgm.gamedata.table.locations["stack"].borrow().contents;
        
        // Combined filter
        let combined_filter = filter!(
            (&cgm.gamedata, ("Rank" "adjacent" using "Rank")), 
            ("and"), 
            ("Suite", "same")
        );        
        let filtered_cards = combined_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Combined-Filter (rank-adjacent, suite same): {:?}", c);
        // }

        let combined_filter = filter!(
            (&cgm.gamedata, ("Rank" "adjacent" using "Rank")),
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

        combo!("all hearts", filter!(
            "Suite", "==", "Hearts"
        ))(&mut cgm.gamedata);

        for c in cgm.gamedata
            .apply_combo("all hearts",
                &LocationRef::Own(String::from("stack")))
            .iter() {
                for card in c {
                    println!("{}", card);
                }
        }
    }

    #[test]
    fn test_combo_filter() {
        let mut cgm = init_model();

        combo!("all hearts", filter!(
            "Suite", "==", "Hearts"
        ))(&mut cgm.gamedata);

        // let combo_filter = filter!(
        //     cgm, "all hearts"
        // );

        let cards: &Vec<Card> = &cgm.gamedata.table.locations["stack"].borrow().contents.clone();

        // let filtered_cards = combo_filter(cards.clone());
        // for cards in filtered_cards {
        //     for card in cards {
        //         println!("{}", card)
        //     }
        // }

        let combo_filter = filter!(
            &cgm.gamedata, not "all hearts"
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
        )(&mut cgm.gamedata);

        let a1 = cardset!("stack")(&mut cgm.gamedata);
        print_loc_cards(a1);
    
        let a2 = cardset!("stack", "hand")(&mut cgm.gamedata);
        print_loc_cards(a2);

        let a3 = cardset!("hand" of player: player_ref!("Kimmy"))(&mut cgm.gamedata);
        print_loc_cards(a3);

        location_on!("team_hand", team: "Team1")((&mut cgm.gamedata));
        card_on!("team_hand", {Team("lol")})((&mut cgm.gamedata));

        let a4 = cardset!("team_hand" of team: team_ref!("Team1"))(&mut cgm.gamedata);
        print_loc_cards(a4);

        let b1 = cardset!("stack" w (filter!("Suite", "==", "Hearts")))(&mut cgm.gamedata);
        print_loc_cards(b1);
        
        let b2 = cardset!("stack", "hand" w (filter!("Suite", "==", "Hearts")))(&mut cgm.gamedata);
        print_loc_cards(b2);

        let b3 = cardset!(
            "hand" of player: player_ref!("Kimmy"),
            w (filter!("Suite", "==", "Hearts")))(&mut cgm.gamedata);
        print_loc_cards(b3);

        let b4 = cardset!(
            "team_hand" of team: team_ref!("Team1"),
            w (filter!("Team", "==", "lol")))((&mut cgm.gamedata));
        print_loc_cards(b4);

        combo!("all hearts", filter!(
            "Suite", "==", "Hearts"
        ))(&mut cgm.gamedata);

        let d1 = cardset!("all hearts" inn "stack")(&mut cgm.gamedata);
        print_loc_cards(d1);

        let d2 = cardset!("all hearts" inn "stack", "hand")(&mut cgm.gamedata);
        print_loc_cards(d2);

        let e1 = cardset!(not "all hearts" inn "stack")(&mut cgm.gamedata);
        print_loc_cards(e1);

        let e2 = cardset!(not "all hearts" inn "stack", "hand")(&mut cgm.gamedata);
        print_loc_cards(e2);

        let f = cardset!((cardposition!("stack" 1)))(&mut cgm.gamedata);
        print_loc_cards(f);
    }

    #[test]
    pub fn test_cardpos() {
        let mut cgm = init_model();

        card_on!(
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
        )(&mut cgm.gamedata);

        precedence!("Rank", ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"))(&mut cgm.gamedata);

        let card = cardposition!("stack" 1)(&mut cgm.gamedata);
        print_loc_cards(card);

        let card = cardposition!("stack" top)(&mut cgm.gamedata);
        print_loc_cards(card);

        let card = cardposition!("stack" bottom)(&mut cgm.gamedata);
        print_loc_cards(card);

        let card = cardposition!(
                min of (cardset!("stack", "hand")) using prec: "Rank")(&mut cgm.gamedata);
        print_loc_cards(card);
        
        let card = cardposition!(
            max of (cardset!("stack", "hand")) using prec: "Rank")(&mut cgm.gamedata);
        print_loc_cards(card);

        pointmap!(
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
        )(&mut cgm.gamedata);

        let card= cardposition!(
            min of (cardset!("stack", "hand")) using pointmap: "Rank")(&mut cgm.gamedata);
        print_loc_cards(card);
        
        let card = cardposition!(
            max of (cardset!("stack")) using pointmap: "Rank")(&mut cgm.gamedata);
        print_loc_cards(card);
        
        
    }

    #[test]
    fn test_int() {
        let mut cgm = init_model();

        assert_eq!(int!(int!(int!(5), "mod", int!(3)), "-", int!(1))(&cgm.gamedata), int!(1)(&cgm.gamedata));

        assert_eq!(int!(3)(&cgm.gamedata), 3);

        assert_eq!(
            int!(
                vec![1, 2, 3, 4],
                (int!(int!(int!(5), "mod", int!(3)), "-", int!(1)))
            )(&cgm.gamedata),
            2
        );

        assert_eq!(int!(sum of vec![1, 2, 3, 4])(&cgm.gamedata), 10);
        
        pointmap!(
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
        )((&mut cgm.gamedata));

        // player!(&mut cgm, "P1");

        // location_on!(&mut cgm.gamedata, "hand", players: "P1");

        card_on!(
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
        )(&mut cgm.gamedata);

        assert_eq!(int!(sum of min (cardset!("hand")), using "Rank")((&cgm.gamedata)), 11);

        assert_eq!(int!(sum of max (cardset!("hand")), using "Rank")((&cgm.gamedata)), 21);

        assert_eq!(int!(sum of (cardset!("hand")), using "Rank" gt int!(20))(&cgm.gamedata), 21);

        assert_eq!(int!(sum of (cardset!("hand")), using "Rank" lt int!(15))(&cgm.gamedata), 11);

        assert_eq!(int!(max of vec![1, 2, 3, 4])(&cgm.gamedata), 4);

        assert_eq!(int!(min of vec![1, 2, 3, 4])(&cgm.gamedata), 1);
    }

    #[test]
    fn test_string() {
        let mut cgm = init_model();

        assert_eq!(string!("banana")(&cgm.gamedata), "banana");

        assert_eq!(string!("Rank" of cardposition!("stack" top))(&cgm.gamedata), "2");

        assert_eq!(string!(vec!["a", "b", "c", "d"], int!(1))(&cgm.gamedata), "b");
    }

    #[test]
    fn test_bool() {
        let mut cgm = init_model();

        assert_eq!(bool!(
            string:
            string!("Rank" of cardposition!("stack" top)),
            "!=",
            string!("Rank" of cardposition!("stack" bottom)))(&cgm.gamedata),
        true);
        
        assert_eq!(bool!(
            string:
            string!("Rank" of cardposition!("stack" top)),
            "!=",
            string!("Rank" of cardposition!("stack" top)))(&cgm.gamedata),
        false);

        pointmap!(
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
        )(&mut cgm.gamedata);

        player!("P1", "P2")(&mut cgm.gamedata);

        location_on!("hand", players: "P1")(&mut cgm.gamedata);

        card_on!(
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
        )(&mut cgm.gamedata);

        assert_eq!(bool!(
            int:
            int!(sum of min (cardset!("hand")), using "Rank"),
            "==",
            int!(sum of (cardset!("hand")), using "Rank" lt int!(15)))(&cgm.gamedata),
        true);


        assert_eq!(bool!(
            cardset:
            cardset!("hand"), "==", cardset!("hand"))(&cgm.gamedata),
            true
        );

        assert_eq!(bool!(
            cardset:
            cardset!("hand"), "==", cardset!("stack"))(&cgm.gamedata),
            false
        );

        location_on!("hand_empty", players: "P1", "P2", "Jimmy", "Timmy", "Kimmy")((&mut cgm.gamedata));

        assert_eq!(bool!(
            cardset!("hand_empty"), is empty)(&cgm.gamedata),
            true
        );

        assert_eq!(bool!(
            cardset!("hand"), is not empty)(&cgm.gamedata),
            true
        );
        
        // player and team tests
        assert_eq!(
            bool!(
                pt:
                player_ref!(owner of cardposition!("hand" top)),
                "==",
                player_ref!(owner of cardposition!("hand" top))
            )(&cgm.gamedata),
            true
        );
        
        assert_eq!(
            bool!(
                pt:
                player_ref!(current),
                "!=",
                player_ref!(previous)
            )(&cgm.gamedata),
            true
        );

        assert_eq!(
            bool!(
                pt:
                player_ref!(previous),
                "!=",
                player_ref!(next)
            )(&cgm.gamedata),
            true
        );

        assert_eq!(
            bool!(
                pt:
                player_ref!(turnorder int!(1)),
                "!=",
                player_ref!(next)
            )(&cgm.gamedata),
            true
        );
        
        assert_eq!(
            bool!(
                pt:
                team_ref!("Team1"),
                "==",
                team_ref!(team of player_ref!(current))
            )(&cgm.gamedata),
            true
        );
    }

    #[test]
    fn test_moveaction() {
        let mut cgm = CardGameModel::new("test_moveaction");

        player!("P1", "P2", "P3")(&mut cgm.gamedata);

        turn_order!(("P1", "P2", "P3"))(&mut cgm.gamedata);

        location_on!("hand", players: "P1", "P2", "P3")(&mut cgm.gamedata);
        location_on!("stack", table)((&mut cgm.gamedata));
        card_on!(
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
        )(&mut cgm.gamedata);

        card_on!(
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
        )(&mut cgm.gamedata);


        // moveaction!(
        //     &cgm.gamedata,
        //     mv
        //     (cardset!(&cgm.gamedata, "hand" of player: player_ref!(&cgm.gamedata, "P1")))
        //     to
        //     (cardset!(&cgm.gamedata, "hand" of player: player_ref!(&cgm.gamedata, "P2"))));

        // let mv5 = moveaction!(
        //     &cgm.gamedata,
        //     mv 5 from 
        //     (cardset!(&cgm.gamedata, "hand"))
        //     to 
        //     (cardset!(&cgm.gamedata, "stack"))
        // );
        
        // mv5(vec![
        //     ((LocationRef::Own(String::from("hand")), 0),
        //     (LocationRef::Own(String::from("stack")), 0)),
        //     ((LocationRef::Own(String::from("hand")), 1),
        //     (LocationRef::Own(String::from("stack")), 1)),
        //     ((LocationRef::Own(String::from("hand")), 2),
        //     (LocationRef::Own(String::from("stack")), 2)),
        //     ((LocationRef::Own(String::from("hand")), 3),
        //     (LocationRef::Own(String::from("stack")), 3)),
        //     ((LocationRef::Own(String::from("hand")), 4),
        //     (LocationRef::Own(String::from("stack")), 4)),
        //     ]);
        
        // print_cards(cgm.gamedata.players.get("P1").unwrap().locations.get("hand").unwrap().borrow().contents.clone());

        let deal2 = moveaction!(
            deal 2 from 
            (cardset!("stack"))
            to 
            (cardset!("hand"))
        )(&mut cgm.gamedata);

        // let handlen = cgm.gamedata.get_player("P1").locations.get("hand").unwrap().borrow().contents.clone().len();

        // println!("{}", handlen);

        deal2(
            vec![
                (LocationRef::Own(String::from("stack")),
                LocationRef::Own(String::from("hand"))),
                (LocationRef::Own(String::from("stack")),
                LocationRef::Own(String::from("hand"))),
            ]
        );

        println!("{}", cgm.gamedata.players.get("P1").unwrap().locations.get("hand").unwrap().borrow().contents.clone().len());

    }


    #[test]
    fn test_condrule() {
        let mut cgm = CardGameModel::new("test_moveaction");

        player!("P1", "P2", "P3")(&mut cgm.gamedata);

        turn_order!(("P1", "P2", "P3"))(&mut cgm.gamedata);

        location_on!("hand", players: "P1", "P2", "P3")(&mut cgm.gamedata);
        
        location_on!("stack", table)(&mut cgm.gamedata);
        
        card_on!(
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
        )(&mut cgm.gamedata);

        card_on!(
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
        )(&mut cgm.gamedata);

        let b1 = bool!(
            string:
            string!("Rank" of cardposition!("stack" top)),
            "!=",
            string!("Rank" of cardposition!("stack" bottom))
        )((&mut cgm.gamedata));

        let b = moveaction!(
            deal 1 from 
            (cardset!("stack"))
            to 
            (cardset!("hand"))
        )((&mut cgm.gamedata));

        // let condrule = condrule!{
        //     (conditional: (case: b1 (deal1card)))
        // };
    }
}
