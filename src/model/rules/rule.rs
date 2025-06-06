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
                                // I think you should end the turn if you cycle to someone new
                                vec![GameFlowChange::CycleTo(cycleto.clone()), GameFlowChange::EndTurn]
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
            Self::SCORINGRULE(scoring) => {
                scoring.run(cgm, input)
            },
            _ => {
                vec![GameFlowChange::None]
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
}
