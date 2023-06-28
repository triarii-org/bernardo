use crate::experiments::screen_shot::screenshot;
use crate::mocks::mock_labels_provider::MockLabelsProvider;
use crate::primitives::printable::Printable;
use crate::widgets::editor_widget::label::label::{Label, LabelPos, LabelStyle};
use crate::widgets::editor_widget::label::labels_provider::LabelsProvider;
use crate::widgets::tests::editor_view_testbed::EditorViewTestbed;
use crate::widgets::tests::widget_testbed_builder::WidgetTestbedBuilder;

fn get_setup() -> EditorViewTestbed {
    let mut mock_labels_provider = MockLabelsProvider::new();

    mock_labels_provider.labels.push(
        Label::new(
            LabelPos::Inline { char_idx: 49 },
            LabelStyle::TypeAnnotation,
            Box::new(
                ":PathBuf".to_string()
            )));

    let mut editor_view_testbed = WidgetTestbedBuilder::new()
        .with_label_provider(
            mock_labels_provider.into_ref()
        )
        .build_editor();


    {
        let some_text = r#"use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./src");

    // some comment to avoid formatting collapse
}"#;
        let mut buffer_lock = editor_view_testbed.editor_view.get_buffer_ref().lock_rw().unwrap();
        buffer_lock.set_text(some_text);
    }

    editor_view_testbed
}

#[test]
fn editor_label() {
    let mut setup = get_setup();
    setup.next_frame();

    assert!(setup.interpreter().unwrap().is_editor_focused());

    let interpreter = setup.interpreter().unwrap();

    let first_type = interpreter.get_type_annotations().next().unwrap();
    assert_eq!(first_type.y, 3);
    assert_eq!(first_type.contents.text, ":PathBuf");

    assert_eq!(interpreter.get_line_by_y(3).unwrap().text.trim(), "let path:PathBuf = PathBuf::from(\"./src\");⏎");
}