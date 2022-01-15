use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use log::{debug, error, warn};
use unicode_width::UnicodeWidthStr;

use crate::io::input_event::InputEvent;
use crate::io::keys::Keycode;
use crate::io::output::Output;
use crate::primitives::arrow::Arrow;
use crate::primitives::helpers;
use crate::primitives::size_constraint::SizeConstraint;
use crate::primitives::theme::Theme;
use crate::primitives::xy::{XY, ZERO};
use crate::widget::any_msg::AnyMsg;
use crate::widget::widget::{get_new_widget_id, WID, Widget, WidgetAction};
use crate::widgets::tree_view::tree_it::TreeIt;
use crate::widgets::tree_view::tree_view_node::{ChildRc, TreeViewNode};

pub struct TreeViewWidget<Key: Hash + Eq + Debug + Clone> {
    id: WID,
    filter: String,
    filter_enabled: bool,
    root_node: Rc<dyn TreeViewNode<Key>>,

    expanded: HashSet<Key>,
    highlighted: usize,

    //events
    on_miss: Option<WidgetAction<TreeViewWidget<Key>>>,
    on_highlighted_changed: Option<WidgetAction<TreeViewWidget<Key>>>,
    on_flip_expand: Option<WidgetAction<TreeViewWidget<Key>>>,
    // called on hitting "enter" over a selection.
    on_select_highlighted: Option<WidgetAction<TreeViewWidget<Key>>>
}

#[derive(Debug)]
enum TreeViewMsg {
    Arrow(Arrow),
    HitEnter,
}

impl AnyMsg for TreeViewMsg {}

/*
Warranties:
- (TODO double check) Highlighted always exists.
 */
impl<Key: Hash + Eq + Debug + Clone> TreeViewWidget<Key> {
    pub fn new(root_node: Rc<dyn TreeViewNode<Key>>) -> Self {
        Self {
            id: get_new_widget_id(),
            root_node,
            filter_enabled: false,
            filter: "".to_owned(),
            expanded: HashSet::new(),
            highlighted: 0,
            on_miss: None,
            on_highlighted_changed: None,
            on_flip_expand: None,
            on_select_highlighted: None,
        }
    }

    pub fn with_filter_enabled(self, enabled: bool) -> Self {
        TreeViewWidget {
            filter_enabled: enabled,
            ..self
        }
    }

    pub fn is_expanded(&self, key: &Key) -> bool {
        self.expanded.contains(key)
    }

    fn size_from_items(&self) -> XY {
        self.items().fold(ZERO, |old_size, item| {
            XY::new(
                // depth * 2 + 1 + label_length
                old_size
                    .x
                    .max(item.0 * 2 + 1 + item.1.label().width() as u16), // TODO fight overflow here.
                old_size.y + 1,
            )
        })
    }

    pub fn with_on_flip_expand(self, on_flip_expand: WidgetAction<TreeViewWidget<Key>>) -> Self {
        Self {
            on_flip_expand: Some(on_flip_expand),
            ..self
        }
    }

    pub fn with_on_highlighted_changed(self, on_highlighted_changed: WidgetAction<TreeViewWidget<Key>>) -> Self {
        Self {
            on_highlighted_changed: Some(on_highlighted_changed),
            ..self
        }
    }

    fn event_highlighted_changed(&self) -> Option<Box<dyn AnyMsg>> {
        self.on_highlighted_changed.map(|f| f(self)).flatten()
    }

    pub fn with_on_select_hightlighted(self, on_select_highlighted: WidgetAction<TreeViewWidget<Key>>) -> Self {
        Self {
            on_select_highlighted: Some(on_select_highlighted),
            ..self
        }
    }

    fn event_select_highlighted(&self) -> Option<Box<dyn AnyMsg>> {
        self.on_select_highlighted.map(|f| f(self)).flatten()
    }

    fn event_miss(&self) -> Option<Box<dyn AnyMsg>> {
        self.on_miss.map(|f| f(self)).flatten()
    }

    fn event_flip_expand(&self) -> Option<Box<dyn AnyMsg>> {
        self.on_flip_expand.map(|f| f(self)).flatten()
    }

    // returns new value
    fn flip_expanded(&mut self, id_to_flip: &Key) -> bool {
        if self.expanded.contains(id_to_flip) {
            self.expanded.remove(id_to_flip);
            false
        } else {
            self.expanded.insert(id_to_flip.clone());
            true
        }
    }

    pub fn items(&self) -> TreeIt<Key> {
        TreeIt::new(&self.root_node, &self.expanded)
    }

    pub fn get_highlighted(&self) -> (u16, ChildRc<Key>) {
        self.items().nth(self.highlighted).unwrap()
    }
}

