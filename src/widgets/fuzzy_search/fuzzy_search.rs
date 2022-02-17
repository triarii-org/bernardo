use std::fmt::Debug;
use std::hash::Hash;

use unicode_segmentation::UnicodeSegmentation;

use crate::{AnyMsg, InputEvent, Output, SizeConstraint, Theme, Widget, ZERO};
use crate::io::sub_output::SubOutput;
use crate::layout::layout::WidgetIdRect;
use crate::layout::leaf_layout::LeafLayout;
use crate::layout::split_layout::{SplitDirection, SplitLayout, SplitRule};
use crate::primitives::rect::Rect;
use crate::primitives::xy::XY;
use crate::widget::widget::{get_new_widget_id, WID};
use crate::widgets::edit_box::EditBoxWidget;
use crate::widgets::fuzzy_search::item_provider::{Item, ItemsProvider};

pub struct FuzzySearchWidget {
    id: WID,
    edit: EditBoxWidget,
    providers: Vec<Box<dyn ItemsProvider>>,
    context_shortcuts: Vec<String>,
}

impl FuzzySearchWidget {
    pub fn new() -> Self {
        let edit = EditBoxWidget::new();

        Self {
            id: get_new_widget_id(),
            edit,
            providers: vec![],
            context_shortcuts: vec![],
        }
    }

    pub fn with_provider(self, provider: Box<dyn ItemsProvider>) -> Self {
        let mut contexts = self.providers;
        contexts.push(provider);

        let mut context_shortcuts: Vec<String> = vec![];
        context_shortcuts.resize(contexts.len(), String::default());

        // TODO add common prefixes bla bla bla
        for (idx, c) in contexts.iter().enumerate() {
            context_shortcuts[idx] += c.context_name().graphemes(true).next().unwrap_or("");
        }

        Self {
            providers: contexts,
            context_shortcuts,
            ..self
        }
    }

    pub fn shortened_contexts(&self) -> &Vec<String> {
        &self.context_shortcuts
    }

    fn width(&self) -> u16 {
        16
    }

    fn items(&self) -> ItemIter {
        ItemIter {
            providers: &self.providers,
            context_shortcuts: &self.shortened_contexts(),
            query: &self.edit.get_text(),
            pos: 0,
            cur_iter: None,
        }
    }
}

struct ItemIter<'a> {
    providers: &'a Vec<Box<dyn ItemsProvider>>,
    context_shortcuts: &'a Vec<String>,
    query: &'a str,
    pos: usize,
    cur_iter: Option<Box<dyn Iterator<Item=Item<'a>>>>,
}

impl<'a> Iterator for ItemIter<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.providers.len() {
            if self.cur_iter.is_none() {
                self.cur_iter = Some(self.providers[self.pos].items(self.query));
            }

            let mut iter = self.cur_iter.as_mut().unwrap();

            match iter.next() {
                Some(item) => { return Some(item); }
                None => {
                    self.cur_iter = None;
                    self.pos += 1;
                }
            }
        }

        None
    }
}


impl Widget for FuzzySearchWidget {
    fn id(&self) -> WID {
        self.id
    }

    fn typename(&self) -> &'static str {
        "fuzzy_search"
    }

    fn min_size(&self) -> XY {
        // Completely arbitrary
        XY::new(16, 5)
    }

    fn layout(&mut self, sc: SizeConstraint) -> XY {
        let items_len = self.items().count() + 1;

        //TODO
        XY::new(16, items_len as u16)
    }

    fn on_input(&self, input_event: InputEvent) -> Option<Box<dyn AnyMsg>> {
        None
    }

    fn update(&mut self, msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
        None
    }

    fn render(&self, theme: &Theme, focused: bool, output: &mut dyn Output) {
        let mut suboutput = SubOutput::new(Box::new(output),
                                           Rect::new(ZERO, XY::new(self.width(), 1)));
    }
}