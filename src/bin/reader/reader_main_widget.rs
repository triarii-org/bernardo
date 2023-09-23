use std::fmt::Debug;

use bernardo::config::theme::Theme;
use bernardo::io::buffer_output::buffer_output::BufferOutput;
use bernardo::io::input_event::InputEvent;
use bernardo::io::output::Output;
use bernardo::primitives::rect::Rect;
use bernardo::primitives::scroll::ScrollDirection;
use bernardo::primitives::xy::XY;
use bernardo::widget::any_msg::AnyMsg;
use bernardo::widget::widget::{get_new_widget_id, WID, Widget};
use bernardo::widgets::dump_visualizer_widget::DumpVisualizerWidget;
use bernardo::widgets::with_scroll::with_scroll::WithScroll;

pub struct ReaderMainWidget {
    wid: WID,
    main_display: WithScroll<DumpVisualizerWidget>,
}

impl ReaderMainWidget {
    pub const TYPENAME: &'static str = "reader_main_widget";

    pub fn new(dump: BufferOutput) -> Self {
        Self {
            wid: get_new_widget_id(),
            main_display: WithScroll::new(
                ScrollDirection::Both,
                DumpVisualizerWidget::new()
                    .with_dump(dump)
                ,
            ),
        }
    }
}

#[derive(Clone, Debug)]
enum ReaderMainWidgetMsg {
    Close,
}

impl AnyMsg for ReaderMainWidgetMsg {}

impl Widget for ReaderMainWidget {
    fn id(&self) -> WID {
        self.wid
    }

    fn static_typename() -> &'static str where Self: Sized {
        Self::TYPENAME
    }

    fn typename(&self) -> &'static str {
        Self::TYPENAME
    }

    fn full_size(&self) -> XY {
        XY::new(10, 10)
    }

    fn layout(&mut self, output_size: XY, visible_rect: Rect) {
        self.main_display.layout(output_size, visible_rect)
    }

    fn on_input(&self, input_event: InputEvent) -> Option<Box<dyn AnyMsg>> {
        None
    }

    fn update(&mut self, msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
        None
    }

    fn get_focused(&self) -> Option<&dyn Widget> {
        Some(&self.main_display)
    }

    fn get_focused_mut(&mut self) -> Option<&mut dyn Widget> {
        Some(&mut self.main_display)
    }

    fn render(&self, theme: &Theme, focused: bool, output: &mut dyn Output) {
        self.main_display.render(theme, focused, output)
    }
}