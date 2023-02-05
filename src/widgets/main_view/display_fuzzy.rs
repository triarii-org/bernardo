use std::borrow::Cow;
use std::rc::Rc;

use crate::widget::any_msg::{AnyMsg, AsAny};
use crate::widgets::fuzzy_search::item_provider::{Item, ItemsProvider};
use crate::widgets::main_view::display::MainViewDisplay;
use crate::widgets::main_view::msg::MainViewMsg;

pub struct DisplayItem {
    idx: usize,
    display: Rc<String>,
}

impl DisplayItem {
    pub fn new(idx: usize, display: Rc<String>) -> DisplayItem {
        DisplayItem {
            idx,
            display,
        }
    }
}

impl Item for &DisplayItem {
    fn display_name(&self) -> Cow<str> {
        Cow::Borrowed(self.display.as_str())
    }

    fn on_hit(&self) -> Box<dyn AnyMsg> {
        MainViewMsg::FuzzyBuffersHit { pos: self.idx }.boxed()
    }
}

impl ItemsProvider for Vec<DisplayItem> {
    fn context_name(&self) -> &str {
        "displays"
    }

    fn items(&self, query: String, limit: usize) -> Box<dyn Iterator<Item=Box<dyn Item + '_>> + '_> {
        Box::new(self.iter().map(|item| Box::new(item) as Box<dyn Item>))
    }
}