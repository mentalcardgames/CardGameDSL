use crate::ast::CardGameModel;

pub fn run() {
    let mut cgm = CardGameModel::new("BlackJack");

    player!("P1", "P2")(&mut cgm.gamedata);
    turn_order!(("P1", "P2"))(&mut cgm.gamedata);
    location_on!("hand", players: "P1", "P2")(&mut cgm.gamedata);
    location_on!("stack", table)(&mut cgm.gamedata);
    location_on!("flop", table)(&mut cgm.gamedata);
    location_on!("turn", table)(&mut cgm.gamedata);
    location_on!("river", table)(&mut cgm.gamedata);
    card_on!(
        "stack",
        {
            Rank("2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"),
            Suite("Diamond", "Hearts"),
            Color("Red")
        },
        {
            Rank("2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"),
            Suite("Spades", "Clubs"),
            Color("Black")
        }
    )(&mut cgm.gamedata);
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
            "T" => [10],
            "J" => [10],
            "Q" => [10],
            "K" => [10],
            "A" => [11, 1]
            )
        }
    )(&mut cgm.gamedata);

    stage!(
        stage "get-card" player_ref!(current), endcondition!(
            once
        ),
        substages: (
            (substage!(stage "shuffle-cards" player_ref!(current), endcondition!(
                once
            ),
            substages: ()
            rules: (
                    (shuffleaction!(shuffle (cardset!("stack"))))
                )
            )),
            (substage!(stage "deal-2-cards" player_ref!(current), endcondition!(
                once
            ),
            substages: ()
            rules: (
                    (actionrule!(
                        deal 2 from
                        (cardset!("stack"))
                        to 
                        (cardset!("hand")))
                    ),
                    // needed to deal 2 cards to all players
                    // (if not mentoined then the turn stays with the current player!)
                    // Buggie Doesnt do it twice
                    (cycleaction!(cycle to player_ref!(next)))
                )
            )),
            // TODO:
            // Betting!
            // !!! INSERT FIRST BETTING ACTION HERE !!!
            substage!(stage "flop" player_ref!(current), endcondition!(
                once
            ),
            substages: ()
            rules: (
                    (actionrule!(
                        deal 3 from
                        (cardset!("stack"))
                        to 
                        (cardset!("flop")))
                    )
                )
            ),
            // TODO:
            // Betting!
            // !!! INSERT FIRST BETTING ACTION HERE !!!
            substage!(stage "turn" player_ref!(current), endcondition!(
                once
            ),
            substages: ()
            rules: (
                    (actionrule!(
                        deal 1 from
                        (cardset!("stack"))
                        to 
                        (cardset!("turn")))
                    )
                )
            ),
            // TODO:
            // Betting!
            // !!! INSERT FIRST BETTING ACTION HERE !!!
            substage!(stage "river" player_ref!(current), endcondition!(
                once
            ),
            substages: ()
            rules: (
                    (actionrule!(
                        deal 1 from
                        (cardset!("stack"))
                        to 
                        (cardset!("river")))
                    )
                )
            )
            // TODO:
            // Betting!
            // !!! INSERT FIRST BETTING ACTION HERE !!!
            // ----------------------------------------
            // Now Show-Down. However there is no combo-precedence!
            // That should be implemented! Get the highest combo using combo-precedence on cardset (or smth like that).
            // Evaluate winner with looking at ("hand", "flop", "turn", "river")-Locations and get the HIGHEST 5-card-combo 
        )
        rules: (
        // winnerrule!(
        //     highest score lt int!(21)
        // )
        )
    )(&mut cgm);

    cgm.game_loop();
}
