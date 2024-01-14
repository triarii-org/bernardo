use crate::*;
use crate::*;
use crate::*;
use crate::*;
use crate::*;
use crate::*;

use crate::*;

pub struct SaveFileInterpreter<'a> {
    meta: &'a Metadata,
    output: &'a MetaOutputFrame,

    tree_view: TreeViewInterpreter<'a>,
    list_view: ListViewInterpreter<'a>,
    edit_widget: EditWidgetInterpreter<'a>,

    ok_button: ButtonWidgetInterpreter<'a>,
    cancel_button: ButtonWidgetInterpreter<'a>,
}

impl<'a> SaveFileInterpreter<'a> {
    pub fn new(meta: &'a Metadata, output: &'a MetaOutputFrame) -> Self {
        let tree_view_meta: Vec<&Metadata> = output
            .get_meta_by_type(TREE_VIEW_TYPENAME)
            .filter(|c| meta.rect.contains_rect(c.rect))
            .collect();

        debug_assert!(tree_view_meta.len() == 1);
        let tree_view = TreeViewInterpreter::new(tree_view_meta[0], output);

        let list_view_meta: Vec<&Metadata> = output
            .get_meta_by_type(LIST_TYPENAME)
            .filter(|c| meta.rect.contains_rect(c.rect))
            .collect();

        debug_assert!(list_view_meta.len() == 1);
        let list_view = ListViewInterpreter::new(list_view_meta[0], output);

        let editor_widget_meta: Vec<&Metadata> = output
            .get_meta_by_type(EditBoxWidget::TYPENAME)
            .filter(|c| meta.rect.contains_rect(c.rect))
            .collect();

        debug_assert!(editor_widget_meta.len() == 1);
        let edit_widget = EditWidgetInterpreter::new(editor_widget_meta[0], output);

        let button_metas: Vec<&Metadata> = output
            .get_meta_by_type(ButtonWidget::TYPENAME)
            .filter(|c| meta.rect.contains_rect(c.rect))
            .collect();

        debug_assert!(button_metas.len() == 2);
        let buttons: Vec<ButtonWidgetInterpreter> = button_metas.into_iter().map(|c| ButtonWidgetInterpreter::new(c, output)).collect();

        let ok_button = buttons
            .iter()
            .find(|b| b.contents().contains(SaveFileDialogWidget::OK_LABEL))
            .unwrap()
            .clone();
        let cancel_button = buttons
            .iter()
            .find(|b| b.contents().contains(SaveFileDialogWidget::CANCEL_LABEL))
            .unwrap()
            .clone();

        Self {
            meta,
            output,
            tree_view,
            list_view,
            edit_widget,
            ok_button,
            cancel_button,
        }
    }

    pub fn tree_view(&self) -> &TreeViewInterpreter<'a> {
        &self.tree_view
    }

    pub fn list_view(&self) -> &ListViewInterpreter<'a> {
        &self.list_view
    }

    pub fn edit_widget(&self) -> &EditWidgetInterpreter<'a> {
        &self.edit_widget
    }

    pub fn ok_button(&self) -> &ButtonWidgetInterpreter<'a> {
        &self.ok_button
    }

    pub fn cancel_button(&self) -> &ButtonWidgetInterpreter<'a> {
        &self.cancel_button
    }

    pub fn is_focused(&self) -> bool {
        self.meta.focused
    }

    pub fn meta(&self) -> &Metadata {
        self.meta
    }
}
