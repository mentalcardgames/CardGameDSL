use std::io::{self, Write};

use crate::model::location::location_ref::LocationRef;
use crate::model::rules::scoring_rule::ScoringRule;
use crate::model::rules::play_rule::PlayRule;
use crate::model::rules::setup_rule::SetupRule;
use crate::model::action::action::Action;
use crate::model::card_game_model::CardGameModel;
use crate::model::enums::rule_input::RuleInput;
use crate::model::enums::game_flow_change::GameFlowChange;
use crate::model::enums::action_type::ActionType;



#[derive(Debug, Clone)]
pub enum Rule {
    SETUPRULE(SetupRule),
    SCORINGRULE(ScoringRule),
    PLAYRULE(PlayRule),
}
impl Rule {
    pub fn run<'a>(&self, cgm: &'a mut CardGameModel) -> GameFlowChange {
        self.display_game_info(cgm);
        println!("{}", self.get_str_repr());

        let actype = self.get_action_type();
        let input = self.get_input(actype);

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
                                // I think you should end the turn if you cycle to someone new
                                GameFlowChange::CycleTo(cycleto.clone())
                            },
                            Action::EndTurn => {
                                // TODO:
                                // should increment the stagecounter!
                                GameFlowChange::EndTurn
                            },
                            Action::EndStage => {
                                GameFlowChange::EndStage
                            },
                            Action::EndPlay => {
                                GameFlowChange::EndPlay
                            },
                            Action::EndGame => {
                                GameFlowChange::EndGame
                            },
                            Action::ShuffleAction(shuffle) => {
                                shuffle.clone().shuffle(cgm);
                                GameFlowChange::None
                            },
                            Action::OutAction(out) => {
                                out.run(cgm)
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
            // scoring and winner rule
            Self::SCORINGRULE(scoring) => {
                match scoring {
                    ScoringRule::Score(score) => {
                        score.run(cgm)
                    },
                    ScoringRule::Winner(winner) => {
                        winner.run(cgm)
                    }
                }
            },
            _ => {
                GameFlowChange::None
            }
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
                    _ => {ActionType::None}
                }
            },
            // Default return type: Needs to be changed later
            _ => {ActionType::None},
        }
    }

    pub fn get_str_repr(&self) -> String {
        match self {
            Self::PLAYRULE(p) => {
                match p {
                    PlayRule::ACTIONRULE(a) => {
                        a.str_repr.clone()
                    },
                    PlayRule::CHOOSERULE(a) => {
                        a.str_repr.clone()
                    },
                    PlayRule::CONDITIONALRULE(a) => {
                        a.str_repr.clone()
                    },
                    PlayRule::IFRULE(a) => {
                        a.str_repr.clone()
                    },
                    PlayRule::TRIGGERRULE(a) => {
                        a.str_repr.clone()
                    },
                    PlayRule::OPTIONALRULE(a) => {
                        a.str_repr.clone()
                    },
                }
            },
            Self::SCORINGRULE(s) => {
                match s {
                    ScoringRule::Score(s) => {
                        s.str_repr.clone()
                    },
                    ScoringRule::Winner(w) => {
                        w.str_repr.clone()
                    },
                }
            },
            _ => {format!("")}
        }
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

    pub fn display_game_info(&self, cgm: &CardGameModel) {
        println!("============================================");
        let current_name = cgm.gamedata.get_current_name();
        println!("{}'s Cards:", &current_name);
        // TODO:
        // hard-coded to hand for now
        let hand_cards = &cgm.gamedata
            .get_player(&current_name)
            .locations
            .get("hand")
            .unwrap()
            .borrow()
            .contents;
        for i in 0..hand_cards.len() {
            println!("Card {}: {}", i, hand_cards[i]);
        }

        println!("Current Score: {}", cgm.gamedata.get_current().score);
        println!("============================================");
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
