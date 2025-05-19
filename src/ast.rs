use core::fmt;
use std::cell::RefCell;
use std::collections::{HashMap};
use std::hash::Hash;
use std::io::{self, Write};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CardGameModel {
    pub name: String,
    pub gamedata: GameData,
    pub ruleset: RuleSet,
}
impl CardGameModel {
    pub fn new(name: &str) -> CardGameModel {
        CardGameModel {
            name: name.to_string(),
            gamedata: GameData::default(),
            ruleset: RuleSet::new(),
        }
    }

    pub fn play(&mut self) {
        // 1. the setup   rules
        // 2. the play    rules
        // 3. the scoring rules
    }

    pub fn game_loop(&mut self) {
        // while all endconditions hold:
        // do the rules
        // Rule: PlayRule, ...
        // match the PlayRule and then decide what to do.
        // If ActionRule      -> This needs UI-communication! (ZKP)
        // If ChooseRule      -> Ask what kind of rule they want to do (ZKP?)
        // If ConditionalRule -> Check if Condition True and then do the rule
        // If OptionalRule    -> Decide if you want to do that

        self.do_stage_logic_play();
    }

    fn do_stage_logic_play(&mut self) {
        let lenstages = self.ruleset.play.stages.len().clone();
        for i in 0..lenstages {
            // cloning to not change the original
            let mut current_stage = self.ruleset.play.stages[i].clone();

            self.do_stage_logic(&mut current_stage);
        }
    }

    fn do_stage_logic(&mut self, stage: &mut Stage) {
        let mut endstage = false;
        let mut endplay = false;
        let mut endconds = true;

        // initialize the rep counter for this stage (setting 0 for every Player)
        stage.init_stage_logic(&self.gamedata.turnorder);

        // Loop the stage until the endconditions hold
        // Build Stage-Logic for current Player 
        let current_index = self.gamedata.current;
        // things that change the GameFlow
        let mut current_name = self.gamedata.turnorder[current_index].clone();
        let mut rep = stage.get_current_reps(&current_name);
        endconds = self.evaluate_endconditions(&stage.endconditions.clone(), rep);

        // Do the stage Logic
        while endconds {
            // rules should come after substages because of some special keywards like cycle to ...
            // do the substage-Logic
            let lenstages = stage.substages.len().clone();
            let stage_current = self.gamedata.current.clone();
            for i in 0..lenstages {
                self.do_stage_logic(&mut stage.substages[i]);
            }

            // this stage has a different sequence of players and starts at the last player that was in this stage!
            self.gamedata.current = stage_current;

            let rules = &stage.rules.clone();

            // check what gameflow changed after the set of rules!
            let gfcs = self.do_rules(rules);
            let gfcs_set = self.to_gfcs_set(gfcs);

            // evaluate GameChangeFlow now
            if gfcs_set.contains(&GameFlowChange::EndPlay) {
                endplay = true;
                break;
            }
            if gfcs_set.contains(&GameFlowChange::EndStage) {
                endstage = true;
                break;
            }

            // set player out logic in here
            let someoneout = self.check_for_out_action(stage, &gfcs_set);

            // Update reps of the current player
            stage.update_reps();

            // ==========================================
            // NEXT PLAYER:
            // (meaning current_name is now the next player)
            // change current name if the GameFlow changes
            // check_for... returns the next Player's Name
            current_name = self.check_for_c2_and_et(&gfcs_set);
            if someoneout {
                // check if all players are out
                if self.handle_player_out(&mut current_name, stage) {
                    // end stage because no Player is in teh Stage anymore (Maybe change later???)
                    endstage = true;
                    break
                }
                // next player could be out so current_name has to be updated again
                current_name = self.gamedata.get_current_name();               
            }

            // get the current reps of the current player
            rep = stage.get_current_reps(&current_name);

            // check if the endconditions hold for the next Player
            let endconditions = &stage.endconditions.clone();
            endconds = self.evaluate_endconditions(endconditions, rep);
        }
    }

    fn evaluate_endconditions(&mut self, condition: &Vec<EndCondition>, rep: usize) -> bool {
        for cond in condition {
            if cond.evaluate(&self, rep) {
                return false
            }
        }

        return true
    }

    pub fn do_rules(&mut self, rules: &Vec<Rule>) -> Vec<GameFlowChange> {
        let mut gfc: Vec<GameFlowChange> = vec![];
        for i in 0..rules.len() {
            gfc = vec![gfc, self.do_rule(rules[i].clone()).clone()].concat();
        }

        gfc
    }

    fn do_rule(&mut self, rule: Rule) -> Vec<GameFlowChange> {
        match &rule {
            Rule::PLAYRULE(play) => {
                self.handle_playrule(&play)
            },
            Rule::SCORINGRULE(scoring) => {
                self.handle_scoringrule(&scoring)
            },
            _ => {
                vec![GameFlowChange::None]
            }
            // Rule::SETUPRULE(setup) => self.handle_setuprule(&setup),
        }
    }

    fn handle_playrule(&mut self, play: &PlayRule) -> Vec<GameFlowChange> {
        match play {
            PlayRule::ACTIONRULE(actions) => {
                let gfc = self.handle_action(actions);
                self.display_game_info();

                gfc
            },
            PlayRule::CHOOSERULE(choose) => {
                let input = self.get_input(ActionType::ChooseAction);
                choose.run(self, input)
            },
            PlayRule::OPTIONALRULE(optional) => {
                let input = self.get_input(ActionType::OptionalAction);
                optional.run(self, input)
            },
            PlayRule::CONDITIONALRULE(condcases) => {
                condcases.run(self, RuleInput::None)
            },
            PlayRule::IFRULE(ifrule) => {
                ifrule.run(self, RuleInput::None)
            },
            PlayRule::TRIGGERRULE(trigger) => {
                trigger.run(self, RuleInput::None)
            },
            _ => {
                vec![GameFlowChange::None]
            }
        }
    }

    fn handle_scoringrule(&mut self, scoring: &ScoringRule) -> Vec<GameFlowChange> {
        match &scoring {
            ScoringRule::Score(scorerule) => {
                scorerule.evaluate(self);
                println!("Score: {}", self.gamedata.get_current().score);
                vec![GameFlowChange::None]
            },
            ScoringRule::Winner(winnerrule) => {
                winnerrule.evaluate(self);
                vec![GameFlowChange::EndGame]
            }
        }
    }

    fn handle_setuprule(&self, setup: &SetupRule) {

    }

    fn handle_action(&mut self, action: &ActionRule) -> Vec<GameFlowChange> {
        match &action.action {
            Action::Deal(deal) => {{}
                let input = self.get_input(ActionType::DealAction);
                deal.run(self, input)
            },
            Action::Move(mv) => {
                let input = self.get_input(ActionType::MoveAction);
                mv.run(self, input)
            },
            Action::MoveCardSet(mvcs) => {
                let input = self.get_input(ActionType::MoveCardSetAction);
                mvcs.run(self, input)
            },
            Action::EndTurn => {
                vec![GameFlowChange::EndTurn]
            },
            Action::EndStage => {
                vec![GameFlowChange::EndStage]
            },
            Action::EndPlay => {
                vec![GameFlowChange::EndPlay]
            },
            Action::EndGame => {
                vec![GameFlowChange::EndGame]
            },
            Action::CycleAction(cycleto) => {
                vec![GameFlowChange::CycleTo(cycleto.clone())]
            },
            Action::ShuffleAction(shuffle) => {
                shuffle.clone().shuffle(self);
                vec![GameFlowChange::None]
            }
            Action::OutAction(out) => {
                let gfc = out.evaluate(self);
                gfc
            },
        }
    }

