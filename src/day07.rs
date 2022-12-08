use serde::__private::de::Borrowed;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::error::Error;
use std::rc::Rc;

type WrappedNode = Rc<RefCell<Node>>;

fn new_dir(name: &str) -> WrappedNode {
    let node = Node::new(name, FSType::Dir);
    Rc::new(RefCell::new(node))
}

fn new_file(name: &str, fs_size: usize) -> WrappedNode {
    let mut node = Node::new(name, FSType::File);
    node.value = Some(fs_size);

    Rc::new(RefCell::new(node))
}

// processes all instructions returning the root
fn process(instructions: Vec<Instruction>) -> WrappedNode {
    let root = Rc::new(RefCell::new(Node::new("/", FSType::Dir)));
    let mut cwd: Rc<RefCell<Node>> = Rc::clone(&root);

    for instruction in instructions {
        match instruction {
            Instruction::CD(x) => {
                match x.as_str() {
                    "/" => cwd = Rc::clone(&root),
                    ".." => {
                        let node: &RefCell<Node> = cwd.borrow();
                        let node = node.borrow();
                        let parent: &Rc<RefCell<Node>> = node.parent.as_ref().unwrap();
                        cwd = Rc::clone(parent);
                    }
                    dir_name => {
                        let node: &RefCell<Node> = cwd.borrow();
                        let children = &node.borrow().children;
                        cwd = Rc::clone(
                            children
                                .iter()
                                .find(|n| {
                                    let node: &RefCell<Node> = cwd.borrow();
                                    let node = node.borrow();
                                    node.fs_type == FSType::Dir && node.name == dir_name
                                })
                                .unwrap(),
                        );
                    }
                };
            }
            Instruction::LS(res) => {
                for (ls_type, ls_val) in res.iter() {
                    match ls_type {
                        LSTypeSize::Dir => {
                            let node: &RefCell<Node> = cwd.borrow();
                            let mut children = &node.borrow().children;
                            if !children.iter().any(|wn| {
                                let node: &RefCell<Node> = wn.borrow();
                                let node = node.borrow();
                                node.fs_type == FSType::Dir && node.name.as_str() == ls_val
                            }) {
                                let child_dir = new_dir(ls_val);
                                children.push(child_dir);
                            }
                        }
                        LSTypeSize::Size(s) => {
                            let node: &RefCell<Node> = cwd.borrow();
                            let mut children = &node.borrow().children;
                            if !children.iter().any(|wn| {
                                let node: &RefCell<Node> = wn.borrow();
                                let node = node.borrow();
                                node.fs_type == FSType::File && node.name.as_str() == ls_val
                            }) {
                                let fs_size: usize = ls_val.parse().unwrap();
                                let child_file = new_file(ls_val, fs_size);
                                children.push(child_file)
                            }
                        }
                    }
                }
            }
        }
    }
    root
}

#[derive(PartialEq, Debug)]
struct Node {
    name: String,
    fs_type: FSType,
    value: Option<usize>,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

#[derive(PartialEq, Debug)]
enum FSType {
    File,
    Dir,
}

impl Node {
    fn new(name: &str, fs_type: FSType) -> Node {
        Node {
            name: name.to_string(),
            fs_type,
            value: None,
            parent: None,
            children: vec![],
        }
    }

    fn fs_size(&self) -> usize {
        match self.fs_type {
            FSType::Dir => self
                .children
                .iter()
                .map(|n| {
                    let node: &RefCell<Node> = n.borrow();
                    let node = node.borrow();
                    node.fs_size()
                })
                .sum(),
            FSType::File => self.value.unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    CD(String),
    LS(Vec<(LSTypeSize, String)>),
}

#[derive(Debug, PartialEq)]
enum LSTypeSize {
    Dir,
    Size(usize),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .split("\n$ ")
        .enumerate()
        .map(|(i, cmd)| {
            if i == 0 {
                Instruction::CD("/".to_string())
            } else {
                let cmd_str = &cmd[..2];
                if cmd_str == "cd" {
                    Instruction::CD(cmd[3..].to_string())
                } else {
                    let file_list = cmd
                        .lines()
                        .skip(1)
                        .map(|l| {
                            let (x, name) = l.split_once(" ").unwrap();
                            let name = name.to_string();
                            if x == "dir" {
                                (LSTypeSize::Dir, name)
                            } else {
                                let size: usize = x.parse().unwrap();
                                (LSTypeSize::Size(size), name)
                            }
                        })
                        .collect();
                    Instruction::LS(file_list)
                }
            }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_fixture() -> Vec<Instruction> {
        vec![
            Instruction::CD("/".to_string()),
            Instruction::LS(vec![
                (LSTypeSize::Dir, "a".to_string()),
                (LSTypeSize::Size(14848514), "b.txt".to_string()),
                (LSTypeSize::Size(8504156), "c.dat".to_string()),
                (LSTypeSize::Dir, "d".to_string()),
            ]),
            Instruction::CD("a".to_string()),
            Instruction::LS(vec![
                (LSTypeSize::Dir, "e".to_string()),
                (LSTypeSize::Size(29116), "f".to_string()),
                (LSTypeSize::Size(2557), "g".to_string()),
                (LSTypeSize::Size(62596), "h.lst".to_string()),
            ]),
            Instruction::CD("e".to_string()),
            Instruction::LS(vec![(LSTypeSize::Size(584), "i".to_string())]),
            Instruction::CD("..".to_string()),
            Instruction::CD("..".to_string()),
            Instruction::CD("d".to_string()),
            Instruction::LS(vec![
                (LSTypeSize::Size(4060174), "j".to_string()),
                (LSTypeSize::Size(8033020), "d.log".to_string()),
                (LSTypeSize::Size(5626152), "d.ext".to_string()),
                (LSTypeSize::Size(7214296), "k".to_string()),
            ]),
        ]
    }

    #[test]
    fn parse_test() {
        let input = include_str!("../input/day07_test.txt");
        let result = parse(input);
        let expected = input_fixture();

        assert_eq!(result, expected);
    }

    #[test]
    fn building_file_system_test() {
        let root = process(input_fixture());
        assert_eq!(root, new_dir("/"));
    }
}
