use tree_lib;

fn main() {
    let left_left_node = tree_lib::Node{data : 60,
                                        left : vec![],
                                        right : vec![]};

    let left_right_node = tree_lib::Node{data : 1000,
                                        left : vec![],
                                        right : vec![]};

    let left_node = tree_lib::Node{data : 20,
                                left : vec![left_left_node],
                                right : vec![left_right_node]};

    let right_right_node = tree_lib::Node{data : 30,
                                          left : vec![],
                                          right : vec![]};

    let right_left_node = tree_lib::Node{data : 100,
                                          left : vec![],
                                          right : vec![]};

    let right_node = tree_lib::Node{data : 15,
                                left : vec![right_left_node],
                                right : vec![right_right_node]};

    let root = tree_lib::Node{data : 10,
                              left : vec![left_node],
                              right : vec![right_node]};

    // Here's a visualisation of the structure of the values above:
/* 
                               root
                            /        \
                   left_node         right_node
                    /  \               /      \
        left_left_node   left-    |  right-    right_right_node
                       right node | left node 
*/
    // Runs each traversal function.
    println!("Preorder:");
    preorder_traversal(&root);
    println!("\n");
    
    println!("Postorder:");
    postorder_traversal(&root);
    println!("\n");

    println!("Inorder:");
    tree_lib::Node::inorder_traversal(&root, &root);
}

// Displays the root, and then runs the traversal function.
fn preorder_traversal(root_node : &tree_lib::Node) {
    tree_lib::Node::display_root(root_node);
    tree_lib::Node::preorder_movement(root_node, root_node);
}

// Runs the traversal function, then displays the root.
fn postorder_traversal(root_node : &tree_lib::Node) {
    tree_lib::Node::postorder_movement(root_node, root_node);
    tree_lib::Node::display_root(root_node);
}
