use super::tree_node::TreeNode;

pub struct PreorderIter<'a> {
	stack: Vec<&'a TreeNode>,
}

impl<'a> PreorderIter<'a> {
	pub fn new(root: Option<&'a TreeNode>) -> Self {
		if let Some(node) = root {
			PreorderIter {
				stack: vec![node]
			}
		}
		else {
			PreorderIter {
				stack: vec![]
			}
		}
	}
}

impl<'a> Iterator for PreorderIter<'a> {
	type Item = &'a TreeNode;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(node) = self.stack.pop() {
			if let Some(right) = &node.right {
				self.stack.push(right)
			}

			if let Some(left) = &node.left {
				self.stack.push(left)
			}

			return Some(node);
		}
		None
	}

}