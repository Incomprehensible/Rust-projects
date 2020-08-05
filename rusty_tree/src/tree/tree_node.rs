pub type TreeIndex = usize;

pub struct TreeNode {
	pub value: usize,
	pub left: Option<TreeIndex>,
	pub right: Option<TreeIndex>,
}

impl TreeNode {
	pub fn new(value: usize, left: Option<TreeIndex>, right: Option<TreeIndex>) -> Self {
		TreeNode {
			value,
			left,
			right
		}
	}
}