impl<K: Hash + Eq + Debug + Clone> Widget for TreeViewWidget<K> {
    fn id(&self) -> WID {
        self.id
    }

    fn typename(&self) -> &'static str {
        "TreeView"
    }

    fn min_size(&self) -> XY {
        let mut from_items = self.size_from_items();

        if from_items.x < 5 {
            from_items.x = 5;
        };
        if from_items.y < 1 {
            from_items.y = 1;
        };

        from_items
    }

    fn layout(&mut self, sc: SizeConstraint) -> XY {
        let from_items = self.size_from_items();
        let mut res = sc.hint().size;

        if from_items.x > res.x && sc.x().is_none() {
            res.x = from_items.x;
        }

        if from_items.y > res.y && sc.y().is_none() {
            res.y = from_items.y;
        }

        res
    }

    fn on_input(&self, input_event: InputEvent) -> Option<Box<dyn AnyMsg>> {
        debug!("tree_view.on_input {:?}", input_event);

        return match input_event {
            InputEvent::KeyInput(key) => {
                match key.keycode {
                    Keycode::ArrowUp => Some(Box::new(TreeViewMsg::Arrow(Arrow::Up))),
                    Keycode::ArrowDown => Some(Box::new(TreeViewMsg::Arrow(Arrow::Down))),
                    Keycode::ArrowLeft => Some(Box::new(TreeViewMsg::Arrow(Arrow::Left))),
                    Keycode::ArrowRight => Some(Box::new(TreeViewMsg::Arrow(Arrow::Right))),
                    Keycode::Enter => { Some(Box::new(TreeViewMsg::HitEnter)) },
                    _ => None,
                }
            }
            _ => None,
        };
    }

    fn update(&mut self, msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
        let our_msg = msg.as_msg::<TreeViewMsg>();
        if our_msg.is_none() {
            warn!("expecetd TreeViewMsg, got {:?}", msg);
            return None;
        }

        return match our_msg.unwrap() {
            TreeViewMsg::Arrow(arrow) => match arrow {
                Arrow::Up => {
                    if self.highlighted > 0 {
                        self.highlighted -= 1;
                        self.event_highlighted_changed()
                    } else {
                        self.event_miss()
                    }
                }
                Arrow::Down => {
                    if self.highlighted < self.items().count() - 1 {
                        self.highlighted += 1;
                        self.event_highlighted_changed()
                    } else {
                        self.event_miss()
                    }
                }
                _ => None,
                // Arrow::Left => {}
                // Arrow::Right => {}
            },
            TreeViewMsg::HitEnter => {
                let node = {
                    let highlighted_pair = self.items().skip(self.highlighted).next();

                    if highlighted_pair.is_none() {
                        warn!(
                            "TreeViewWidget #{} highlighted non-existent node {}!",
                            self.id(),
                            self.highlighted
                        );
                        return None;
                    }
                    let (_, highlighted_node) = highlighted_pair.unwrap();
                    highlighted_node
                };

                if node.is_leaf() {
                    self.event_select_highlighted()
                } else {
                    self.flip_expanded(node.id());
                    self.event_flip_expand()
                }
            }
        };
    }

    fn render(&self, theme: &Theme, focused: bool, output: &mut dyn Output) {
        let primary_style = theme.default_text(focused);
        helpers::fill_output(primary_style.background, output);
        let cursor_style = theme.cursor().maybe_half(focused);

        for (idx, (depth, node)) in self.items().enumerate()
            // skipping lines that cannot be visible, because they are before hint()
            .skip(output.size_constraint().hint().upper_left().y as usize) {

            // skipping lines that cannot be visible, because larger than the hint()
            if idx >= output.size_constraint().hint().lower_right().y as usize {
                break;
            }

            match output.size_constraint().y() {
                Some(y) => if idx >= y as usize {
                    debug!("idx {}, output.size().y {}", idx, output.size_constraint());
                    break;
                }
                None => {}
            }


            let style = if idx == self.highlighted {
                cursor_style
            } else {
                primary_style
            };

            let prefix = if node.is_leaf() {
                " "
            } else {
                if self.expanded.contains(node.id()) {
                    "▶"
                } else {
                    "▼"
                }
            };

            let text = format!("{} {}", prefix, node.label());

            output.print_at(
                XY::new(depth * 2, idx as u16), // TODO idx in u16
                style,
                text.as_str(),
            );
        }
    }

    fn anchor(&self) -> XY {
        //TODO add x corresponding to depth
        XY::new(0, self.highlighted as u16) //TODO unsafe cast
    }
}
