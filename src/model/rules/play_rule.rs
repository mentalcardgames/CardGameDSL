use crate::model::rules::action_rule::ActionRule;
use crate::model::rules::conditional_rule::ConditionalRule;
use crate::model::rules::trigger_rule::TriggerRule;
use crate::model::rules::optional_rule::OptionalRule;
use crate::model::rules::if_rule::IfRule;
use crate::model::rules::choose_rule::ChooseRule;


#[derive(Debug, Clone)]
pub enum PlayRule {
    CONDITIONALRULE(ConditionalRule),
    ACTIONRULE(ActionRule),
    OPTIONALRULE(OptionalRule),
    CHOOSERULE(ChooseRule),
    IFRULE(IfRule),
    TRIGGERRULE(TriggerRule),
}
