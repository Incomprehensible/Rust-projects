use super::tree_node::TreeIndex;
use super::the_tree::Tree;

pub struct PreorderIter {
	stack: Vec<TreeIndex>,
}

impl PreorderIter {
	pub fn new(root: Option<TreeIndex>) -> Self {
		if let Some(index) = root {
			PreorderIter {
				stack: vec![index]
			}
		}
		else {
			PreorderIter {
				stack: vec![]
			}
		}
	}

	pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
		while let Some(index) = self.stack.pop() {
			if let Some(node) = tree.get_node_at(index) {
				if let Some(right) = node.right {
					self.stack.push(right);
				}
				if let Some(left) = node.left {
					self.stack.push(left)
				}

				return Some(index);
			}
		}
		None
	}
}
