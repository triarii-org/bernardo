use std::rc::Rc;

use crate::*;
use crate::*;
use crate::*;

pub struct DisplayItem {
    idx: usize,
    display: Rc<String>,
}

impl DisplayItem {
    pub fn new(idx: usize, display: Rc<String>) -> DisplayItem {
        DisplayItem { idx, display }
    }
}

impl Item for &DisplayItem {
    fn display_name(&self) -> Rc<String> {
        self.display.clone()
    }

    fn on_hit(&self) -> Box<dyn AnyMsg> {
        MainViewMsg::FuzzyBuffersHit { pos: self.idx }.boxed()
    }
}

impl ItemsProvider for Vec<DisplayItem> {
    fn context_name(&self) -> Rc<String> {
        Rc::new("displays".to_string())
    }

    fn items(&self, query: String, limit: usize) -> Box<dyn Iterator<Item = Box<dyn Item + '_>> + '_> {
        Box::new(
            self.iter()
                .filter(move |f| f.display.contains(&query))
                .take(limit)
                .map(|item| Box::new(item) as Box<dyn Item>),
        )
    }
}