    pub fn display_game_info(&self) {
        println!("============================================");
        let current_name = self.gamedata.turnorder[self.gamedata.current].clone();
        println!("{}'s turn!", &current_name);
        // TODO:
        // hard-coded to hand for now
        let hand_cards = &self.gamedata
            .get_player(&current_name)
            .locations
            .get("hand")
            .unwrap()
            .borrow()
            .contents;
        for i in 0..hand_cards.len() {
            println!("Card {}: {}", i, hand_cards[i]);
        }
        println!("============================================");
    }

    pub fn get_input(&self, actype: ActionType) -> RuleInput {
        self.display_game_info();
        // TODO:
        // UI-communication (Event making)
        // Protocol-Validation
        match actype {
            ActionType::ChooseAction => {
                self.get_choose_action()
            },
            ActionType::OptionalAction => {
                self.get_optional_action()
            },
            ActionType::TriggerAction => {
                self.get_trigger_action()
            },
            ActionType::MoveCardSetAction => {
                self.get_movecs_action()
            },
            ActionType::DealAction => {
                self.get_move_action()
            },
            ActionType::MoveAction => {
                self.get_move_action()
            },
            _ => {RuleInput::NoOp},
        }
    }

    pub fn get_choose_action(&self) -> RuleInput {
        loop {
            print!("Enter your action (as a number): ");
            // Make sure to flush stdout so the prompt appears
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                match input.trim().parse::<usize>() {
                    Ok(num) => return RuleInput::ChooseInput(num),
                    Err(_) => {
                        println!("Invalid input, please enter a number.");
                        continue;
                    }
                }
            } else {
                println!("Failed to read input.");
            }
        }
    }

    pub fn get_move_action(&self) -> RuleInput {
        let mut moves = Vec::new();

        loop {
            println!("Enter a move in the format:");
            println!("from_location from_index to_location to_index");
            println!("Example: Own:hand 0 Table:discard 0");
            println!("Or type 'done' to finish entering moves.");

            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input.");
                continue;
            }

            let input = input.trim();
            if input.eq_ignore_ascii_case("done") {
                break;
            }

            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() != 4 {
                println!("Invalid format. Please enter exactly four items.");
                continue;
            }

            let from_loc = match parse_location_ref(parts[0]) {
                Some(loc) => loc,
                None => {
                    println!("Invalid source location.");
                    continue;
                }
            };

            let from_index = match parts[1].parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid source index.");
                    continue;
                }
            };

            let to_loc = match parse_location_ref(parts[2]) {
                Some(loc) => loc,
                None => {
                    println!("Invalid destination location.");
                    continue;
                }
            };

            let to_index = match parts[3].parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid destination index.");
                    continue;
                }
            };

            moves.push(((from_loc, from_index), (to_loc, to_index)));
        }

        RuleInput::MoveInput(moves)
    }

    pub fn get_movecs_action(&self) -> RuleInput {
        RuleInput::MoveCardSet
    }

    pub fn get_optional_action(&self) -> RuleInput {
        let mut rulein = RuleInput::NoOp;

        loop {
            print!("Do you want to do the action: ");
            // Make sure to flush stdout so the prompt appears
            io::stdout().flush().unwrap();

            let input = String::new();

            if &input == "y" {
                rulein = RuleInput::DoOp;
                break;
            } else {
                println!("Failed to read input.");
                break;
            }
        }

        self.display_game_info();

        rulein
    }

    pub fn get_trigger_action(&self) -> RuleInput {
        RuleInput::Trigger
    }

    // this return the next player
    fn check_for_c2_and_et(&mut self, gfcs_set: &Vec<GameFlowChange>, ) -> String {
        
        let mut endturn = false;

        if gfcs_set.contains(&GameFlowChange::EndTurn) {
            let mut cycletoB = false;
            for gfc in gfcs_set.iter() {
                match gfc {
                    GameFlowChange::CycleTo(cycleto) => {
                        cycletoB = true;
                        // switch to referenced player
                        // update current (in CardGameModel and Stage)
                        self.gamedata.update_current(cycleto.get_pos(self));
                        return cycleto.get_name(self)
                    },
                    _ => {}
                }
            }

            if !cycletoB {
                endturn = true;
            }
        }

        if endturn {
            // switch to next player
            self.gamedata.set_next_player();
            // get current_name because the current Player has switched
            return self.gamedata.get_current_name()
        }

        self.gamedata.get_current_name()
    }

    fn check_for_out_action(&mut self, stage: &mut Stage, gfcs_set: &Vec<GameFlowChange>) -> bool {
        for gfc in gfcs_set.iter() {
            match gfc {
                GameFlowChange::OutOfStage(players) => {
                    for p in players.iter() {
                        stage.set_player_out(p);
                    }
                    return true
                },
                GameFlowChange::OutOfPlay(players) => {
                    return true
                },
                GameFlowChange::OutOfGameSuccessful(players) => {
                    return true
                },
                GameFlowChange::OutOfGameFail(players) => {
                    return true
                },
                _ => {}
            }
        }

        false
    }

    fn handle_player_out(&mut self, current_name: &mut String, current_stage: &mut Stage) -> bool {
        let mut cname = current_name.clone();
        for _ in 0..self.gamedata.turnorder.len() {
            if current_stage.is_player_out(&cname) {
                // get the next player if current player is out!
                // no turn changed!
                // just set the next player as current!
                self.gamedata.set_next_player();
                cname = self.gamedata.get_current_name();
                continue
            }

            current_stage.set_current(&cname);
            return false;
        }

        return true
    }

    fn to_gfcs_set(&self, gfcs: Vec<GameFlowChange>) -> Vec<GameFlowChange> {
        let mut gfcs_set = vec![];
        for gfc in gfcs {
            if !gfcs_set.contains(&gfc) {
                gfcs_set.push(gfc);
            }
        }

        gfcs_set
    }
}

