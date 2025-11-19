use crate::token::{Token,TokenType};
use crate::parser::{Tree,Node};

// 1 + 2
// + { 1, 2 }

pub fn print_tree(node: Option<&Node>)->i32{

    if node != None{ 
    return print_tree(node.unwrap().branch.l_child.as_ref()) + print_tree(node.unwrap().branch.r_child.as_ref());
        if node.unwrap().value.tok_type == TokenType::NUM_LIT {
            if let Ok(parsed_node_value) = node.unwrap().value.tok_value.parse::<i32>(){
                return parsed_node_value;
            }
        }
    }
    return 0;
}
