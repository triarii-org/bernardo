/*
I was not able to design a "silver bullet" solution to views, messaging, layout and focus
without giving up on rust's safety warranties.

I decided that instead I will decouple the concerns, so same as layouting, focus is handled
in a separate component. When it's working the way I like it, I will then see if it can
get merged with some other component.
 */

use std::iter::Map;
use crate::primitives::rect::Rect;
use std::collections::HashMap;
use crate::io::keys::Key;
use crate::io::input_event::InputEvent;
use log::debug;
use crate::widget::widget::WID;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum FocusUpdate {
    Left,
    Right,
    Up,
    Down,
    Next,
    Prev,
}

pub trait FocusGroup {
    fn has_view(&self, widget_id : WID) -> bool;
    fn get_focused(&self) -> usize;

    /*
    returns whether focus got updated or not. It is designed to give a sound feedback, not for
    the purpose of escalation. There will be no "focus escalation".
     */
    fn update_focus(&mut self, focus_update : FocusUpdate) -> bool;

    //TODO proper error reporting
    fn override_edges(&mut self, widget_id : WID, edges : Vec<(FocusUpdate, WID)>) -> bool;
}

struct FocusGraphNode {
    widget_id : WID,
    neighbours : HashMap<FocusUpdate, WID>
}

impl FocusGraphNode {
    fn new(widget_id : WID) -> Self {
        FocusGraphNode {
            widget_id,
            neighbours: HashMap::new(),
        }
    }
}

pub struct FocusGroupImpl {
    nodes : HashMap<WID, FocusGraphNode>,
    selected : usize
}

impl FocusGroupImpl {
    pub fn new(widget_ids : Vec<WID>) -> Self {
        let mut nodes = HashMap::<WID, FocusGraphNode>::new();
        
        for widget_id in widget_ids.iter() {
            let node = FocusGraphNode::new(*widget_id);
            
            nodes.insert(*widget_id, node);
        };

        let selected  = widget_ids.first().unwrap();

        FocusGroupImpl {
            nodes,
            selected : *selected,
        }
    }
}

impl FocusGroup for FocusGroupImpl {
    fn has_view(&self, widget_id: WID) -> bool {
        self.nodes.contains_key(&widget_id)
    }

    fn get_focused(&self) -> WID {
        debug!("get_focused : {}", self.selected);
        self.selected
    }

    fn update_focus(&mut self, focus_update: FocusUpdate) -> bool {
        let curr = self.nodes.get(&self.selected).unwrap();
        let next_op = curr.neighbours.get(&focus_update);

        match next_op {
            None => false,
            Some(next) => {
                self.selected = *next;
                debug_assert!(self.nodes.contains_key(next));
                true
            }
        }
    }

    fn override_edges(&mut self, widget_id: WID, edges: Vec<(FocusUpdate, WID)>) -> bool {
        match self.nodes.get_mut(&widget_id) {
            None => false,
            Some(node) => {
                node.neighbours.clear();
                for (e, v)  in edges {
                    node.neighbours.insert(e, v);
                }
                true
            }
        }
    }
}