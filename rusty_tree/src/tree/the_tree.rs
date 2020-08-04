use super::tree_node::TreeNode;
use super::preorder_iter::PreorderIter;

pub struct Tree {
	root: Option<TreeNode>
}

impl Tree {
	pub fn new(root: Option<TreeNode>) -> Self {
		Tree {
			root
		}
	}

	pub fn iter(&self) -> PreorderIter {
		PreorderIter::new(self.root.as_ref())
	}
}
