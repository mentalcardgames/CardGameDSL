use crate::setup;

pub fn run(){
    let bjsetup = Setup {
        "Blackjack",
        vec!["P1", "P2"],
        vec!["P1", "P2"],
        vec![("hand", players: "P1", "P2"), ("stack", table)],
        ("stack",
        {
            Rank("2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"),
            Suite("Diamond", "Hearts"),
            Color("Red")
        },
        {
            Rank("2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"),
            Suite("Spades", "Clubs"),
            Color("Black")
        }),
        ("Rank",
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
            )}),
    }
}