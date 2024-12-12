use ast::GameData;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub cg); // synthesized by LALRPOP

pub mod ast;


fn main() {
    let mut gd = GameData::default();
    println!("Hello World!");
    // println!("{:?}", cg::StatusParser::new().parse("faceup"));
    // println!("{:?}", cg::CollectionParser::new().parse("String(abc, abc, abc)"));
    // println!("{:?}", cg::CollectionParser::new().parse("Int(31, 42, 1)"));
    // println!("{:?}", cg::PlayerCollectionParser::new().parse("Player(a31, b42, c1)"));
    // println!("{:?}", cg::CreatePlayerParser::new().parse("Player Jimmy, Timmy, Kimmy,"));
    // println!("{:?}", cg::CreateTeamParser::new().parse("Team KAKA with Player(Jimmy, Timmy, Kimmy,),"));
    // println!("{:?}", cg::TeamCollectionParser::new().parse("Team(KAKA, Popo, ufwh, oij, af)"));
    // let _ = cg::CreatePlayerParser::new().parse(&mut gd, "Player Jimmy, Timmy, Kimmy,");
    // let _ = cg::CreateTeamParser::new().parse(&mut gd, "Team KAKA with Player(Jimmy, Timmy, Kimmy,)");
    // let _ = cg::CreateLocParser::new().parse(&mut gd, "Location Location(Hand,) on Player(Jimmy, Timmy, Kimmy,)");
    // let _ = cg::CreateLocParser::new().parse(&mut gd, "Location Location(Hand,) on Team(KAKA,)");
    // let _ = cg::CreateTurnorderParser::new().parse(&mut gd, "Turnorder Player(Jimmy, Timmy, Kimmy,)");
    // let _ = cg::CreateLocParser::new().parse(&mut gd, "Location Location(Stock) on table");
    let _ = cg::CreateCardParser::new().parse(&mut gd, "Card on Stock:
         Rank (Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen , King , Ace)
         : for Suit (Spades, Clubs), for Color (Black)");
    // let _ = cg::CreatePrecParser::new().parse(&mut gd,"Precedence Rankorder on Rank (Ace, Two, Three, Four, Five, Six,
    //                         Seven, Eight, Nine, Ten, Jack, Queen, King)");
    // let _ = cg::CreatePointMapParser::new().parse(&mut gd, "Points Values on Rank (Ace: 1, King: 10, Queen: 10, Jack: 10, Two: 2,
    //                         Three: 3, Four: 4, Five: 5, Six: 6, Seven: 7, Eight: 8, Nine: 9, Ten: 10)");
    let _ = cg::CreatePointMapParser::new().parse(&mut gd, "Points Values on (Rank Ace: 1)");

    let fil1 = cg::FilterParser::new().parse(&mut gd, "Rank same").unwrap();
    let test = vec![("Rank".to_string(), "1".to_string()),
    ("Rank".to_string(), "2".to_string()),
    ("Black".to_string(), "1".to_string()),
    ("Rank".to_string(), "1".to_string()),
    ("Rank".to_string(), "1".to_string())];
    let f1: Vec<(String, String)> = test.iter()
        .filter(|x| fil1(&(x.0.to_string())))
        .map(|x| (x.0.clone(), x.1.clone()))
        .collect();

    // for element in f1.iter() {
    //     println!("{}, {}", element.0, element.1);
    // }

    let fil2 = cg::FilterParser::new().parse(&mut gd, "Rank distinct").unwrap();
    let f2: Vec<(String, String)> = test.iter()
    .filter(|x| fil2(&(x.0.to_string())))
    .map(|x| (x.0.clone(), x.1.clone()))
    .collect();

    for element in f2.iter() {
        println!("{}, {}", element.0, element.1);
    }



    println!("{:?}", gd);
}

#[test]
fn cg() {
    // assert!(cg::StatusParser::new().parse("faceup").is_ok());
    // assert!(cg::StatusParser::new().parse("private").is_ok());
    // assert!(cg::StatusParser::new().parse("facedown").is_ok());
    // assert!(cg::CollectionParser::new().parse("(abc, abc, abc)").is_ok());
}