// Makes a struct that holds the value of the node, and the structs of the left/right node.
// Using a vector list allows for there to be no values, corresponding to no nodes in that direction. 
#[derive(PartialEq)]
pub struct Node{
    pub data : i32,
    pub left : Vec<Node>,
    pub right : Vec<Node>,
} 

impl Node {
    // Displays the value of the root node.
    pub fn display_root(node : &Node) {
        println!("Root -> {}", node.data);
    }

    // The movement system for the preorder traversal.
    pub fn preorder_movement(node : &Node, root_node : &Node) {
        if node.left.is_empty() {
            // If both nodes don't exist, it ends the search through that subtree.
            if node.right.is_empty() {
                return;
            }
            
            // If the left node doesn't exist but the right does, it prints the value
            // of the node and continues searching down its subtree.
            else {
                println!("Node -> {}", node.right[0].data);
                Node::inorder_traversal(&node.right[0], &root_node);
            }
        }
        else {
            // If the left node exists but the right doesn't, it prints the value 
            // of the node and summons the movement function again to check its child nodes.
            if node.right.is_empty() {
                println!("Node -> {}", node.left[0].data);
                Node::inorder_traversal(&node.left[0], &root_node);
            }

            // If both right & left nodes exist, it prints the left ndoe value and moves 
            // through its subtree; when the subtree ends, it then prints the right node value
            // and moves through its subtree.
            else {
                println!("Node -> {}", node.left[0].data);
                Node::inorder_traversal(&node.left[0], &root_node);

                println!("Node -> {}", node.right[0].data);
                Node::inorder_traversal(&node.right[0], &root_node);
            }
        }
    }

    // Movement system for the inorder traversal - very similar to the preorder movement.
    pub fn inorder_traversal(node : &Node, root_node : &Node) {
        if node.left.is_empty() {
            // If both nodes don't exist, it ends the search through that subtree.
            if node.right.is_empty() {
                return;
            }
            
            // If the left node doesn't exist but the right does, it prints the value
            // of the node and continues searching down its subtree.
            else {
                println!("Node -> {}", node.right[0].data);
                Node::inorder_traversal(&node.right[0], &root_node);
            }
        }
        else {
            // If the left node exists but the right doesn't, it prints the value 
            // of the node and summons the movement function again to check its child nodes.
            if node.right.is_empty() {
                println!("Node -> {}", node.left[0].data);
                Node::inorder_traversal(&node.left[0], &root_node);
            }

            // If both right & left nodes exist, it prints the left ndoe value and moves 
            // through its subtree; when the subtree ends, it then prints the right node value
            // and moves through its subtree.
            else {
                println!("Node -> {}", node.left[0].data);
                Node::inorder_traversal(&node.left[0], &root_node);

                // If the root is encountered, it is explicity printed to the user.
                if root_node == node {
                    println!{"Root -> {}", root_node.data};
                }

                println!("Node -> {}", node.right[0].data);
                Node::inorder_traversal(&node.right[0], &root_node);
            }
        }
    }

    // Movement system for the postorder system.
    pub fn postorder_movement(node : &Node, root_node : &Node) {
        // Checks if the node has two children, one child in a specific direction, or no children.
        // It makes the appropriate movement (or no movement at all), and prints the value of its data.
        if !node.left.is_empty() && !node.right.is_empty(){
            Node::postorder_movement(&node.left[0], &root_node);
            Node::postorder_movement(&node.right[0], &root_node);
            if node != root_node {println!("Node -> {}", node.data);}
        }

        else if !node.left.is_empty() && node.right.is_empty() {
            Node::postorder_movement(&node.left[0], &root_node);
            println!("Node -> {}", node.data);
        }

        else if node.left.is_empty() && !node.right.is_empty() {
            Node::postorder_movement(&node.right[0], &root_node);
            println!("Node -> {}", node.data);
        } 

        else {
            println!("Node -> {}", node.data);
        }
        
    }
}
