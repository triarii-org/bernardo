use crate::*;
use crate::*;
use crate::widgets::main_view::main_view::DocumentIdentifier;

use crate::*;

#[test]
fn fuzz_1() {
    let mut bf = BufferState::full(None, DocumentIdentifier::new_unique());

    bf.apply_cem(CommonEditMsg::Char('䄀'), get_new_widget_id(), 10, None);
}
