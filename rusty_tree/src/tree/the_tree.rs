use super::tree_node::{TreeNode, TreeIndex};
use super::preorder_iter::PreorderIter;

// region based allocation
pub struct Tree {
	arena: Vec<Option<TreeNode>>,
	root: Option<TreeIndex>
}

impl Tree {
	pub fn new() -> Self {
		Tree {
			arena: Vec::new(),
			root: None
		}
	}

	// test it when there's no root and u iter it
	pub fn iter(&self) -> PreorderIter {
		PreorderIter::new(self.root)
	}

	pub fn set_root(&mut self, root: Option<TreeIndex>) {
		self.root = root;
	}

	// altered
	pub fn add_node(&mut self, node: TreeNode) -> TreeIndex {
		let index = self.arena.len();
		self.arena.push(Some(node));
		index
	}

	pub fn remove_node_at(&mut self, index: TreeIndex) -> Option<TreeNode> {
		if let Some(node) = self.arena.remove(index) {
			self.arena.insert(index, None);
			return Some(node);
		}

		None
	}

	// as_ref() converts ref to inner elem &Option<Elem> to Option<&Elem>
	pub fn get_node_at(&self, index: TreeIndex) -> Option<&TreeNode> {
		return if let Some(node) = self.arena.get(index) {
			node.as_ref()
		}
		else {
			None
		}
	}

	// try take mutable node ref to the same index 2 times
	pub fn mut_node_at(&mut self, index: TreeIndex) -> Option<&mut TreeNode> {
		if let Some(node) = self.arena.get_mut(index) {
			return node.as_mut();
		}

		None
	}
}
