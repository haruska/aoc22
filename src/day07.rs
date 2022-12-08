use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

type WrappedNode<'a> = Rc<RefCell<Node<'a>>>;
//
// fn new_dir(name: &str, parent: Option<WrappedNode>) -> WrappedNode {
//     let mut node = Node::new(name, FSType::Dir);
//     node.parent = parent;
//     Rc::new(RefCell::new(node))
// }
//
// fn new_file(name: &str, fs_size: usize) -> WrappedNode {
//     let mut node = Node::new(name, FSType::File);
//     node.value = Some(fs_size);
//
//     Rc::new(RefCell::new(node))
// }
//
// fn parent(cwd: WrappedNode) -> WrappedNode {
//     let node: &RefCell<Node> = cwd.borrow();
//     let node = node.borrow();
//     let parent = node.parent.as_ref();
//
//     if let Some(parent) = parent {
//         Rc::clone(parent)
//     } else {
//         panic!("Could not find parent for {:?}", node);
//     }
// }
//
// fn child_dir(cwd: WrappedNode, dir_name: &str) -> WrappedNode {
//     let node: &RefCell<Node> = cwd.borrow();
//     let children = &node.borrow().children;
//     let child = children.iter().find(|n| {
//         let n = Rc::clone(n);
//         let node: &RefCell<Node> = n.borrow();
//         let node = node.borrow();
//         node.fs_type == FSType::Dir && node.name == dir_name
//     });
//
//     if let Some(child) = child {
//         Rc::clone(child)
//     } else {
//         panic!(
//             "Could not find child dir {} in children {:?}",
//             dir_name, children
//         );
//     }
// }

// processes all instructions returning the root
fn process(instructions: Vec<Instruction>) -> WrappedNode {
    let root = Rc::new(RefCell::new(Node::new("/", FSType::Dir)));
    let mut cwd = root.clone();

    for instruction in instructions {
        match instruction {
            Instruction::CD(path) => {
                match path {
                    "/" => cwd = root.clone(),
                    ".." => {
                        let parent = cwd.borrow().parent.clone().unwrap();
                        cwd = parent;
                    }
                    _ => {
                        let child = cwd.borrow_mut().children.get_mut(path).unwrap().clone();
                        cwd = child;
                    }
                };
            }
            Instruction::LS(entries) => {
                for entry in entries {
                    match entry {
                        Entry::Dir(dir_name) => {
                            let entry = cwd.borrow_mut().children.entry(dir_name).or_insert(Rc::new(RefCell::new(Node::new(dir_name, FSType::Dir)))).clone();
                            entry.borrow_mut().parent = Some(cwd.clone());
                        }
                        Entry::File(fname, fsize) => {
                            let entry = cwd.borrow_mut().children.entry(fname).or_insert(Rc::new(RefCell::new(Node::new(fname, FSType::File)))).clone();
                            entry.borrow_mut().fs_size = Some(fsize);
                            entry.borrow_mut().parent = Some(cwd.clone());
                        }
                    }
                }
            }
        }
    }
    root
}

#[derive(PartialEq)]
struct Node<'a> {
    name: Option<&'a str>,
    fs_type: Option<FSType>,
    fs_size: Option<usize>,
    children: HashMap<&'a str, WrappedNode<'a>>,
    parent: Option<WrappedNode<'a>>,
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("fs_type", &self.fs_type)
            .field("size", &self.fs_size)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(PartialEq, Debug)]
enum FSType {
    File,
    Dir,
}

impl<'a> Node<'a> {
    fn new(name: &str, fs_type: FSType) -> Node {
        Node {
            name: Some(name),
            fs_type: Some(fs_type),
            fs_size: None,
            parent: None,
            children: HashMap::new(),
        }
    }

    fn fs_size(&self) -> usize {
        if let Some(fs_type) = &self.fs_type {
            match fs_type {
                FSType::Dir => self
                    .children
                    .iter()
                    .map(|(_, n)| {

                        n.borrow().fs_size()
                    })
                    .sum(),
                FSType::File => self.fs_size.unwrap(),
            }
        } else {
            0
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    CD(&'a str),
    LS(Vec<Entry<'a>>),
}

#[derive(Debug, PartialEq)]
enum Entry<'a> {
    Dir(&'a str),
    File(&'a str, usize)
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .split("\n$ ")
        .enumerate()
        .map(|(i, cmd)| {
            if i == 0 {
                Instruction::CD("/")
            } else {
                let cmd_str = &cmd[..2];
                if cmd_str == "cd" {
                    Instruction::CD(&cmd[3..])
                } else { // "ls"
                    let entries = cmd
                        .lines()
                        .skip(1)
                        .map(|l| {
                            let (x, name) = l.split_once(" ").unwrap();
                            if x == "dir" {
                                Entry::Dir(name)
                            } else {
                                let size: usize = x.parse().unwrap();
                                Entry::File(name, size)
                            }
                        })
                        .collect();
                    Instruction::LS(entries)
                }
            }
        })
        .collect()
}

fn all_dirs(root: WrappedNode) -> Vec<WrappedNode> {
    let mut dirs = vec![];

    let mut dirs_to_visit = vec![root.clone()];
    while let Some(cwd) = dirs_to_visit.pop() {
        for (_name, child) in &cwd.borrow().children {
            if child.borrow().fs_type == Some(FSType::Dir) {
                dirs.push(child.clone());
                dirs_to_visit.push(child.clone());
            }
        }
    }
    dirs
}

fn part_one(root: WrappedNode) -> usize {
    let dirs = all_dirs(root);

    dirs.iter().map(|dir| {
        dir.clone().borrow().fs_size()
    }).filter(|s| *s <= 100000).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day07.txt");
    let instructions = parse(input);

    let root = process(instructions);

    let p1 = part_one(root);
    println!("Part One: {p1}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;

    fn input_fixture() -> Vec<Instruction<'static>> {
        vec![
            Instruction::CD("/"),
            Instruction::LS(vec![
                Entry::Dir("a"),
                Entry::File("b.txt", 14848514),
                Entry::File("c.dat", 8504156),
                Entry::Dir("d")
            ]),
            Instruction::CD("a"),
            Instruction::LS(vec![
                Entry::Dir("e"),
                Entry::File("f", 29116),
                Entry::File("g", 2557),
                Entry::File("h.lst", 62596)
            ]),
            Instruction::CD("e"),
            Instruction::LS(vec![Entry::File("i", 584)]),
            Instruction::CD(".."),
            Instruction::CD(".."),
            Instruction::CD("d"),
            Instruction::LS(vec![
                Entry::File("j", 4060174),
                Entry::File("d.log", 8033020),
                Entry::File("d.ext", 5626152),
                Entry::File("k", 7214296),
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
    fn fs_size_test() {
        let root = process(input_fixture());
        let root_size: usize = root.borrow().fs_size();

        assert_eq!(root_size, 48381165)
    }

    #[test]
    fn all_dirs_test() {
        let root = process(input_fixture());
        let dirs = all_dirs(root);

       let dir_names: Vec<&str> = dirs.iter().map(|dir| {
            dir.clone().borrow().name.unwrap()
        }).sorted().collect();

        assert_eq!(dir_names, vec!["a", "d", "e"])
    }

    #[test]
    fn part_one_test() {
        let root = process(input_fixture());
        let res = part_one(root);

        assert_eq!(res, 95437)
    }
}
