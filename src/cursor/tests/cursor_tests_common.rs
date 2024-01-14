use std::collections::HashSet;

use ropey::Rope;

use crate::*;
use crate::*;

// these are the tests of testing framework. It's complicated.

#[test]
fn test_text_to_buffer_cursors_1() {
    let (text, cursors) = common_text_to_buffer_cursors_with_selections("te[xt)");
    assert_eq!(text, "text");
    assert_eq!(cursors.set().len(), 1);
    assert_eq!(cursors.set()[0].a, 2);
    assert_eq!(cursors.set()[0].s, Some(Selection::new(2, 4)));
}

#[test]
fn test_common_text_to_buffer_cursors_2() {
    let (text, cursors) = common_text_to_buffer_cursors_with_selections("te(xt]");
    assert_eq!(text, "text");
    assert_eq!(cursors.set().len(), 1);
    assert_eq!(cursors.set()[0].a, 4);
    assert_eq!(cursors.set()[0].s, Some(Selection::new(2, 4)));
}

#[test]
fn test_common_text_to_buffer_cursors_3() {
    let (text, cursors) = common_text_to_buffer_cursors_with_selections("(t]e(xt]");
    assert_eq!(text, "text");
    assert_eq!(cursors.set().len(), 2);
    assert_eq!(cursors.set()[0].a, 1);
    assert_eq!(cursors.set()[0].s, Some(Selection::new(0, 1)));
    assert_eq!(cursors.set()[1].a, 4);
    assert_eq!(cursors.set()[1].s, Some(Selection::new(2, 4)));
}

#[test]
fn test_common_text_to_buffer_cursors_4() {
    let (text, cursors) = common_text_to_buffer_cursors_with_selections("(te](xt]");
    assert_eq!(text, "text");
    assert_eq!(cursors.set().len(), 2);
    assert_eq!(cursors.set()[0].a, 2);
    assert_eq!(cursors.set()[0].s, Some(Selection::new(0, 2)));
    assert_eq!(cursors.set()[1].a, 4);
    assert_eq!(cursors.set()[1].s, Some(Selection::new(2, 4)));
}

#[test]
fn test_common_text_to_buffer_cursors_5() {
    let (text, cursors) = common_text_to_buffer_cursors_with_selections("text#");
    assert_eq!(text, "text");
    assert_eq!(cursors.set().len(), 1);
    assert_eq!(cursors.set()[0].a, 4);
    assert_eq!(cursors.set()[0].s, None);
}

#[test]
fn test_buffer_cursors_sel_to_text_0() {
    let text = common_buffer_cursors_sel_to_text(&Rope::from("text"), &CursorSet::new(vec![]));

    assert_eq!(text, "text");
}

#[test]
fn test_buffer_cursors_sel_to_text_1() {
    let text = common_buffer_cursors_sel_to_text(
        &Rope::from("text"),
        &CursorSet::new(vec![Cursor::new(0).with_selection(Selection::new(0, 2))]),
    );

    assert_eq!(text, "[te)xt");
}

#[test]
fn test_buffer_cursors_sel_to_text_2() {
    let text = common_buffer_cursors_sel_to_text(
        &Rope::from("text"),
        &CursorSet::new(vec![
            Cursor::new(0).with_selection(Selection::new(0, 2)),
            Cursor::new(2).with_selection(Selection::new(2, 4)),
        ]),
    );

    assert_eq!(text, "[te)[xt)");
}

#[test]
fn test_buffer_cursors_sel_to_text_3() {
    let text = common_buffer_cursors_sel_to_text(&Rope::from("text\n"), &CursorSet::new(vec![Cursor::new(5)]));

    assert_eq!(text, "text\n#");
}

#[test]
fn apply_sel_works() {
    let f: fn(&mut CursorSet, &dyn TextBuffer) = |_c: &mut CursorSet, _b: &dyn TextBuffer| {};

    assert_eq!(common_apply("text", f), "text");
    assert_eq!(common_apply("te[xt)", f), "te[xt)");
    assert_eq!(common_apply("[t)(ext]", f), "[t)(ext]");
}
