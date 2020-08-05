mod tree;

//use tree::{tree_node, the_tree, preorder_iter};
use tree::{
	tree_node::TreeNode,
	the_tree::Tree,
};


fn main() {
	let mut rusty_tree = Tree::new();
	let a = rusty_tree.add_node(TreeNode::new(4, None, None));
	let b = rusty_tree.add_node(TreeNode::new(5, None, None));
	let c = rusty_tree.add_node(TreeNode::new(2, Some(a), Some(b)));
	let d = rusty_tree.add_node(TreeNode::new(3, None, None));
	let e_root = rusty_tree.add_node(TreeNode::new(1, Some(c), Some(d)));

	rusty_tree.set_root(Some(e_root));

	let mut preorder = rusty_tree.iter();

	while let Some(node_index) = preorder.next(&rusty_tree) {
		if let Some(node) = rusty_tree.mut_node_at(node_index) {
			(*node).value *= 10;
		}
	}

	let mut preorder_imm = rusty_tree.iter();

	while let Some(node_index) = preorder_imm.next(&rusty_tree) {
		//let node = rusty_tree.get_node_at(node_index).expect("no node at that index");
		if let Some(node) = rusty_tree.get_node_at(node_index) {
			println!("{}", node.value);
		}
	}



}
