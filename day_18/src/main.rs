use std::{cell::RefCell, panic};

use regex::Regex;

fn main() {
    // Parse all snail numbers
    // SnailNumber {
    // value?
    // children?: Vec<RefCell<SnailNumber>>
    // }
    // findNodeToTheLeft:
    // if current.parent has a child to the left of current => go to this child => start searching depth first choosnig right before left
    // else current = current.parent
    //
    // First step in addition:
    // create new tree with left and right
    //
    //
    let temp_tree = parse_snail_number("[[[[4,3],4],4],[7,[[8,4],9]]]");
    println!("Hello, world!");
}

struct Node {
    parent: Option<Box<Node>>,
    children: Vec<Box<Node>>,
    value: Option<usize>,
};

fn parse_snail_number(input: &str) {
    let regex = Regex::new(r"(\[|\]|\d+|,\[|,)").unwrap();

    let current_node = None; 

    for cap in regex.captures_iter(input) {
        match &cap[0] {
            "[" => {
                let parent = match current_node {
                    Some(node) => Some(Box::new(node)),
                    None => None,
                };

                let new_node = Box::new(Node {
                    parent,
                    children: Vec::new(),
                    value: None,
                });

                if let Some(node) = parent {
                    (*node).children.push(Box::new(*new_node))
                }

                current_node = Some(*new_node);
            }
            "]" => {
                current_node = match current_node {
                    Some(node) => Some(*node.parent.unwrap()),
                    None => None,
                }
            }
            ",[" | "," => {
                // newNode = Node {}
                // currentNode.parent.children.push(newNode)
                // currentNode = newNode
            }
            number => {
                // currentNode.value = number.parse
            }
        }
    }
}