fn parse_location_ref(s: &str) -> Option<LocationRef> {
    let parts: Vec<&str> = s.split(':').collect();
    match parts.as_slice() {
        ["Own", loc] => Some(LocationRef::Own(loc.to_string())),
        ["Table", loc] => Some(LocationRef::Table(loc.to_string())),
        ["Player", player, loc] => Some(LocationRef::Player(player.to_string(), loc.to_string())),
        ["Team", team, loc] => Some(LocationRef::Team(team.to_string(), loc.to_string())),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum GameFlowChange {
    None,
    // for current Player
    EndTurn,
    EndStage,
    EndPlay,
    EndGame,
    // for ANY Player and Players
    // TODO:
    OutOfStage(Vec<String>),
    OutOfPlay(Vec<String>),
    OutOfGameSuccessful(Vec<String>),
    OutOfGameFail(Vec<String>),
    // cycle to someone else
    CycleTo(CycleAction),
}
impl PartialEq for GameFlowChange {
    fn eq(&self, other: &Self) -> bool {
        use GameFlowChange::*;

        match (self, other) {
            (None, None)
            | (EndTurn, EndTurn)
            | (EndStage, EndStage)
            | (EndPlay, EndPlay)
            | (EndGame, EndGame) => true,

            (OutOfStage(a), OutOfStage(b))
            | (OutOfPlay(a), OutOfPlay(b))
            | (OutOfGameSuccessful(a), OutOfGameSuccessful(b)) => a == b,
            | (OutOfGameFail(a), OutOfGameFail(b)) => a == b,


            (CycleTo(_), CycleTo(_)) => true, // ignore the function, just match variant

            _ => false,
        }
    }
}
impl Eq for GameFlowChange {}


#[derive(Debug, Clone)]
pub enum ActionType {
    EndAction,
    ChooseAction,
    MoveAction,
    DealAction,
    MoveCardSetAction,
    TriggerAction,
    OptionalAction,
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub table: Table,
    pub teams: HashMap<String, Team>,
    pub players: HashMap<String, Player>,
    pub playertoteam: HashMap<String, String>,
    pub turnorder: Vec<String>,
    pub precedences: HashMap<String, Precedence>,
    pub pointmaps: HashMap<String, PointMap>,
    pub cardcombinations: HashMap<String, CardCombination>,
    // current playerindex
    pub current: usize
}
impl Default for GameData {
    fn default() -> Self {
        GameData { table: Table { locations: HashMap::new() },
                    teams: HashMap::new(),
                    players: HashMap::new(),
                    playertoteam: HashMap::new(),
                    turnorder: vec![],
                    precedences: HashMap::new(),
                    pointmaps: HashMap::new(),
                    cardcombinations: HashMap::new(),
                    current: 0
                }
    }
}
impl GameData {
    pub fn add_player(&mut self, name: &str) {
        self.players.insert(String::from(name), Player::new(String::from(name)));
    }

    pub fn add_players(&mut self, names: Vec<&str>) {
        for name in names {
            self.players.insert(String::from(name), Player::new(String::from(name)));
        }
    }

    fn get_mut_player(&mut self, name: &str) -> &mut Player {
        self.players.get_mut(name).expect(&format!("Could not find Player with name: {name}"))
    }

    pub fn get_player(&self, name: &str) -> &Player {
        self.players.get(name).expect(&format!("Could not find Player with name: {name}"))
    }

    pub fn get_player_copy(&self, name: &str) -> Player {
        self.players.get(name).expect(&format!("Could not find Player with name: {name}")).clone()
    }

    pub fn add_loc_player(&mut self, locname: &str, name: &str) {
        let player = self.get_mut_player(name);
        player.locations.insert(
            locname.to_string(),
            Rc::new(RefCell::new(Location::new(locname.to_string())))
        );
    }

    pub fn get_current(&self) -> &Player {
        let pname = self.turnorder[self.current].clone();
        self.get_player(&pname)
    }

    pub fn get_current_name(&self) -> String {
        self.turnorder[self.current].clone()
    }

    pub fn get_next_name(&self) -> String {
        let index = (self.current + 1) % self.turnorder.len();
        self.turnorder[index].clone()
    }

    pub fn update_current(&mut self, i: usize) {
        self.current = i % self.turnorder.len();
    }

    pub fn get_player_pos(&self, name: String) -> usize {
        for i in 0..self.turnorder.len() {
            if self.turnorder[i] == name {
                return i
            }
        }

        // TODO:
        // Default
        return 0
    }

    pub fn set_next_player(&mut self) {
        self.current = (self.current + 1) % self.turnorder.len();
    }

    fn get_mut_team(&mut self, name: &str) -> &mut Team {
        self.teams.get_mut(name).expect(&format!("Could not find team with name: {name}"))
    }    

    pub fn get_team(&self, name: &str) -> &Team {
        self.teams.get(name).expect(&format!("Could not find team with name: {name}"))
    }

    pub fn get_team_copy(&self, name: &str) -> Team {
        self.teams.get(name).expect(&format!("Could not find team with name: {name}")).clone()
    }

    pub fn add_loc_team(&mut self, locname: &str, teamname: &str) {
        let team = self.get_mut_team(teamname); // Uses `?` to propagate error
        team.locations.insert(
            locname.to_string(),
            Rc::new(RefCell::new(Location::new(locname.to_string())))
        );
    }

    pub fn add_loc_table(&mut self, locname: &str) {
        self.table.locations.insert(String::from(locname),
        Rc::new(RefCell::new(Location::new(String::from(locname)))));
    }

    pub fn get_mut_locs(&mut self, locname: &str) -> Vec<&mut Rc<RefCell<Location>>> {
        let mut locs: Vec<&mut Rc<RefCell<Location>>> = vec![];
    
        // Check self.table
        if let Some(l) = self.table.locations.get_mut(locname) {
            locs.push(l);
        }

        // Iterate over self.players and collect matching locations
        for (_, v) in self.players.iter_mut() {
            if let Some(loc) = v.locations.get_mut(locname) {
                locs.push(loc);
            }
        }
    
        // Iterate over self.teams and collect matching locations
        for (_, v) in self.teams.iter_mut() {
            if let Some(loc) = v.locations.get_mut(locname) {
                locs.push(loc);
            }
        }
    
        if locs.is_empty() {
            eprintln!("No Location found in the whole Game with the name: {locname}");
        }

        locs
    }

    pub fn get_location(&self, loc_ref: &LocationRef) -> &Rc<RefCell<Location>> {
        match loc_ref {
            LocationRef::Own(locname) => {
                let pname = &self.turnorder[self.current];
                self
                    .get_player(pname)
                    .locations
                    .get(locname)
                    .or_else(|| {
                        self
                            .table
                            .locations
                            .get(locname)
                        }
                    )
                    .expect(&format!("No Location found at Player '{pname}' with name: {locname}\n
                                 AND No Location found at Table with name: {locname}"))
            }
            LocationRef::Player(pname, locname) => {
                self
                    .get_player(pname)
                    .locations
                    .get(locname)
                    .expect(&format!("No Location found at Player '{pname}' with name: {locname}"))
            }
            LocationRef::Team(teamname, locname) => {
                self
                    .get_team(teamname)
                    .locations
                    .get(locname)
                    .expect(&format!("No Location found at Team '{teamname}' with name: {locname}"))
            }
            LocationRef::Table(locname) => {
                self
                    .table
                    .locations
                    .get(locname)
                    .expect(&format!("No Location found at Table with name: {locname}"))
            }
        }
    }


    // move quantity cards
    pub fn move_q_cards<'a>(&'a mut self, q: usize, mut fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> Box<dyn FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a> {
        Box::new(
            move |cardsfromto: Vec<((LocationRef, usize), (LocationRef, usize))>| {
                use std::collections::HashMap;

                if cardsfromto.len() != q {
                    panic!("Player has to move {} Cards!", q)
                }

                // Validate all locations exist
                for ((from_loc, i1), (to_loc, _)) in &cardsfromto {
                    if !fromcs.contains_key(from_loc) {
                        panic!("Source location {:?} not in fromcs", from_loc);
                    } else {
                        let location_ref = self.get_location(from_loc);
                        let location_borrow = location_ref.borrow();
                        let cards_ref = location_borrow.get_cards_ref();
                        let card = cards_ref[*i1].clone();

                        if !fromcs.get(from_loc).unwrap().contains(&card) {
                            panic!("Card not on Source-CardSet {:?} in fromcs", card);
                        }
                    }
                    if !tocs.contains_key(to_loc) {
                        panic!("Target location {:?} not in tocs", to_loc);
                    }
                }
            
                // Group by from location
                let mut grouped_from: HashMap<LocationRef, Vec<usize>> = HashMap::new();
                for ((from_loc, index), _) in &cardsfromto {
                    grouped_from.entry(from_loc.clone()).or_default().push(*index);
                }
            
                // For each from location, sort indices descending and remove cards
                let mut moved_cards: Vec<(Card, LocationRef, usize)> = vec![];
                for (loc, mut indices) in grouped_from {
                    indices.sort_unstable_by(|a, b| b.cmp(a)); // high to low
                    let from_vec = fromcs.get_mut(&loc).unwrap();
            
                    for index in indices {
                        let card = from_vec.remove(index);
                        
                        // remove from the Location
                        let location = self.get_location(&loc);
                        location.borrow_mut().remove_card_at_index(index);

                        // Find destination info from original list
                        let (_, (to_loc, to_index)) = cardsfromto
                            .iter()
                            .find(|((f, i), _)| f == &loc && *i == index)
                            .unwrap();
                        moved_cards.push((card, to_loc.clone(), *to_index));
                    }
                }
            
                // Sort by destination index descending and insert
                moved_cards.sort_by(|a, b| b.2.cmp(&a.2));
                for (card, to_loc, index) in moved_cards {
                    let location = self.get_location(&to_loc);
                    println!("{}", location.borrow().name);
                    location.borrow_mut().add_card_index(card, index);
                }
            }
        )
    }

    // moving something bound means,
    // that after the cards have been moved,
    // they stay 'glued' together and you can reference all of the cards by one index
    // (in my opinion its very 'annoying' to implement and not an important feature, but i can be wrong)
    // pub fn move_q_cards_bound<'a>(&'a self, q: usize, mut fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> impl FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a{
    //        
    // }
    pub fn move_cardset(&mut self, fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) {
        for (from_locref, cards) in fromcs.into_iter() {
            let _: Vec<Card> = cards;
            let from_loc = self.get_location(&from_locref);
            for (to_locref, _) in &tocs {
                let to_loc = self.get_location(to_locref);
                from_loc.borrow_mut().move_cards(&mut to_loc.borrow_mut(), &cards);
                break; // Only move to one destination per source
            } 
        }
    }

    

    pub fn deal_1_card<'a>(&'a mut self, fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> Box<(dyn FnOnce((LocationRef, LocationRef)) + 'a)> {
        Box::new(
            move |cardsfromto: (LocationRef, LocationRef)| {
                let from_loc = cardsfromto.0;
                let to_loc = cardsfromto.1;
                
                // Validate all locations exist
                if !fromcs.contains_key(&from_loc) {
                    panic!("Source location {:?} not in fromcs", from_loc);
                }
                if !tocs.contains_key(&to_loc) {
                    panic!("Target location {:?} not in tocs", to_loc);
                }
                
                let mut fromlocation = self.get_location(&from_loc).borrow_mut();
                let mut tolocation = self.get_location(&to_loc).borrow_mut();

                fromlocation.move_card_index(&mut *tolocation, 0, 0);        
            }
        )
    }

    pub fn deal_q_cards<'a>(&'a mut self, q: usize, fromcs: HashMap<LocationRef, Vec<Card>>, tocs: HashMap<LocationRef, Vec<Card>>) -> Box<dyn FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a> {
        Box::new(
            move |cards: Vec<((LocationRef, usize), (LocationRef, usize))>| {
                let deal_cards: Vec<(LocationRef, LocationRef)> = cards
                    .iter()
                    .map(|card| (card.0.0.clone(), card.1.0.clone()))
                    .collect();
                if q > deal_cards.len() {
                    panic!("To few cards to deal!");
                }
                for _ in 0..q {
                    self.deal_1_card(fromcs.clone(), tocs.clone())(deal_cards[0].clone());
                }
            }
        )
    }

    pub fn add_team(&mut self, name: &str, players: Vec<&str>) {
        self.teams.insert(String::from(name),
        Team::new(String::from(name), players
            .iter()
            .map(|p| String::from(*p))
            .collect()));

        for p in players {
            self.playertoteam.insert(String::from(p), String::from(name));
        }
    }

    pub fn add_precedence(&mut self, precedence: Precedence) {
        self.precedences.insert(precedence.name.clone(),precedence);
    }

    pub fn get_precedence(&self, precname: &str) -> &Precedence {
        self.precedences.get(precname).expect(&format!("No Precedence found with name: {precname}"))
    }

    pub fn add_pointmap(&mut self, pointmap: PointMap) {
        self.pointmaps.insert(pointmap.name.clone(), pointmap);
    }

    pub fn get_pointmap(&self, pname: &str) -> &PointMap {
        self.pointmaps.get(pname).expect(&format!("No PointMap with name: {pname}"))
    }

    pub fn set_turnorder(&mut self, playernames: Vec<String>) {
        self.turnorder = playernames;
    }

    pub fn add_cardcombination(&mut self, name: &str, cardcomb: CardCombination) {
        self.cardcombinations.insert(String::from(name), cardcomb);
    }

    pub fn get_combo(&self, comboname: &str) -> &CardCombination {
        self.cardcombinations.get(comboname).expect(&format!("No CardCombination with the name: {comboname}"))
    }

    pub fn apply_combo(&mut self, comboname: &str, locref: &LocationRef) -> Vec<Vec<Card>> {
        let loc = self.get_location(locref);
        let cardcombo: &CardCombination = self.get_combo(comboname);
        let cards = cardcombo
            .attributes
            .deref()(loc
                .borrow()
                .contents
                .clone());
        cards
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    FACEUP,
    FACEDOWN,
    PRIVATE,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub score: i32,
    // Location needs to be a Rc<RefCell<Location>>
    // Because it needs to be mut borrowed with other Locations
    // And because they are all in a Model that means they need to be
    // Rc<RefCell<...>
    pub locations: HashMap<String, Rc<RefCell<Location>>>,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.name.clone();
        let locs: Vec<Rc<RefCell<Location>>> = self.locations.values().cloned().collect();
        
        // Print the name first
        write!(f, "Player-name: {}", s)?;
        
        // Print each location
        for l in locs {
            write!(f, " Location: {}", l.borrow())?; 
        }

        // Print the score at the end
        write!(f, " score: {}", self.score)
    }
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            score: 0,
            locations: HashMap::new()
        }
    }

    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname))));
    }

    pub fn show_locations(&mut self) {
        if self.locations.is_empty() {
            println!("No Locations!")
        }
        for (k, _) in self.locations.iter() {
            println!("Player {}: locname={}", self.name, k.to_string())
        }
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.score == other.score
        // Not comparing locations!
    }
}
impl Eq for Player {}

