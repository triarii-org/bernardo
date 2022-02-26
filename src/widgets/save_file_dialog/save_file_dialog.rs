/*
this widget is supposed to offer:
- tree view on the right, along with scrolling,
- file list view on the most of display (with scrolling as well)
- filename edit box
- buttons save and cancel

I hope I will discover most of functional constraints while implementing it.
 */

use std::borrow::Borrow;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use log::{debug, warn};

use crate::experiments::focus_group::{FocusGroup, FocusUpdate};
use crate::io::filesystem_tree::file_front::FileFront;
use crate::io::filesystem_tree::filesystem_front::FilesystemFront;
use crate::io::input_event::InputEvent;
use crate::io::output::Output;
use crate::io::sub_output::SubOutput;
use crate::layout::cached_sizes::DisplayState;
use crate::layout::layout::{Layout, WidgetIdRect};
use crate::layout::leaf_layout::LeafLayout;
use crate::layout::split_layout::{SplitDirection, SplitLayout, SplitRule};
use crate::primitives::scroll::ScrollDirection;
use crate::primitives::size_constraint::SizeConstraint;
use crate::primitives::theme::Theme;
use crate::primitives::xy::XY;
use crate::widget::any_msg::AnyMsg;
use crate::widget::widget::{get_new_widget_id, WID, Widget};
use crate::widgets::button::ButtonWidget;
use crate::widgets::edit_box::EditBoxWidget;
use crate::widgets::list_widget::ListWidget;
use crate::widgets::tree_view::tree_view::TreeViewWidget;
use crate::widgets::tree_view::tree_view_node::TreeViewNode;
use crate::widgets::with_scroll::WithScroll;

// TODO now it displays both files and directories in tree view, it should only directories

pub struct SaveFileDialogWidget {
    id: WID,

    display_state: Option<DisplayState>,

    tree_widget: WithScroll<TreeViewWidget<PathBuf, Rc<FileFront>>>,
    list_widget: ListWidget<Rc<FileFront>>,
    edit_box: EditBoxWidget,

    ok_button: ButtonWidget,
    cancel_button: ButtonWidget,

    curr_display_path: PathBuf,

    // TODO this will probably get moved
    filesystem_provider: Box<dyn FilesystemFront>,
}

#[derive(Clone, Debug)]
pub enum SaveFileDialogMsg {
    FocusUpdateMsg(FocusUpdate),
    // Sent when a left hand-side file-tree subtree is expanded (default: on Enter key)
    TreeExpanded(Rc<FileFront>),
    // Sent when a left hand-side file-tree subtree selection changed
    TreeHighlighted(Rc<FileFront>),
}

impl AnyMsg for SaveFileDialogMsg {}

impl SaveFileDialogWidget {
    pub fn new(filesystem_provider: Box<dyn FilesystemFront>) -> Self {
        let tree = filesystem_provider.get_root();
        let tree_widget = TreeViewWidget::<PathBuf, Rc<FileFront>>::new(tree)
            .with_on_flip_expand(|widget| {
                let (_, item) = widget.get_highlighted();
                Some(Box::new(SaveFileDialogMsg::TreeExpanded(item)))
            })
            .with_on_highlighted_changed(|widget| {
                let (_, item) = widget.get_highlighted();
                Some(Box::new(SaveFileDialogMsg::TreeHighlighted(item)))
            });

        let scroll_tree_widget = WithScroll::new(tree_widget, ScrollDirection::Vertical);

        let list_widget = ListWidget::new().with_selection();
        let edit_box = EditBoxWidget::new().with_enabled(true);

        let ok_button = ButtonWidget::new("OK".to_owned());
        let cancel_button = ButtonWidget::new("Cancel".to_owned());

        SaveFileDialogWidget {
            id: get_new_widget_id(),
            display_state: None,
            tree_widget: scroll_tree_widget,
            list_widget,
            edit_box,
            ok_button,
            cancel_button,
            curr_display_path: filesystem_provider.get_root().id().clone(),
            filesystem_provider,
        }
    }

    fn internal_layout(&mut self, max_size: XY) -> Vec<WidgetIdRect> {
        let tree_widget = &mut self.tree_widget;
        let list_widget = &mut self.list_widget;
        let edit_box = &mut self.edit_box;

        let mut left_column = LeafLayout::new(tree_widget);

        let mut list = LeafLayout::new(list_widget);
        let mut edit = LeafLayout::new(edit_box);
        let mut right_column = SplitLayout::new(SplitDirection::Vertical)
            .with(SplitRule::Proportional(1.0),
                  &mut list)
            .with(SplitRule::Fixed(1),
                  &mut edit,
            );

        let mut layout = SplitLayout::new(SplitDirection::Horizontal)
            .with(SplitRule::Proportional(1.0),
                  &mut left_column)
            .with(SplitRule::Proportional(4.0),
                  &mut right_column,
            );

        let res = layout.calc_sizes(max_size);

        res
    }

    fn todo_filesystem_updated(&mut self) {}
}

impl Widget for SaveFileDialogWidget {
    fn id(&self) -> WID {
        self.id
    }

