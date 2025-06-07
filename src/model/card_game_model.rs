use crate::model::gamedata::game_data::GameData;
use crate::model::rule_set::RuleSet;
use crate::model::stage::Stage;
use crate::model::enums::game_flow_change::GameFlowChange;
use crate::model::rules::rule::Rule;
use crate::model::end_condition::EndCondition;

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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    // pub fn play(&mut self) {
    //     // 1. the setup   rules
    //     // 2. the play    rules
    //     // 3. the scoring rules
    // }

    pub fn game_loop(&mut self) {
        println!("You are playing the Game: {}!", self.get_name());
        self.do_play_logic();
    }

    fn do_play_logic(&mut self) {
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

    fn do_stage_logic(&mut self, stage: &mut Stage) -> GameFlowChange {
        // initialize the rep counter for this stage (setting 0 for every Player)
        stage.init_stage_logic(&self.gamedata.turnorder);

        // Do the stage Logic
        while self.check_current_endconditions(stage) {
            let load_current = self.gamedata.current.clone();

            let lenstages = stage.substages.len().clone();
            for i in 0..lenstages {
                let gfc = self.do_stage_logic(&mut stage.substages[i]);
                match self.handle_game_flow_change(stage, gfc) {
                    GameFlowChange::EndStage => {
                        // return GameFlowChange::EndStage;
                        // return GameFlowChange::None;
                        // Do nothing (former stage is over)
                    },
                    GameFlowChange::EndPlay => {
                        return GameFlowChange::EndPlay;
                    },
                    GameFlowChange::EndGame => {
                        return GameFlowChange::EndGame;
                    },
                    _ => {
                        // Do Nothing
                    },
                }
            }

            self.gamedata.current = load_current;
            stage.set_current(&self.gamedata.get_current_name());

            let rules = &stage.rules.clone();
            for rule in rules {
                let gfc = self.run_rule(rule);
                match self.handle_game_flow_change(stage, gfc) {
                    GameFlowChange::EndStage => {
                        return GameFlowChange::EndStage;
                    },
                    GameFlowChange::EndPlay => {
                        return GameFlowChange::EndPlay;
                    },
                    GameFlowChange::EndGame => {
                        return GameFlowChange::EndGame;
                    },
                    _ => {
                        // Do Nothing
                    },
                }
            }
        }

        return GameFlowChange::None;
    }

    fn evaluate_endconditions(&self, condition: &Vec<EndCondition>, rep: usize) -> bool {
        for cond in condition {
            if cond.evaluate(&self, rep) {
                return false
            }
        }

        return true
    }

    fn handle_game_flow_change(&mut self, stage: &mut Stage, gfc: GameFlowChange) -> GameFlowChange {
        match gfc {
            GameFlowChange::None => {

            },
            GameFlowChange::EndTurn => {
                stage.update_reps();
                return self.handle_player_out(stage);
            },
            GameFlowChange::EndStage => {
                return GameFlowChange::EndStage
            },
            GameFlowChange::EndPlay => {
                return GameFlowChange::EndPlay
            },
            GameFlowChange::EndGame => {
                return GameFlowChange::EndGame
            },
            GameFlowChange::CycleTo(cycleto) => {
                stage.update_reps();
                self.gamedata.current = cycleto.get_pos(self);
                self.handle_player_out(stage);
            },
            GameFlowChange::OutOfStage(players) => {
                stage.update_reps();

                for p in players.iter() {
                    stage.set_player_out(p);
                }

                stage.update_reps();
                return self.handle_player_out(stage);
            },
            GameFlowChange::OutOfPlay(players) => {
                stage.update_reps();

                for p in players.iter() {
                    self.ruleset.play.set_player_out(p);
                }

                stage.update_reps();
                return self.handle_player_out(stage);
            },
            GameFlowChange::OutOfGameFail(players) => {
                stage.update_reps();

                for p in players.iter() {
                    self.ruleset.set_player_out_fail(p);
                }

                stage.update_reps();
                return self.handle_player_out(stage);
            },
            GameFlowChange::OutOfGameSuccessful(players) => {
                stage.update_reps();

                for p in players.iter() {
                    self.ruleset.set_player_out_succ(p);
                }

                stage.update_reps();
                return self.handle_player_out(stage);
            },
        }

        GameFlowChange::None
    }

    fn handle_player_out(&mut self, stage: &mut Stage) -> GameFlowChange {
        let mut current = self.gamedata.get_current_name();
        let mut all_out = true;
        for _ in 0..self.gamedata.turnorder.len() {
            if stage.is_player_out(&current)
                || self.ruleset.play.is_player_out(&current)
                || self.ruleset.is_player_out(&current) {
                    self.gamedata.set_next_player();
                    current = self.gamedata.get_current_name();
                    continue
            }

            all_out = false;
            stage.set_current(&current);                
        }

        if all_out {
            return GameFlowChange::EndStage
        }

        GameFlowChange::None
    }

    fn run_rule(&mut self, rule: &Rule) -> GameFlowChange {
        rule.run(self)
    }

    fn check_current_endconditions(&self, stage: &Stage) -> bool {
        let current_name = self.gamedata.get_current_name();
        let rep = stage.get_current_reps(&current_name);
        self.evaluate_endconditions(&stage.endconditions.clone(), rep)
    }
}