impl Default for Player {
    fn default() -> Player {
        Player {
            name: format!("default"),
            score: 0,
            locations: HashMap::new()
        }
    }
}


#[derive(Debug, Clone)]
pub struct Team {
    pub teamname: String,
    pub players: Vec<String>,
    pub locations: HashMap<String, Rc<RefCell<Location>>>,
}
impl Team {
    pub fn new(name: String, players: Vec<String>) -> Team {
        Team {
            teamname: name,
            players: players,
            locations: HashMap::new()
        }
    }

    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname))));
    }

    pub fn show_locations(&mut self) {
        if self.locations.is_empty() {
            println!("No Locations!")
        }
        for (k, _) in self.locations.iter() {
            println!("Player {}: locname={}", self.teamname, k.to_string())
        }
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.teamname == other.teamname && self.players == other.players
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub locations: HashMap<String, Rc<RefCell<Location>>>,
}
impl Table {
    pub fn add_location(&mut self, locname: String) {
        self.locations.insert(locname.clone(), Rc::new(RefCell::new(Location::new(locname))));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationRef {
    Own(String),            // e.g., Own("hand")
    Player(String, String), // e.g., Player("P2", "hand")
    Table(String),          // e.g., Table("drawpile")
    Team(String, String),   // e.g., Team("TeamA", "bench")
}
impl fmt::Display for LocationRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationRef::Own(loc) => write!(f, "\"{}\"", loc),
            LocationRef::Player(player, loc) => write!(f, "\"{}\" of {}", loc, player),
            LocationRef::Table(loc) => write!(f, "\"{}\" of Table", loc),
            LocationRef::Team(team, loc) => write!(f, "\"{}\" of Team {}", loc, team),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    // TODO:
    //    AREA(Area),
    //    PILE(Pile),
    pub name: String,
    // I dont like that we can have Tokens and Cards in one Vec.
    // I would rather have a seperate Location that either takes tokens or cards.
    // This is just inconvenient for everything and can lead to unwanted bugs.
    pub contents: Vec<Card>
}
impl Location {
    pub fn new(locname: String) -> Location {
        Location { name: locname, contents: vec![]}
    }

