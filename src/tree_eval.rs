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
pub fn eval_tree(node: Option<&Node>)->Option<f32>{
    if let Some(node) = node{
        match node{

            Node{value: Token{ tok_type:TokenType::NUM_LIT,.. }, ..}=>{
                return Some(node.value.tok_value.parse::<f32>().unwrap());
            }
            Node{value: Token{ tok_type:TokenType::PLUS_SIGN,.. }, ..}=>{
                return Some(eval_tree(node.branch.l_child.as_ref()).unwrap_or(0.0) + eval_tree(node.branch.r_child.as_ref()).unwrap_or(0.0));
            }

            Node{value: Token{ tok_type:TokenType::MUL_SIGN,.. }, ..}=>{
                return Some(eval_tree(node.branch.l_child.as_ref()).unwrap_or(0.0) * eval_tree(node.branch.r_child.as_ref()).unwrap_or(0.0));
            }
            Node{value: Token{ tok_type:TokenType::DIV_SIGN,.. }, ..}=>{
                return Some(eval_tree(node.branch.l_child.as_ref()).unwrap_or(0.0) / eval_tree(node.branch.r_child.as_ref()).unwrap_or(0.0));
            }
            Node{value: Token{ tok_type:TokenType::SUB_SIGN,.. }, ..}=>{
                return Some(eval_tree(node.branch.l_child.as_ref()).unwrap_or(0.0) - eval_tree(node.branch.r_child.as_ref()).unwrap_or(0.0));
            }

            _=>{todo!();}
        }
    }else{
        return None;
    }
}
