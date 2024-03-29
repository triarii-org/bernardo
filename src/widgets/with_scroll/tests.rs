use std::rc::Rc;

use test_log::test;

use crate::*;

impl ListWidgetItem for Rc<String> {
    fn get_column_name(idx: usize) -> &'static str {
        match idx {
            0 => "name",
            _ => "",
        }
    }

    fn get_min_column_width(idx: usize) -> u16 {
        match idx {
            0 => 8,
            _ => 0,
        }
    }

    fn len_columns() -> usize {
        1
    }

    fn get(&self, idx: usize) -> Option<Rc<String>> {
        match idx {
            0 => Some(self.clone()),
            _ => None,
        }
    }
}

pub type ListWithScroll = WithScroll<ListWidget<Rc<String>>>;

pub struct WithScrollTestbed {
    pub widget: ListWithScroll,
    pub size: XY,
    pub theme: Theme,
    pub last_frame: Option<MetaOutputFrame>,
}

impl WithScrollTestbed {
    pub fn new() -> Self {
        let mut list_widget = ListWidget::<Rc<String>>::new();
        list_widget.set_fill_policy(SizePolicy::MATCH_LAYOUT);

        Self {
            widget: ListWithScroll::new(ScrollDirection::Vertical, list_widget).with_line_no(),
            size: XY::new(10, 20),
            theme: Default::default(),
            last_frame: None,
        }
    }

    pub fn next_frame(&mut self) {
        let (mut output, rcvr) = MockOutput::new(self.size, false, self.theme.clone());

        self.widget.prelayout();
        self.widget.layout(Screenspace::full_output(output.size()));
        self.widget.render(&self.theme, true, &mut output);

        output.end_frame().unwrap();

        let frame = rcvr.recv().unwrap();
        self.last_frame = Some(frame);
    }

    pub fn frame_op(&self) -> Option<&MetaOutputFrame> {
        self.last_frame.as_ref()
    }

    pub fn interpreter(&self) -> Option<WithScrollWidgetInterpreter<'_, ListWidget<Rc<String>>>> {
        self.frame_op().map(|frame| {
            let meta = frame
                .metadata
                .iter()
                .find(|item| item.typename == ListWithScroll::static_typename())
                .unwrap();

            WithScrollWidgetInterpreter::new(frame, meta)
        })
    }

    pub fn screenshot(&self) -> bool {
        self.frame_op().map(|frame| screenshot(&frame.buffer)).unwrap_or(false)
    }

    pub fn send_input(&mut self, input: InputEvent) {
        recursive_treat_views(&mut self.widget, input);
        self.next_frame();
    }
}

fn get_setup() -> WithScrollTestbed {
    let mut testbed = WithScrollTestbed::new();
    {
        let list = testbed.widget.internal_mut();

        let items: Vec<Rc<String>> = (1..51).map(|idx| Rc::new(format!("item{}", idx))).collect();

        list.set_provider(Box::new(items));

        list.set_highlighted(0);
    }

    testbed
}

#[test]
fn basic_with_scroll_testbed_test_page_down_and_page_up_works() {
    let mut setup = get_setup();
    setup.next_frame();

    assert!(setup.interpreter().is_some());

    assert_eq!(setup.frame_op().unwrap().buffer.get_line(19).unwrap().trim(), "20item19");
    setup.send_input(InputEvent::KeyInput(Keycode::PageDown.to_key()));

    assert_eq!(setup.frame_op().unwrap().buffer.get_line(19).unwrap().trim(), "21item20");

    setup.send_input(InputEvent::KeyInput(Keycode::PageDown.to_key()));
    assert_eq!(setup.frame_op().unwrap().buffer.get_line(19).unwrap().trim(), "41item40");

    setup.send_input(InputEvent::KeyInput(Keycode::PageDown.to_key()));
    assert_eq!(setup.frame_op().unwrap().buffer.get_line(19).unwrap().trim(), "50item49");
    assert_eq!(setup.frame_op().unwrap().buffer.get_line(0).unwrap().trim(), "31item30");

    setup.send_input(InputEvent::KeyInput(Keycode::PageUp.to_key()));
    assert_eq!(setup.frame_op().unwrap().buffer.get_line(0).unwrap().trim(), "30item29");

    setup.send_input(InputEvent::KeyInput(Keycode::PageUp.to_key()));
    assert_eq!(setup.frame_op().unwrap().buffer.get_line(0).unwrap().trim(), "10item9");

    setup.send_input(InputEvent::KeyInput(Keycode::PageUp.to_key()));
    assert_eq!(setup.frame_op().unwrap().buffer.get_line(0).unwrap().trim(), "1name");
}