    pub fn get_cards(self) -> Vec<Card> {
        self.contents
    }

    pub fn get_cards_ref(&self) -> &Vec<Card> {
        &self.contents
    }

    pub fn remove_card_at_index(&mut self, i: usize) -> Card {
        self.contents.remove(i)
    }

    pub fn add_card(&mut self, card: Card) {
        self.contents.push(card);
    }

    pub fn add_card_index(&mut self, card: Card, index: usize) {
        self.contents.insert(index, card);
    }

    pub fn remove_card(&mut self, card: &Card) {
        self.contents.retain(|c| {
            c != card
        });
    }

    pub fn extract_cards(self) -> Vec<Card> {
        self.contents
    }

    pub fn has_card(&self, card: &Card) -> bool {
        self.contents.contains(card)
    }

    pub fn move_card(&mut self, target: &mut Location, card: &Card) -> bool {
        if let Some(pos) = self.contents.iter().position(|c| c == card) {
            let removed = self.contents.remove(pos);
            target.contents.push(removed);
            true
        } else {
            false
        }
    }

    pub fn move_cards(&mut self, target: &mut Location, cards: &Vec<Card>) -> usize {
        let mut moved_count = 0;

        for card in cards {
            if let Some(index) = self.contents.iter().position(|c| c == card) {
                let comp = self.contents.remove(index);
                target.contents.push(comp);
                moved_count += 1;
            }
        }

        moved_count
    }

    pub fn move_card_index(
        &mut self,
        target: &mut Location,
        target_index: usize,
        card_index: usize
    ) {
        let card = self.remove_card_at_index(card_index);
        target.add_card_index(card, target_index);
    }
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.name.clone();
        write!(f, "{}\n", s)?;
        write!(f, "content-len: {}", self.contents.len())
    }
}



#[derive(Debug, Clone)]
pub struct Area {
    pub name: String,
    pub contents: Vec<Component>
}

#[derive(Debug, Clone)]
pub struct Pile {
    pub name: String,
    pub contents: Vec<Component>
}

#[derive(Debug, Clone)]
pub enum Component {
    CARD(Card),
    TOKEN,
}

impl Component {
    pub fn to_card(self) -> Option<Card> {
        match self {
            Component::CARD(card) => Some(card), // Properly destructure `Component::CARD`
            _ => None, // Return `None` if it's not a `CARD`
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub status: Status,
    pub attributes: HashMap<String, String>,
}
impl Card {
    pub fn new(attributes: HashMap<String, String>) -> Card {
        Card {
            status: Status::FACEUP,
            attributes: attributes,
        }
    }

    pub fn change_status(&mut self, status: Status) {
        self.status = status;
        // HERE HAS TO HAPPEN THE ENCRYPTION OF THE CARD!
        //
        //
        //
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement how you want to format the output
        let mut card = "Card:\n".to_string();
        for key in self.attributes.keys() {
            card = card + key + ": [";
            for value in self.attributes.get(key).iter() {
                card = card + value;
            }
            card = card + &"]\n";
        }
        write!(f, "{}", card)
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if self.attributes.len() != other.attributes.len() {
            return false;
        }
        for kv in &self.attributes {
            if *kv.1 != other.attributes[kv.0] {
                return false;
            }
        }
        return true;
    }
}
impl Eq for Card {}

#[derive(Debug, Clone)]
pub struct Precedence {
    pub name: String,
    pub attributes: HashMap<String, usize>,
}

impl Default for Precedence {
    fn default() -> Self {
        Precedence { name: format!("default"), attributes: HashMap::new() }
    }
}

impl Precedence {
    pub fn get_card_value_ref(&self, card: &Card) -> Option<usize> {
        for (_, value) in &card.attributes {
            if let Some(score) = self.attributes.get(value) {                
                return Some(*score);
            }
        }

        None
    }

    pub fn get_card_value(&self, card: Card) -> Option<usize> {
        for (_, value) in card.attributes {
            if let Some(score) = self.attributes.get(&value) {
                return Some(*score);
            }
        }

        None
    }
}


// Wrapper for function to avoid Debug issue
pub struct CardFunction(Rc<dyn Fn(Vec<Card>) -> Vec<Vec<Card>>>);

impl CardFunction {
    pub fn new(fun: Rc<dyn Fn(Vec<Card>) -> Vec<Vec<Card>>>) -> Self {
        Self(fun)
    }
}

impl Deref for CardFunction {
    type Target = dyn Fn(Vec<Card>) -> Vec<Vec<Card>>;

    fn deref(&self) -> &Self::Target {
        &*self.0 // Dereferences Rc/Arc to get the function
    }
}

impl Clone for CardFunction {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

#[derive(Clone)]
pub struct CardCombination {
    pub name: String,
    pub attributes: CardFunction,
}

// Manual Debug implementation for CardCombination
impl fmt::Debug for CardCombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CardCombination {{ name: {:?}, attributes:  functions }}",
            self.name,
            // self.attributes
        )
    }
}

#[derive(Debug, Clone)]
pub struct PointMap {
    pub name: String,
    pub entries: HashMap<String, Vec<i32>>,
}

impl PointMap {
    pub fn get_card_value_ref(&self, card: &Card) -> Option<Vec<i32>> {
        for (_, value) in &card.attributes {
            if let Some(score) = self.entries.get(value) {                
                return Some(score.clone());
            }
        }

        None
    }

    pub fn get_card_value(&self, card: Card) -> Option<Vec<i32>> {
        for (_, value) in card.attributes {
            if let Some(score) = self.entries.get(&value) {
                return Some(score.clone());
            }
        }

        println!("SOMETHING WENT WRONG!");

        None
    }   
}

pub struct Stage {
    pub name: String,
    pub endconditions: Vec<EndCondition>,
    pub substages: Vec<Stage>,
    // TODO: should be synchronuzed with the gamedata turncounter (self.current)
    pub turncounter: i32,
    pub rules: Vec<Rule>,
    pub pref: TRefPlayer,
    // Keeping track how often a Player has been in this Stage
    pub reps: HashMap<String, usize>,
    // Players Out of this Stage:
    // Are not allowed to play in this stage anymore but the others are still able to participate in the Stage
    pub playersout: HashMap<String, bool>,
    // Name of current Player (Because of some Examples of Games it is necessary)
    // Example: 
    // set Player out of stage
    // cycle to next
    pub current: String,
}

impl Stage {
    pub fn new(name: &str) -> Self {
        Stage {
            name: String::from(name),
            endconditions: vec![],
            substages: vec![],
            turncounter: 0,
            rules: vec![],
            pref: Arc::new(|gd: &GameData| {
                gd.get_player_copy(&gd.turnorder[gd.current])
            }),
            reps: HashMap::new(),
            playersout: HashMap::new(),
            current: String::from(""),
        }
    }

