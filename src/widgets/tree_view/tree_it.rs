/*
This iterator implements depth-first-order using a double ended queue to emulate recursion,
so I don't have to fight borrow-checker, that seem hard to marry with lazy instantiation.

I got this idea in Zurich Operahouse, watching some ballet. Creativity sprouts from boredom.
 */
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use log::warn;

use crate::widgets::tree_view::tree_view_node::TreeViewNode;

type QueueType<'a, Key> = &'a dyn TreeViewNode<Key>;

pub struct TreeIt<'a, Key: Hash + Eq + Debug> {
    queue: Vec<(u16, QueueType<'a, Key>)>,
    expanded: &'a HashSet<Key>,
}

impl<'a, Key: Hash + Eq + Debug + Clone> TreeIt<'a, Key> {
    pub fn new(root: &'a dyn TreeViewNode<Key>, expanded: &'a HashSet<Key>) -> TreeIt<'a, Key> {
        let mut queue: Vec<(u16, QueueType<'a, Key>)> = Vec::new();

        queue.push((0, root));

        TreeIt {
            queue,
            expanded,
        }
    }
}

impl<'a, Key: Hash + Eq + Debug + Clone> Iterator for TreeIt<'a, Key> {
    type Item = (u16, &'a dyn TreeViewNode<Key>);

    fn next(&mut self) -> Option<Self::Item> {
        warn!("expanded {:?}", self.expanded);

        while self.queue.is_empty() == false {
            let mut head = self.queue.pop().unwrap();
            let (depth, node_ref) = head;

            // If it's expanded, I have to throw all children on the stack.
            if self.expanded.contains(node_ref.id()) {
                for idx in (0..node_ref.num_child()).rev() {
                    let item = node_ref.get_child(idx);
                    self.queue.push(
                        (depth + 1, item)
                    );
                }
            }

            return Some((depth, node_ref))
        }

        None
    }
}

// pub struct OwningTreeIt<Key: Hash + Eq + Debug + Clone> {
//     root: Box<dyn TreeViewNode<Key>>,
//     tree_it: TreeIt<'_, Key>,
// }