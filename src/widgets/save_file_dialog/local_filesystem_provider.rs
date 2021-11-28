use std::borrow::BorrowMut;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use log::{debug, warn};

use crate::widgets::save_file_dialog::filesystem_provider::FilesystemProvider;
use crate::widgets::save_file_dialog::filesystem_tree::FilesystemNode;
use crate::widgets::tree_view::tree_view_node::TreeViewNode;

pub struct LocalFilesystemProvider {
    root: PathBuf,
    root_node: Rc<FilesystemNode>,
}

impl LocalFilesystemProvider {
    pub fn new(root: PathBuf) -> Self {
        let root_node = Rc::new(FilesystemNode::new(root.clone()));

        LocalFilesystemProvider {
            root,
            root_node,
        }
    }

    // substitutes current node corresponding to path with one where children cache is filled.
    // if path is invalid in current tree, it fails.
    pub fn expand_last(&mut self, path: &Path) -> bool {
        // TODO here I am assuming that self.root is prefix to path. This should be checked.

        let mut curr_node = &mut self.root_node;
        let mut curr_prefix = PathBuf::new();

        let num_components = path.components().count();

        for (idx, c) in path.components().enumerate() {
            let last = idx == num_components - 1;

            if last {
                debug!("expanding {:?}", curr_prefix);
                curr_node.borrow_mut().update_cache();
            } else {
                curr_prefix.push(c);

                if !curr_node.has_child(&curr_prefix) {
                    warn!("{:?} has no child {:?}!", curr_node.id(), curr_prefix);
                    return false;
                }
            }
        }

        true
    }
}

impl FilesystemProvider for LocalFilesystemProvider {
    fn get_root(&self) -> Rc<dyn TreeViewNode<PathBuf>> {
        self.root_node.clone()
    }
}