    pub fn add_setup_rule(&mut self, setup: Rule) {
        self.rules.push(setup);
    }

    pub fn add_play_rule(&mut self, play: Rule) {
        self.rules.push(play);
    }

    pub fn add_scoring_rule(&mut self, scoring: Rule) {
        self.rules.push(scoring);
    }

    pub fn add_sub_stage<'a>(&'a mut self, sub: Stage) {
        self.substages.push(sub);
    }

    pub fn add_end_condition(&mut self, endcond: EndCondition) {
        self.endconditions.push(endcond);
    }

    pub fn set_player_reference(&mut self, pref: TRefPlayer) {
        self.pref = pref;
    }

    // do this before u start the Stage
    fn init_reps(&mut self, players: &Vec<String>) {
        self.current = players[0].clone();

        for p in players.iter() {
            self.reps.insert(p.clone(), 0);
        }
    }

    fn init_playersout(&mut self, players: &Vec<String>) {
        self.current = players[0].clone();

        for p in players.iter() {
            self.playersout.insert(p.clone(), false);
        }
    }

    pub fn init_stage_logic(&mut self, players: &Vec<String>) {
        self.init_reps(players);
        self.init_playersout(players);
    }

    // only update if the player is ends his turn (so if something changes in the GameFlow)
    pub fn update_reps(&mut self) {
        self.reps
            .entry(self.current.clone())
            .and_modify(|v| *v += 1);
    }

    // set player out
    pub fn set_player_out(&mut self, player: &str) {
        self.playersout
            .entry(String::from(player))
            .and_modify(|v| *v = true);
    }

    pub fn set_current(&mut self, name: &str) {
        self.current = String::from(name);
    }

    pub fn get_current_reps(&self, name: &str) -> usize {
        if let Some(rep) = self.reps.get(name) {
            return *rep;
        }

        // TODO:
        // what if player is not found?
        return 0
    }

    pub fn is_player_out(&self, name: &str) -> bool {
        if let Some(b) = self.playersout.get(name) {
            if *b {
                return true
            } else {
                return false
            }
        }

        // TODO:
        // Default Value
        // Give a message or crash game if Player is not found!
        false
    }
}
impl<'a> Clone for Stage {
    fn clone(&self) -> Self {
        Stage {
            name: self.name.clone(),
            endconditions: self.endconditions.clone(),
            substages: self.substages.clone(),
            turncounter: self.turncounter,
            rules: self.rules.clone(),
            pref: Arc::clone(&self.pref),
            reps: self.reps.clone(),
            playersout: self.playersout.clone(),
            current: self.current.clone(),
        }
    }
}
impl fmt::Debug for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stage")
            .field("name", &self.name)
            .field("endconditions", &self.endconditions)
            .field("substages", &self.substages)
            .field("turncounter", &self.turncounter)
            .field("rules", &self.rules)
            .field("pref", &"<function>") // Custom placeholder for non-Debug field
            .finish()
    }
}
// Object-safe trait for cloning boxed functions
pub trait CloneableFn: Fn(&CardGameModel) -> bool {
    fn clone_box(&self) -> Box<dyn CloneableFn>;
}

// Implement the object-safe trait for all compatible Fn types
impl<T> CloneableFn for T
where
    T: Fn(&CardGameModel) -> bool + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFn> {
        Box::new(self.clone())
    }
}

// Now implement Clone for the boxed trait object
impl Clone for Box<dyn CloneableFn> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// Finally, define the Condition struct
pub struct Condition {
    pub condition: Arc<dyn Fn(&CardGameModel) -> bool>,
}
impl Condition {
    pub fn evaluate(&self, cgm: &CardGameModel) -> bool {
        (*self.condition)(cgm)
    }
}
impl Clone for Condition {
    fn clone(&self) -> Self {
        Condition {
            condition: Arc::clone(&self.condition)
        }
    }
}
impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<Condition>")
    }
}

pub struct EndCondition {
    pub condition: Arc<dyn Fn(&CardGameModel, usize) -> bool>,
}
impl EndCondition {
    pub fn evaluate(&self, cgm: &CardGameModel, reps: usize) -> bool {
        (*self.condition)(cgm, reps)
    }
}
impl Clone for EndCondition {
    fn clone(&self) -> Self {
        EndCondition {
            condition: Arc::clone(&self.condition)
        }
    }
}
impl fmt::Debug for EndCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<Condition>")
    }
}

#[derive(Debug, Clone)]
pub struct RuleSet {
    pub setup: Setup,
    pub play: Play,
    pub scoring: Scoring,
}
impl RuleSet {
    pub fn new() -> RuleSet {
        RuleSet {
            setup: Setup {setuprules: vec![]},
            play: Play { endconditions: vec![], stages: vec![], current: String::from(""), reps: HashMap::new()},
            scoring: Scoring {scoringrules: vec![]}
        }
    }

    pub fn assign_setup(&mut self, setup: Setup) {
        self.setup = setup;
    }

    pub fn assign_play(&mut self, play: Play) {
        self.play = play;
    }

    pub fn assign_scoring(&mut self, scoring: Scoring) {
        self.scoring = scoring;
    }
}

#[derive(Debug, Clone)]
pub enum RuleInput {
    None,
    DoOp,
    NoOp,
    Trigger,
    ChooseInput(usize),
    MoveCardSet,
    MoveInput(Vec<((LocationRef, usize), (LocationRef, usize))>),
}

#[derive(Debug, Clone)]
pub enum Rule {
    SETUPRULE(SetupRule),
    SCORINGRULE(ScoringRule),
    PLAYRULE(PlayRule),
}
impl Rule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match self {
            Self::PLAYRULE(play) => {
                match play {
                    PlayRule::ACTIONRULE(action) => {
                        match &action.action {
                            Action::Move(mv) => {
                                mv.run(cgm, input)
                            },
                            Action::Deal(deal) => {
                                deal.run(cgm, input)
                            },
                            Action::MoveCardSet(mvcs) => {
                                mvcs.run(cgm, input)
                            },
                            Action::CycleAction(cycleto) => {
                                vec![GameFlowChange::CycleTo(cycleto.clone())]
                            },
                            Action::EndTurn => {
                                vec![GameFlowChange::EndTurn]
                            },
                            Action::EndStage => {
                                vec![GameFlowChange::EndStage]
                            },
                            Action::EndPlay => {
                                vec![GameFlowChange::EndPlay]
                            },
                            Action::EndGame => {
                                vec![GameFlowChange::EndGame]
                            },
                            Action::ShuffleAction(shuffle) => {
                                shuffle.clone().shuffle(cgm);
                                vec![GameFlowChange::None]
                            },
                            Action::OutAction(out) => {
                                out.evaluate(cgm)
                            }
                        }
                    },
                    PlayRule::CHOOSERULE(choose) => {
                        choose.run(cgm, input)
                    },
                    PlayRule::CONDITIONALRULE(conditional) => {
                        conditional.run(cgm, input)
                    },
                    PlayRule::OPTIONALRULE(optional) => {
                        optional.run(cgm, input)
                    },
                    PlayRule::TRIGGERRULE(trigger) => {
                        trigger.run(cgm, input)
                    },
                    PlayRule::IFRULE(ifrule) => {
                        ifrule.run(cgm, input)
                    },
                }
            },
            _ => {
                vec![GameFlowChange::None]
            },
        }
    }

    pub fn get_action_type(&self) -> ActionType {
        match self {
            Self::PLAYRULE(play) => {
                match play {
                    PlayRule::ACTIONRULE(action) => {
                        match &action.action {
                            Action::Move(_) => {ActionType::MoveAction},
                            Action::Deal(_) => {ActionType::DealAction},
                            Action::MoveCardSet(_) => {ActionType::MoveCardSetAction},
                            _ => {ActionType::EndAction}
                        }
                    },
                    PlayRule::CHOOSERULE(_) => {
                        ActionType::ChooseAction
                    },
                    // PlayRule::CONDITIONALRULE(conditional) => {
                    //     ActionType::ConditionalAction
                    // },
                    PlayRule::OPTIONALRULE(_) => {
                        ActionType::OptionalAction
                    },
                    PlayRule::TRIGGERRULE(_) => {
                        ActionType::TriggerAction
                    },
                    // PlayRule::IFRULE(ifrule) => {
                    //     ActionType::IfAction
                    // },
                    // Default return type: Needs to be changed later
                    _ => {ActionType::OptionalAction}
                }
            },
            // Default return type: Needs to be changed later
            _ => {ActionType::OptionalAction},
        }
    }
}


