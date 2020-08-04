mod tree;

//use tree::{tree_node, the_tree, preorder_iter};
use tree::{
	tree_node::TreeNode,
	the_tree::Tree,
};


fn main() {
	let a = TreeNode::new(4, None, None);
	let b = TreeNode::new(5, None, None);
	let c = TreeNode::new(2, Some(Box::from(a)), Some(Box::from(b)));
	let d = TreeNode::new(3, None, None);
	let e_root = TreeNode::new(1, Some(Box::from(c)), Some(Box::from(d)));
	let rusty_tree = Tree::new(Some(e_root));

	for _node in rusty_tree.iter() {
		//_node.value *= 100;
	}

	let mut iterator = rusty_tree.iter();
	while let Some(node) = iterator.next() {
		println!("{}", node.value);
	}
    
}
