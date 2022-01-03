use log::warn;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::io::input_event::InputEvent;
use crate::io::input_event::InputEvent::KeyInput;
use crate::io::keys::Keycode;
use crate::io::output::Output;
use crate::primitives::helpers;
use crate::primitives::size_constraint::SizeConstraint;
use crate::primitives::theme::Theme;
use crate::primitives::xy::{XY, ZERO};
use crate::widget::any_msg::AnyMsg;
use crate::widget::widget::{get_new_widget_id, WID, Widget, WidgetAction};

const MIN_WIDTH: u16 = 12;
const MAX_WIDTH: u16 = 80; //completely arbitrary

struct EditBoxDisplayState {
    width: u16,
}

pub struct EditBoxWidget {
    id: WID,
    enabled: bool,
    // hit is basically pressing enter.
    on_hit: Option<WidgetAction<EditBoxWidget>>,
    on_change: Option<WidgetAction<EditBoxWidget>>,
    // miss is trying to make illegal move. Like backspace on empty, left on leftmost etc.
    on_miss: Option<WidgetAction<EditBoxWidget>>,
    text: String,
    cursor: usize,

    max_width_op: Option<u16>,

    //display state
    display_state: EditBoxDisplayState,
}

impl EditBoxWidget {
    pub fn new() -> Self {
        EditBoxWidget {
            id: get_new_widget_id(),
            cursor: 0,
            enabled: true,
            text: "".into(),
            on_hit: None,
            on_change: None,
            on_miss: None,
            max_width_op: None,
            display_state: EditBoxDisplayState {
                width: MIN_WIDTH
            },
        }
    }

    pub fn with_max_width(self, max_width: u16) -> Self {
        EditBoxWidget {
            max_width_op: Some(max_width),
            ..self
        }
    }

    pub fn with_on_hit(self, on_hit: WidgetAction<EditBoxWidget>) -> Self {
        EditBoxWidget {
            on_hit: Some(on_hit),
            ..self
        }
    }

    pub fn with_on_change(self, on_change: WidgetAction<EditBoxWidget>) -> Self {
        EditBoxWidget {
            on_change: Some(on_change),
            ..self
        }
    }

    pub fn with_on_miss(self, on_miss: WidgetAction<EditBoxWidget>) -> Self {
        EditBoxWidget {
            on_miss: Some(on_miss),
            ..self
        }
    }

    pub fn with_enabled(self, enabled: bool) -> Self {
        EditBoxWidget { enabled, ..self }
    }

    pub fn with_text(self, text: String) -> Self {
        EditBoxWidget { text, ..self }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    fn event_changed(&self) -> Option<Box<dyn AnyMsg>> {
        if self.on_change.is_some() {
            self.on_change.unwrap()(self)
        } else {
            None
        }
    }

    fn event_miss(&self) -> Option<Box<dyn AnyMsg>> {
        if self.on_miss.is_some() {
            self.on_miss.unwrap()(self)
        } else {
            None
        }
    }

    fn event_hit(&self) -> Option<Box<dyn AnyMsg>> {
        if self.on_hit.is_some() {
            self.on_hit.unwrap()(self)
        } else {
            None
        }
    }
}


impl Widget for EditBoxWidget {
    fn id(&self) -> WID {
        self.id
    }

    fn typename(&self) -> &'static str {
        "EditBox"
    }

    fn min_size(&self) -> XY {
        XY::new(MIN_WIDTH, 1)
    }

    fn layout(&mut self, sc: SizeConstraint) -> XY {
        debug_assert!(sc.bigger_equal_than(self.min_size()));

        let x = sc.x().unwrap_or(self.max_width_op.unwrap_or(MAX_WIDTH));

        XY::new(x, 1)
    }

    fn on_input(&self, input_event: InputEvent) -> Option<Box<dyn AnyMsg>> {
        debug_assert!(
            self.enabled,
            "EditBoxWidgetMsg: received input to disabled component!"
        );

        return match input_event {
            KeyInput(key_event) if key_event.no_modifiers() => match key_event.keycode {
                Keycode::Enter => Some(Box::new(EditBoxWidgetMsg::Hit)),
                Keycode::Char(ch) => Some(Box::new(EditBoxWidgetMsg::Letter(ch))),
                Keycode::Backspace => Some(Box::new(EditBoxWidgetMsg::Backspace)),
                Keycode::ArrowLeft => Some(Box::new(EditBoxWidgetMsg::ArrowLeft)),
                Keycode::ArrowRight => Some(Box::new(EditBoxWidgetMsg::ArrowRight)),
                _ => None
            }
            _ => None,
        };
    }