#[derive(Debug, Clone)]
pub enum PlayRule {
    CONDITIONALRULE(ConditionalRule),
    ACTIONRULE(ActionRule),
    OPTIONALRULE(OptionalRule),
    CHOOSERULE(ChooseRule),
    IFRULE(IfRule),
    TRIGGERRULE(TriggerRule),
}

#[derive(Debug, Clone)]
pub struct ConditionalCase {
    pub condition: Condition,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct ConditionalRule {
    pub condcases: Vec<ConditionalCase>,
}
impl ConditionalRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, _: RuleInput) -> Vec<GameFlowChange> {
        let mut gfs = vec![];

        for i in 0..self.condcases.len() {    
            if self.condcases[i].condition.evaluate(cgm) {
                for j in 0..self.condcases[i].rules.len() { 
                    let actype= self.condcases[i].rules[j].get_action_type();
                    let rulein = cgm.get_input(actype);
                    gfs = vec![gfs, self.condcases[i].rules[j].run(cgm, rulein).clone()].concat();
                }
            }
        }

        gfs
    }
}

#[derive(Debug, Clone)]
pub struct IfRule {
    pub condition: Condition,
    pub rules: Vec<Rule>,
}
impl IfRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, _: RuleInput) -> Vec<GameFlowChange> {
        let mut gfs = vec![];

        if self.condition.evaluate(cgm) {
            for i in 0..self.rules.len() { 
                let actype= self.rules[i].get_action_type();
                let rulein = cgm.get_input(actype);
                gfs = vec![gfs, self.rules[i].run(cgm, rulein).clone()].concat();
            }
        }
    
        gfs
    }
}

#[derive(Debug, Clone)]
pub struct OptionalRule {
    pub rules: Vec<Rule>,
}
impl OptionalRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match input {
            RuleInput::DoOp => {
                let mut gfs = vec![];
                for i in 0..self.rules.len() {
                    let actype= self.rules[i].get_action_type();
                    let rulein = cgm.get_input(actype);
                    gfs = vec![gfs, self.rules[i].run(cgm, rulein).clone()].concat();
                }

                gfs
            },
            _ => {
                vec![GameFlowChange::None]
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChooseRule {
    pub rules: Vec<Rule>,
}
impl ChooseRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match input {
            RuleInput::ChooseInput(i) => {
                let actype= self.rules[i].get_action_type();
                let input = cgm.get_input(actype);
                self.rules[i].run(cgm, input)
            },
            _ => {
                vec![GameFlowChange::None]
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TriggerRule {
    pub rules: Vec<Rule>,
}
impl TriggerRule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, _: RuleInput) -> Vec<GameFlowChange> {
        let mut gfs = vec![];

        for i in 0..self.rules.len() { 
            let actype= self.rules[i].get_action_type();
            let rulein = cgm.get_input(actype);
            gfs = vec![gfs, self.rules[i].run(cgm, rulein).clone()].concat();
        }
    
        gfs
    }
}

pub trait CloneActionFn: Fn(&Vec<((LocationRef, usize),(LocationRef, usize))>) + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneActionFn>;
}

impl<T> CloneActionFn for T
where
    T: Fn(&Vec<((LocationRef, usize),(LocationRef, usize))>) + Clone + 'static + Send + Sync,
{
    fn clone_box(&self) -> Box<dyn CloneActionFn> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneActionFn> {
    fn clone(&self) -> Box<dyn CloneActionFn> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct ActionRule {
    pub action: Action
}

#[derive(Debug, Clone)]
pub enum EndAction {
    EndTurn,
    EndStage,
    EndGame,
}

#[derive(Clone)]
pub enum Action {
    Move(MoveAction),
    Deal(DealAction),
    MoveCardSet(MoveCSAction),
    CycleAction(CycleAction),
    EndTurn,
    EndStage,
    EndPlay,
    EndGame,
    ShuffleAction(ShuffleAction),
    OutAction(OutAction),
}
impl Do for Action {
    fn play<'a>(&self, cgm: &'a mut CardGameModel) -> PlayOutput<'a> {
        match self {
            Self::Move(mv) => {mv.play(cgm)},
            Self::Deal(deal) => {deal.play(cgm)},
            Self::MoveCardSet(mvcs) => {mvcs.play(cgm)},
            _ => {PlayOutput::EndAction},
        }
    }
}
impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move(_) => f.write_str("Action::Move(<closure>)"),
            Action::Deal(_) => f.write_str("Action::Deal(<closure>)"),
            Action::MoveCardSet(_) => f.write_str("Action::MoveCardSet(<closure>)"),
            _ => f.write_str("Action::EndAction(<closure>)"),
        }
    }
}
impl Action {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match self {
            Self::Move(mv) => mv.run(cgm, input),
            Self::Deal(deal) => deal.run(cgm, input),
            Self::MoveCardSet(mvcs) => mvcs.run(cgm, input),
            _ => {
                vec![GameFlowChange::None]
            }
        }
    }
}

pub enum PlayOutput<'a> {
    Move(Box<dyn FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a>),
    MoveCS(()),
    EndAction,
}

pub trait Do {
    fn play<'a>(&self, cgm: &'a mut CardGameModel) -> PlayOutput<'a>;
}

#[derive(Clone)]
pub struct MoveAction {
    pub action: TMoveCards,
}
impl Do for MoveAction {
    fn play<'a>(&self, cgm: &'a mut CardGameModel) -> PlayOutput<'a> {
        PlayOutput::Move((self.action)(cgm))
    }
}
impl MoveAction {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match input {
            RuleInput::MoveInput(mv) => {
                ((self.action)(cgm))(mv);
                vec![GameFlowChange::None]
            },
            _ => {
                vec![GameFlowChange::None]
            }
        }
    }
}

#[derive(Clone)]
pub struct DealAction {
    pub action: TMoveCards,
}
impl Do for DealAction {
    fn play<'a>(&self, cgm: &'a mut CardGameModel) -> PlayOutput<'a> {
        PlayOutput::Move((self.action)(cgm))
    }
}
impl DealAction {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match input {
            RuleInput::MoveInput(mv) => {
                ((self.action)(cgm))(mv);
                vec![GameFlowChange::None]
            },
            _ => {
                vec![GameFlowChange::None]
            }
        }
    }
}

