use crate::primitives::common_edit_msgs::CommonEditMsg;
use crate::*;

#[derive(Clone, Debug)]
pub enum ContextBarWidgetMsg {
    Close,
    Edit(CommonEditMsg),
    Hit,
}

impl AnyMsg for ContextBarWidgetMsg {}
