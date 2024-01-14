use crate::primitives::xy::XY;

use crate::*;

pub type LayoutMaker = Box<dyn Fn(&MockComplexWidget) -> Box<dyn Layout<MockComplexWidget>>>;

pub struct MockComplexWidget {
    id: WID,
    layout_maker: LayoutMaker,
    subwidgets: Vec<MockWidget>,
    size: XY,
}

impl MockComplexWidget {
    pub fn new(size: XY, subwidgets: Vec<MockWidget>, layout_maker: LayoutMaker) -> Self {
        MockComplexWidget {
            id: get_new_widget_id(),
            layout_maker,
            size,
            subwidgets,
        }
    }

    pub fn get_subwidget_ptr(&self, idx: usize) -> SubwidgetPointer<Self> {
        debug_assert!(idx < self.subwidgets.len());
        SubwidgetPointer::new(
            Box::new(move |mcw: &MockComplexWidget| &mcw.subwidgets[idx]),
            Box::new(move |mcw: &mut MockComplexWidget| &mut mcw.subwidgets[idx]),
        )
    }

    /*
    this is supposed to be identical to layout below
     */
    pub fn get_layout_res(&mut self, screenspace: Screenspace) -> LayoutResult<Self> {
        let layout = (self.layout_maker)(self);
        layout.layout(self, screenspace)
    }
}

impl Widget for MockComplexWidget {
    fn id(&self) -> WID {
        self.id
    }

    fn static_typename() -> &'static str
    where
        Self: Sized,
    {
        "MockComplexWidget"
    }

    fn typename(&self) -> &'static str {
        "MockCompexWidget"
    }

    fn full_size(&self) -> XY {
        self.size
    }

    fn layout(&mut self, screenspace: Screenspace) {
        self.get_layout_res(screenspace);
    }

    fn on_input(&self, input_event: InputEvent) -> Option<Box<dyn AnyMsg>> {
        todo!()
    }

    fn update(&mut self, msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
        todo!()
    }

    fn render(&self, theme: &Theme, focused: bool, output: &mut dyn Output) {
        todo!()
    }
}
