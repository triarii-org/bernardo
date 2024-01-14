use std::fmt::Debug;

use bernardo::*;

pub struct ReaderMainWidget {
    wid: WID,
    main_display: WithScroll<DumpVisualizerWidget>,
}

impl ReaderMainWidget {
    pub const TYPENAME: &'static str = "reader_main_widget";

    pub fn new(dump: BufferOutput) -> Self {
        Self {
            wid: get_new_widget_id(),
            main_display: WithScroll::new(ScrollDirection::Both, DumpVisualizerWidget::new().with_dump(dump)),
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

    fn static_typename() -> &'static str
    where
        Self: Sized,
    {
        Self::TYPENAME
    }

    fn typename(&self) -> &'static str {
        Self::TYPENAME
    }

    fn full_size(&self) -> XY {
        XY::new(10, 10)
    }

    fn layout(&mut self, screenspace: Screenspace) {
        self.main_display.layout(screenspace)
    }

    fn on_input(&self, _input_event: InputEvent) -> Option<Box<dyn AnyMsg>> {
        None
    }

    fn update(&mut self, _msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
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
