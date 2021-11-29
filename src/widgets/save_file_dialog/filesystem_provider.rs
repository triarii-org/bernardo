use std::path::{Path, PathBuf};
use std::rc::Rc;

use env_logger::Target;

use crate::widgets::save_file_dialog::filesystem_list_item::FilesystemListItem;
use crate::widgets::tree_view::tree_view_node::TreeViewNode;

pub trait FilesystemProvider {
    fn get_root(&self) -> Rc<dyn TreeViewNode<PathBuf>>;

    fn expand(&mut self, path: &Path) -> bool;

    fn get_files(&self, path: &Path) -> Box<dyn Iterator<Item=FilesystemListItem>>;
}

