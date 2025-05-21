use ast::CardGameModel;

#[macro_use]
pub mod macros;

pub mod ast;
mod tests;


fn main() {
   let mut cgm = CardGameModel::new("BlackJack");

    player!("P1", "P2")(&mut cgm.gamedata);
    turn_order!(("P1", "P2"))(&mut cgm.gamedata);
    location_on!("hand", players: "P1", "P2")(&mut cgm.gamedata);
    location_on!("stack", table)(&mut cgm.gamedata);
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
            substage!(stage "deal-cards" player_ref!(current), endcondition!(
                untilend
            ),
            substages: ()
            rules: (
                (shuffleaction!(shuffle (cardset!("stack")))),
                (chooserule!(
                    choose: 
                        (actionrule!(
                            deal 1 from
                            (cardset!("stack"))
                            to 
                            (cardset!("hand")))
                        )
                    or:
                        (outaction!(set player_ref!(current), out of stage))
                )),
                (ifrule!(iff (bool!(int: int!(sum of min (cardset!("hand")), using "Rank"), ">", int!(21))) then 
                    (scoringrule!(set score (int!(100)), of (player_ref!(current)))),
                    // set player out of Stage
                    (outaction!(set player_ref!(current), out of stage))
                )),
                (ifrule!(iff (bool!(int: int!(sum of min (cardset!("hand")), using "Rank"), "<=", int!(21))) then 
                    (scoringrule!(set score (int!(sum of (cardset!("hand")), using "Rank" lt int!(21))), of (player_ref!(current))))
                )))
            )
        )
        rules: 
            (winnerrule!(
            highest score lt int!(21)
        ))
    )(&mut cgm);

    cgm.game_loop();
}
