use crate::*;

const CANCEL_STRING: &'static str = "Cancel";
const OVERRIDE_STRING: &'static str = "Override";

pub fn override_dialog<T: Printable>(filename: T) -> GenericDialog {
    let mut text = "File \n\"".to_string();
    for grapheme in filename.graphemes() {
        text += grapheme;
    }

    text += "\"\n already exists.\n Do you wish to override?";

    GenericDialog::new(Box::new(text))
        .with_border(&SINGLE_BORDER_STYLE)
        .with_option(ButtonWidget::new(Box::new(CANCEL_STRING)).with_on_hit(|_| SaveFileDialogMsg::CancelOverride.someboxed()))
        .with_option(ButtonWidget::new(Box::new(OVERRIDE_STRING)).with_on_hit(|_| SaveFileDialogMsg::ConfirmOverride.someboxed()))
}
