use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
struct FileSystem {
    root: Rc<RefCell<Directory>>,
    cwd: Rc<RefCell<Directory>>,
}

impl FileSystem {
    fn new() -> Self {
        let root = Directory {
            name: "/".to_string(),
            parent: None,
            children: vec![],
        };

        let wrapped_root = Rc::new(RefCell::new(root));

        FileSystem {
            root: Rc::clone(&wrapped_root),
            cwd: Rc::clone(&wrapped_root),
        }
    }

    fn process(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::CD(x) => {
                let new_dir = match x.as_str() {
                    "/" => Rc::clone(&self.root),
                    ".." => {
                        if let Some(parent) = &self.cwd.borrow().parent {
                            Rc::clone(&parent)
                        } else {
                            Rc::clone(&self.cwd)
                        }
                    }
                    y => {
                        let y_dir = self.cwd.borrow().child_dir(y).unwrap();
                        Rc::clone(&y_dir)
                    }
                };
                self.cwd = new_dir;
            }
            Instruction::LS(res) => {
                let cwd = &mut self.cwd.borrow_mut();

                for (x, y) in res.iter() {
                    match x {
                        LSTypeSize::Dir => {
                            if cwd.child_dir(y).is_none() {
                                let d: Directory = Directory {
                                    name: y.to_string(),
                                    parent: Some(Rc::clone(&self.cwd)),
                                    children: vec![],
                                };

                                cwd.children.push(Node::Dir(Rc::new(RefCell::new(d))))
                            }
                        }
                        LSTypeSize::Size(s) => {
                            if !cwd.file_exists(y) {
                                let f: File = File {
                                    name: y.to_string(),
                                    parent: Rc::clone(&self.cwd),
                                    size: *s,
                                };
                                cwd.children.push(Node::File(f))
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    children: Vec<Node>,
}

impl Directory {
    fn child_dir(&self, name: &str) -> Option<Rc<RefCell<Directory>>> {
        let child = self.children.iter().find(|n| match n {
            Node::Dir(d) => d.borrow().name == name,
            Node::File(_) => false,
        });
        if let Some(Node::Dir(d)) = child {
            Some(Rc::clone(d))
        } else {
            None
        }
    }

    fn file_exists(&self, name: &str) -> bool {
        self.children.iter().any(|n| match n {
            Node::Dir(_) => false,
            Node::File(f) => f.name == name,
        })
    }
}

#[derive(PartialEq, Debug)]
struct File {
    name: String,
    parent: Rc<RefCell<Directory>>,
    size: usize,
}

#[derive(PartialEq, Debug)]
enum Node {
    Dir(Rc<RefCell<Directory>>),
    File(File),
}

impl Node {
    fn size(&self) -> usize {
        match self {
            Node::Dir(dir) => dir.borrow().children.iter().map(|n| n.size()).sum(),
            Node::File(f) => f.size,
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
        let mut fs = FileSystem::new();
        for instruction in input_fixture().into_iter() {
            fs.process(instruction);
        }
        assert_eq!(fs, FileSystem::new());
    }
}
