use std::iter::empty;
use std::rc::Rc;

use log::error;

use crate::*;

impl ListWidgetItem for Completion {
    fn get_column_name(idx: usize) -> &'static str {
        if idx == 0 {
            "function"
        } else {
            "N/A"
        }
    }

    fn get_min_column_width(idx: usize) -> u16 {
        if idx == 0 {
            15
        } else {
            error!("requested size of non-existent column");
            0
        }
    }

    fn len_columns() -> usize {
        1
    }

    fn get(&self, idx: usize) -> Option<Rc<String>> {
        if idx == 0 {
            Some(Rc::new(self.key.clone()))
        } else {
            error!("requested size of non-existent column");
            None
        }
    }
}

impl ListItemProvider<Completion> for CompletionsPromise {
    fn items(&self) -> Box<dyn Iterator<Item = &Completion> + '_> {
        match self.read() {
            None => Box::new(empty()),
            Some(vec) => Box::new(vec.into_iter()),
        }
    }
}