#[derive(Clone)]
pub struct MoveCSAction {
    pub action: TMoveCardSet,
}
impl Do for MoveCSAction {
    fn play<'a>(&self, cgm: &'a mut CardGameModel) -> PlayOutput<'a> {
        PlayOutput::MoveCS((self.action)(cgm))
    }
}
impl MoveCSAction {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel, input: RuleInput) -> Vec<GameFlowChange> {
        match input {
            RuleInput::MoveCardSet => {
                ((self.action)(cgm));
                vec![GameFlowChange::None]
            },
            _ => {
                vec![GameFlowChange::None]
            }
        }
    }
}

pub struct CycleAction {
    pub pref: Arc<dyn for<'a> Fn(&'a CardGameModel) -> &'a Player + 'static>,
}
impl CycleAction {
    pub fn get_name(&self, cgm: &CardGameModel) -> String  {
        ((self.pref)(cgm)).name.clone()
    }

    pub fn get_pos(&self, cgm: &CardGameModel) -> usize {
        let pname = self.get_name(cgm);
        for i in 0..cgm.gamedata.turnorder.len() {
            if cgm.gamedata.turnorder[i] == pname {
                return i;
            }
        }

        // TODO:
        // Default return
        0
    }
}
impl Clone for CycleAction {
    fn clone(&self) -> Self {
        CycleAction {
            pref: Arc::clone(&self.pref)
        }
    }
}
impl std::fmt::Debug for CycleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action::CycleAction(<closure>)")
    }
}

pub struct ShuffleAction {
    pub cardset: TCardSet
}
impl std::fmt::Debug for ShuffleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action::ShuffleAction(<closure>)")
    }
}
impl Clone for ShuffleAction {
    fn clone(&self) -> Self {
        ShuffleAction {
            cardset: Arc::clone(&self.cardset),
        }
    }
}
impl ShuffleAction {
    pub fn shuffle(&mut self, cgm: &mut CardGameModel) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        use std::collections::HashMap;
        use std::rc::Rc;
        use std::cell::RefCell;

        // Get the cardset for this shuffle
        let cardset: HashMap<LocationRef, Vec<Card>> = (self.cardset)(&cgm.gamedata);

        for (loc_ref, cards_to_shuffle) in cardset.iter() {
            let location: &Rc<RefCell<Location>> = cgm.gamedata.get_location(loc_ref);
            let mut loc = location.borrow_mut();

            // Get mutable reference to location contents
            let contents = &mut loc.contents;

            // Find the indices of the cards to shuffle in the contents list
            let indices: Vec<usize> = contents
                .iter()
                .enumerate()
                .filter_map(|(i, card)| {
                    if cards_to_shuffle.contains(card) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();

            // Shuffle the indices logically by shuffling the corresponding cards
            let mut selected_cards: Vec<Card> = indices.iter().map(|&i| contents[i].clone()).collect();

            // Shuffle the cards randomly (ensure you have a reproducible RNG if needed)
            let mut rng = thread_rng();
            selected_cards.shuffle(&mut rng);

            // Put the shuffled cards back into the original indices
            for (i, &idx) in indices.iter().enumerate() {
                contents[idx] = selected_cards[i].clone();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum OutOf {
    Stage,
    Play,
    GameSuccessful,
    GameFail,
}

// Just for Player for now 
// TODO:
// Do it for Team
pub struct OutAction {
    pub pref: TRefPlayer,
    pub outof: OutOf,
}
impl Clone for OutAction {
    fn clone(&self) -> Self {
        OutAction {
            pref: Arc::clone(&self.pref),
            outof: self.outof.clone(),
        }
    }
}
impl std::fmt::Debug for OutAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action::OutAction(<closure>)")
    }
}
impl OutAction {
    pub fn evaluate(&self, cgm: &CardGameModel) -> Vec<GameFlowChange> {
        let pname = vec![(self.pref)(&cgm.gamedata).name];

        match self.outof {
            OutOf::Stage => {
                vec![GameFlowChange::OutOfStage(pname)]
            },
            OutOf::Play => {
                vec![GameFlowChange::OutOfPlay(pname)]
            },
            OutOf::GameSuccessful => {
                vec![GameFlowChange::OutOfGameSuccessful(pname)]
            },
            OutOf::GameFail => {
                vec![GameFlowChange::OutOfGameFail(pname)]
            },
        }
    } 
}


#[derive(Debug, Clone)]
pub enum ScoringRule {
    Score(ScoreRule),
    Winner(WinnerRule)
}

pub struct ScoreRule {
    pub set: bool,
    pub score: Arc<dyn Fn(&GameData) -> i32>,
    pub pref: TRefPlayer,
}
impl std::fmt::Debug for ScoreRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ScoreRule(<closure>)")
    }
}
impl Clone for ScoreRule {
    fn clone(&self) -> Self {
        ScoreRule {
            set: self.set.clone(),
            score: Arc::clone(&self.score),
            pref: Arc::clone(&self.pref)
        }
    }
}
impl ScoreRule {
    pub fn evaluate(&self, cgm: &mut CardGameModel) {
        let score = (self.score)(&cgm.gamedata);
        let name = (self.pref)(&cgm.gamedata).name;

        let player = cgm.gamedata.get_mut_player(&name);
        if self.set {
            player.score = score;
        } else {
            player.score += score;
        }
    }
}

pub struct WinnerRule {
    // evaluates to the winning Player name
    pub winner: Arc<dyn Fn(&CardGameModel) -> &Player>,
    pub str_repr: String,
}
impl std::fmt::Debug for WinnerRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WinnerRule(<closure>)")
    }
}
impl Clone for WinnerRule {
    fn clone(&self) -> Self {
        WinnerRule {
            winner: Arc::clone(&self.winner),
            str_repr: self.str_repr.clone()
        }
    }
}
impl WinnerRule {
    pub fn evaluate(&self, cgm: &CardGameModel) {
        let winner = (self.winner)(cgm);
        println!("The Winner is: {}!", winner.name);
    } 
}


#[derive(Debug, Clone)]

pub enum SetupRule {
    // not specified in the AST
}

#[derive(Debug, Clone)]
pub struct Setup {
    pub setuprules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Scoring {
    pub scoringrules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Play {
    pub endconditions: Vec<Condition>,
    pub stages: Vec<Stage>,
    // current player
    pub current: String,
    pub reps: HashMap<String, usize>,
}
impl Clone for Play {
    fn clone(&self) -> Self {
        Play {
            endconditions: vec![], // or panic!(), or skip, or clone dummy data
            stages: self.stages.clone(),
            current: self.current.clone(),
            reps: self.reps.clone(),
        }
    }
}
impl Play {
    pub fn add_endcondition(&mut self, end_cond: Condition) {
        self.endconditions.push(end_cond);
    }

    pub fn add_stage(&mut self, stage: Stage) {
        self.stages.push(stage);
    }
}

struct Filter {
    // pub func: 
}



pub type TMoveCards   = Arc<dyn for<'a> Fn(&'a mut CardGameModel) -> Box<dyn FnOnce(Vec<((LocationRef, usize), (LocationRef, usize))>) + 'a>>;
pub type TMoveCardSet = Arc<dyn Fn(&mut CardGameModel) + Send + Sync + 'static>;
pub type TCardSet     = Arc<dyn Fn(&GameData) -> HashMap<LocationRef, Vec<Card>> + Send + Sync + 'static>;
pub type TRefPlayer   = Arc<dyn Fn(&GameData) -> Player>;
pub type TRefTeam     = Arc<dyn Fn(&GameData) -> Team>;
