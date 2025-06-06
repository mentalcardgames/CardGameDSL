use std::io::{self, Write};

use crate::model::gamedata::game_data::GameData;
use crate::model::rule_set::RuleSet;
use crate::model::stage::Stage;
use crate::model::enums::game_flow_change::GameFlowChange;
use crate::model::enums::action_type::ActionType;
use crate::model::enums::rule_input::RuleInput;
use crate::model::rules::rule::Rule;
use crate::model::rules::play_rule::PlayRule;
use crate::model::rules::action_rule::ActionRule;
use crate::model::rules::scoring_rule::ScoringRule;
use crate::model::rules::setup_rule::SetupRule;
use crate::model::end_condition::EndCondition;
use crate::model::action::action::Action;
use crate::model::location::location_ref::LocationRef;

#[derive(Debug)]
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

        // keep track of who is still in the game
        self.ruleset.out_of_game_init(&self.gamedata.turnorder);
        self.ruleset.play.out_of_play_init(&self.gamedata.turnorder);

        // do the play stages
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
            // check_for... returns the <NEXT> Player's Name
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

            stage.set_current(&current_name);

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
            PlayRule::ACTIONRULE(action) => {
                // self.display_game_info();
                println!("{}", action.str_repr);
                let gfc = self.handle_action(action);
                self.display_game_info();

                gfc
            },
            PlayRule::CHOOSERULE(choose) => {
                println!("{}", choose.str_repr);
                let input = self.get_input(ActionType::ChooseAction);
                choose.run(self, input)
            },
            PlayRule::OPTIONALRULE(optional) => {
                println!("{}", optional.str_repr);
                let input = self.get_input(ActionType::OptionalAction);
                optional.run(self, input)
            },
            PlayRule::CONDITIONALRULE(condcases) => {
                println!("{}", condcases.str_repr);
                condcases.run(self, RuleInput::None)
            },
            PlayRule::IFRULE(ifrule) => {
                println!("{}", ifrule.str_repr);
                ifrule.run(self, RuleInput::None)
            },
            PlayRule::TRIGGERRULE(trigger) => {
                println!("{}", trigger.str_repr);
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
                scorerule.run(self);
                vec![GameFlowChange::None]
            },
            ScoringRule::Winner(winnerrule) => {
                winnerrule.run(self);
                vec![GameFlowChange::EndGame]
            }
        }
    }

    fn handle_setuprule(&self, setup: &SetupRule) {

    }

    fn handle_action(&mut self, action: &ActionRule) -> Vec<GameFlowChange> {
        match &action.action {
            Action::Deal(deal) => {
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
                // TODO:
                // should increment the stagecounter!
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
                vec![GameFlowChange::CycleTo(cycleto.clone()), GameFlowChange::EndTurn]
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
        println!("{}'s Cards:", &current_name);
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

        println!("Current Score: {}", self.gamedata.get_current().score);
        println!("============================================");
    }

    pub fn get_input(&self, actype: ActionType) -> RuleInput {
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
            _ => {RuleInput::None},
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
            // println!("Enter a move in the format:");
            // println!("from_location from_index to_location to_index");
            // println!("Example: Own:hand 0 Own:discard 0");
            // println!("Type 'done' to finish entering moves.");

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

        rulein
    }

    pub fn get_trigger_action(&self) -> RuleInput {
        RuleInput::Trigger
    }

    // this return the next player
    fn check_for_c2_and_et(&mut self, gfcs_set: &Vec<GameFlowChange>, ) -> String {
        
        let mut endturn = false;

        // TODO:
        // should increment the stagecounter!
        if gfcs_set.contains(&GameFlowChange::EndTurn) {
            let mut cycletoB = false;
            for gfc in gfcs_set.iter() {
                match gfc {
                    GameFlowChange::CycleTo(cycleto) => {
                        cycletoB = true;
                        // switch to referenced player
                        // update current (in CardGameModel and Stage)
                        let name = cycleto.get_name(self);
                        self.gamedata.update_current(cycleto.get_pos(self));
                        return name
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
                    for p in players.iter() {
                        self.ruleset.play.set_player_out(p);
                    }

                    return true
                },
                GameFlowChange::OutOfGameSuccessful(players) => {
                    for p in players.iter() {
                        self.ruleset.set_player_out_succ(p);
                    }

                    return true
                },
                GameFlowChange::OutOfGameFail(players) => {
                    for p in players.iter() {
                        self.ruleset.set_player_out_fail(p);
                    }

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
            if current_stage.is_player_out(&cname)
                || self.ruleset.play.is_player_out(&cname)
                || self.ruleset.is_player_out(&cname) {
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
        [loc] => Some(LocationRef::Own(loc.to_string())),
        ["Own", loc] => Some(LocationRef::Own(loc.to_string())),
        ["Table", loc] => Some(LocationRef::Table(loc.to_string())),
        ["Player", player, loc] => Some(LocationRef::Player(player.to_string(), loc.to_string())),
        ["Team", team, loc] => Some(LocationRef::Team(team.to_string(), loc.to_string())),
        _ => None,
    }
}