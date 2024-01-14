use std::fmt::Debug;
use std::iter::empty;

use crate::*;

pub trait ListItemProvider<Item: ListWidgetItem>: Debug {
    fn items(&self) -> Box<dyn Iterator<Item = &Item> + '_>;
}

impl<Item: ListWidgetItem> ListItemProvider<Item> for () {
    fn items(&self) -> Box<dyn Iterator<Item = &Item> + '_> {
        Box::new(empty())
    }
}

impl<Item: ListWidgetItem> ListItemProvider<Item> for Vec<Item> {
    fn items(&self) -> Box<dyn Iterator<Item = &Item> + '_> {
        Box::new(self.iter())
    }
}
