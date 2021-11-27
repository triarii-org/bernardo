use std::borrow::Borrow;

use crate::widget::tree_view_node::TreeViewNode;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct StupidTree {
    id: usize,
    children: Vec<StupidTree>,
}

impl StupidTree {
    pub fn new(id: usize, children: Vec<StupidTree>) -> Self {
        StupidTree { id, children }
    }
}

impl AsRef<StupidTree> for StupidTree {
    fn as_ref(&self) -> &StupidTree {
        self
    }
}

impl TreeViewNode<usize> for StupidTree {
    fn id(&self) -> &usize {
        &self.id
    }

    fn label(&self) -> String {
        format!("StupidTree {}", self.id)
    }

    fn children(&self) -> Box<(dyn Iterator<Item=Borrow<dyn TreeViewNode<usize>>> + '_)> {
        let vecbox: Vec<Box<StupidTree>> = self.children.iter().map(|b| Box::new(*b)).collect();

        Box::new(self.children.iter()
            .map(|i| Box::new(i))
        ) // TODO this can fail
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

pub fn get_stupid_tree() -> StupidTree {
    let mut stupid_subtree: Vec<StupidTree> = vec![];

    for i in 0..100 {
        stupid_subtree.push(
            StupidTree::new(40000 + i, vec![])
        );
    }

    StupidTree::new(
        0,
        vec![
            StupidTree::new(
                1,
                vec![
                    StupidTree::new(10001, vec![]),
                    StupidTree::new(10002, vec![]),
                ],
            ),
            StupidTree::new(
                2,
                vec![
                    StupidTree::new(20001, vec![]),
                    StupidTree::new(20002, vec![]),
                    StupidTree::new(20003, vec![StupidTree::new(2000301, vec![])]),
                ],
            ),
            StupidTree::new(4, stupid_subtree),
        ],
    )
}
