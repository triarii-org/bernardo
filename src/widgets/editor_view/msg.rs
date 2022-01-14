use std::fmt::{Debug};

use crate::AnyMsg;
use crate::widgets::common_edit_msgs::CommonEditMsg;

#[derive(Clone, Debug)]
pub enum EditorViewMsg {
    EditMsg(CommonEditMsg)
}


impl AnyMsg for EditorViewMsg {}