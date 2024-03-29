use std::rc::Rc;

use crate::*;

const ALPHABET: [&str; 26] = [
    "alfa", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india", "juliet", "kilo", "lima", "mike", "november",
    "oscar", "papa", "quebec", "romeo", "sierra", "tango", "uniform", "victor", "whiskey", "x-ray", "yankee", "zulu",
];

pub struct MockItemProvider {
    num_items: usize,

    items: Vec<String>,
}

impl MockItemProvider {
    pub fn new(num_items: usize) -> Self {
        let mut items: Vec<String> = vec![];

        for i in 0..num_items {
            let mut idx = i;
            let mut item = String::default();

            loop {
                if !item.is_empty() {
                    item += " ";
                }

                item += ALPHABET[idx % ALPHABET.len()];
                idx /= ALPHABET.len();
                if idx == 0 {
                    break;
                }
            }

            items.push(item);
        }

        MockItemProvider { num_items, items }
    }
}

impl Item for String {
    fn display_name(&self) -> Rc<String> {
        Rc::new(self.to_owned())
    }

    fn on_hit(&self) -> Box<dyn AnyMsg> {
        Box::new(MainViewMsg::CloseHover)
    }
}

impl ItemsProvider for MockItemProvider {
    fn context_name(&self) -> Rc<String> {
        Rc::new("mock".to_string())
    }

    fn items(&self, query: String, limit: usize) -> Box<dyn Iterator<Item = Box<dyn Item + '_>> + '_> {
        Box::new(
            self.items
                .iter()
                .filter(move |t| is_subsequence(t, &query))
                .take(limit)
                .map(|f| Box::new(f.to_owned()) as Box<dyn Item>),
        )
    }
}