    fn update(&mut self, msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
        let our_msg = msg.as_msg::<EditBoxWidgetMsg>();
        if our_msg.is_none() {
            warn!("expecetd EditBoxWidgetMsg, got {:?}", msg);
            return None;
        }

        return match our_msg.unwrap() {
            EditBoxWidgetMsg::Hit => self.event_hit(),
            EditBoxWidgetMsg::Letter(ch) => {
                let mut new_text = self
                    .text
                    .graphemes(true)
                    .take(self.cursor)
                    .fold("".to_owned(), |a, b| a + b);

                new_text += ch.to_string().as_str(); //TODO: make this conversion better?

                new_text += self
                    .text
                    .graphemes(true)
                    .skip(self.cursor)
                    .fold("".to_owned(), |a, b| a + b)
                    .as_str();

                self.text = new_text;
                self.cursor += 1;

                self.event_changed()
            }
            EditBoxWidgetMsg::Backspace => {
                if self.cursor == 0 {
                    self.event_miss()
                } else {
                    self.cursor -= 1;
                    let mut new_text = self
                        .text
                        .graphemes(true)
                        .take(self.cursor)
                        .fold("".to_owned(), |a, b| a + b);
                    new_text += self
                        .text
                        .graphemes(true)
                        .skip(self.cursor + 1)
                        .fold("".to_owned(), |a, b| a + b)
                        .as_str();
                    self.text = new_text;
                    self.event_changed()
                }
            }
            EditBoxWidgetMsg::ArrowLeft => {
                if self.cursor == 0 {
                    self.event_miss()
                } else {
                    self.cursor -= 1;
                    None
                }
            }
            EditBoxWidgetMsg::ArrowRight => {
                if self.cursor >= self.text.len() {
                    self.event_miss()
                } else {
                    self.cursor += 1;
                    None
                }
            }
            _ => None,
        };
    }

    fn render(&self, theme: &Theme, focused: bool, output: &mut dyn Output) {
        let primary_style = theme.editable_field().maybe_half(focused);

        helpers::fill_background(primary_style.background, output);
        let cursor_style = theme.cursor().maybe_half(focused);

        let before_cursor = self
            .text
            .graphemes(true)
            // .enumerate()
            .take(self.cursor)
            .map(|g| g.into())
            .fold("".to_string(), |a, b| a + b);

        let cursor_pos = self
            .text
            .graphemes(true)
            .take(self.cursor)
            .map(|g| g.width())
            .fold(0, |a, b| a + b);

        let at_cursor = self
            .text
            .graphemes(true)
            .skip(self.cursor)
            .next()
            .unwrap_or(" ");

        let after_cursor = self
            .text
            .graphemes(true)
            .skip(self.cursor + 1)
            .map(|g| g.into())
            .fold("".to_string(), |a, b| a + b);

        output.print_at((0, 0).into(), primary_style, before_cursor.as_str());
        output.print_at((cursor_pos, 0).into(), cursor_style, at_cursor);
        if after_cursor.len() > 0 {
            output.print_at(
                (cursor_pos + 1, 0).into(),
                primary_style,
                after_cursor.as_str(),
            );
        }

        // if cursor is after the text, we need to add an offset, so the background does not
        // overwrite cursor style.
        let cursor_offset: u16 = if cursor_pos == self.text.len() { 1 } else { 0 };

        // background after the text
        if self.display_state.width as usize > self.text.width() {
            let background_length = self.display_state.width - (cursor_offset + self.text.width() as u16);
            let begin_pos: XY = XY::new(cursor_offset + self.text.width() as u16, 0);
            for i in 0..background_length {
                let pos = begin_pos + XY::new(i, 0);
                output.print_at(
                    pos,
                    primary_style,
                    " ",
                );
            }
        }
    }

    fn anchor(&self) -> XY {
        ZERO
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EditBoxWidgetMsg {
    Hit,
    Letter(char),
    Backspace,
    ArrowLeft,
    ArrowRight,
}

impl AnyMsg for EditBoxWidgetMsg {}