    fn typename(&self) -> &'static str {
        "SaveFileDialog"
    }

    fn min_size(&self) -> XY {
        XY::new(4, 4)
    }

    fn layout(&mut self, sc: SizeConstraint) -> XY {
        // TODO this entire function is a makeshift and experiment
        let max_size = sc.hint().size;

        // TODO this lazy relayouting kills resizing on data change.
        // if self.display_state.as_ref().map(|x| x.for_size == max_size) == Some(true) {
        //     return max_size
        // }

        // TODO relayouting destroys focus selection.

        let res_sizes = self.internal_layout(max_size);

        debug!("size {}, res_sizes {:?}", max_size, res_sizes);

        // Retention of focus. Not sure if it should be here.
        let focus_op = self.display_state.as_ref().map(|ds| ds.focus_group.get_focused());

        self.display_state = Some(DisplayState::new(max_size, res_sizes));

        // re-setting focus.
        match (focus_op, &mut self.display_state) {
            (Some(focus), Some(ds)) => { ds.focus_group.set_focused(focus); },
            _ => {}
        };

        max_size
    }

    fn on_input(&self, input_event: InputEvent) -> Option<Box<dyn AnyMsg>> {
        debug!("save_file_dialog.on_input {:?}", input_event);

        return match input_event {
            InputEvent::KeyInput(key) => match key {
                key if key.keycode.is_arrow() && key.modifiers.ALT => {
                    debug!("arrow {:?}", key);
                    match key.keycode.as_focus_update() {
                        None => {
                            warn!("failed expected cast to FocusUpdate of {:?}", key);
                            None
                        }
                        Some(event) => {
                            let msg = SaveFileDialogMsg::FocusUpdateMsg(event);
                            debug!("sending {:?}", msg);
                            Some(Box::new(msg))
                        }
                    }
                }
                unknown_key => {
                    debug!("unknown_key {:?}", unknown_key);
                    None
                }
            }
            InputEvent::Tick => None
        }
    }

    fn update(&mut self, msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
        debug!("save_file_dialog.update {:?}", msg);

        let our_msg = msg.as_msg::<SaveFileDialogMsg>();
        if our_msg.is_none() {
            warn!("expecetd SaveFileDialogMsg, got {:?}", msg);
            return None;
        }

        return match our_msg.unwrap() {
            SaveFileDialogMsg::FocusUpdateMsg(focus_update) => {
                warn!("updating focus");
                let fc = *focus_update;
                let ds: &mut DisplayState = self.display_state.as_mut().unwrap();
                let fg = &mut ds.focus_group;
                let msg = fg.update_focus(fc);
                warn!("focus updated {}", msg);
                None
            }
            SaveFileDialogMsg::TreeExpanded(node) => {
                // TODO load data if necessary

                None
            }
            SaveFileDialogMsg::TreeHighlighted(node) => {
                self.list_widget.set_items(node);
                None
            }
            unknown_msg => {
                warn!("SaveFileDialog.update : unknown message {:?}", unknown_msg);
                None
            }
        };
    }

    fn get_focused(&self) -> Option<&dyn Widget> {
        let wid_op = self.display_state.as_ref().map(|ds| ds.focus_group.get_focused());
        wid_op.map(|wid| self.get_subwidget(wid)).flatten()
    }

    fn get_focused_mut(&mut self) -> Option<&mut dyn Widget> {
        let wid_op = self.display_state.as_ref().map(|ds| ds.focus_group.get_focused());
        wid_op.map(move |wid| self.get_subwidget_mut(wid)).flatten()
    }

    fn render(&self, theme: &Theme, _focused: bool, output: &mut dyn Output) {
        match self.display_state.borrow().as_ref() {
            None => warn!("failed rendering save_file_dialog without cached_sizes"),
            Some(cached_sizes) => {
                debug!("widget_sizes : {:?}", cached_sizes.widget_sizes);
                for wir in &cached_sizes.widget_sizes {
                    match self.get_subwidget(wir.wid) {
                        Some(widget) => {
                            let sub_output = &mut SubOutput::new(output, wir.rect);
                            widget.render(theme,
                                          cached_sizes.focus_group.get_focused() == widget.id(),
                                          sub_output,
                            );
                        },
                        None => {
                            warn!("subwidget {} not found!", wir.wid);
                        }
                    }
                }
            }
        }
    }

    fn subwidgets_mut(&mut self) -> Box<dyn std::iter::Iterator<Item=&mut dyn Widget> + '_> {
        debug!("call to save_file_dialog subwidget_mut on {}", self.id());
        Box::new(vec![&mut self.tree_widget as &mut dyn Widget, &mut self.list_widget, &mut self.edit_box].into_iter())
    }

    fn subwidgets(&self) -> Box<dyn std::iter::Iterator<Item=&dyn Widget> + '_> {
        debug!("call to save_file_dialog subwidget on {}", self.id());
        Box::new(vec![&self.tree_widget as &dyn Widget, &self.list_widget, &self.edit_box].into_iter())
    }
}