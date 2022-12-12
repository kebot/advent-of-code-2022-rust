// use std::borrow::{Borrow, BorrowMut};
// build the tree, count the file size, sum them
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};

type FileNodeRef = Rc<RefCell<FileNode>>;

#[derive(Debug)]
struct FileNode {
    name: String,
    is_dir: bool,
    size: u32,
    children: HashMap<String, FileNodeRef>,
}

impl FileNode {
    fn root() -> Self {
        Self::dir("/")
    }

    fn dir(name: &str) -> Self {
        FileNode {
            name: String::from(name),
            size: 0,
            children: HashMap::new(),
            is_dir: true
        }
    }

    fn file(name: &str, size: u32) -> Self {
        FileNode {
            name: String::from(name),
            size: size,
            children: HashMap::new(),
            is_dir: false
        }
    }

    fn get_children_dirs (&self) -> Vec<FileNodeRef> {
        let mut dirs = vec![];

        for (_key, value) in self.children.iter() {
            // first * deref of a normal ref, second * deref of Rc. Rust is amazing language
            let item = &**value;

            if item.borrow().is_dir {
                dirs.push(value.clone());
                let mut children_dirs = item.borrow().get_children_dirs();
                dirs.append(&mut children_dirs);
            }
        }

        return dirs
    }
}


struct Shell {
    // root takes the real ownership of the data (as a FileTree)
    root: FileNodeRef,

    // dir_history takes a mutable reference of a FileNode
    dir_history: Vec<FileNodeRef>,
}

impl<'a> Shell {
    fn init() -> Self {
        let root = FileNode::root();

        Shell {
            root: Rc::new(RefCell::new(root)),
            dir_history: vec![],
        }
    }

    fn pwd(&'a self) -> FileNodeRef {
        self.dir_history.last().unwrap_or(&self.root).clone()
    }

    fn cd(&mut self, path: &str) {
        if path == ".." {
            self.dir_history.pop();
            return;
        }

        // why it's so dirty here?
        let binding = self.pwd();
        let binding = (*binding).borrow();
        let file_node = binding.children.get(path);

        if file_node.is_some() {
            let node = file_node.unwrap();
            self.dir_history.push(node.clone())
        }
    }

    fn create_file (&mut self, size: u32, filename: &str) {
        let value = FileNode::file(filename, size);
        let binding = self.pwd();
        let mut binding = (*binding).borrow_mut();

        binding.children.insert(
            String::from(filename),
            Rc::new(RefCell::new(value))
        );
    }

    fn touch(&mut self, size: u32, filename: &str) {
        self.create_file(size, filename);

        // increase the size of the file and their parents (including root node)
        for dir in self.dir_history.iter() {
            // WTF is `&**`
            let item = &**dir;

            let mut m = item.borrow_mut();
            m.size += size;

            // println!("increase {} -> {}", m.name, m.size);
        }

        let mut root = (*self.root).borrow_mut();
        root.size += size;
    }

    fn mkdir(&mut self, dirname: &str) {
        let value = FileNode::dir(dirname);
        let binding = self.pwd();
        let mut binding = (*binding).borrow_mut();

        binding.children.insert(
            String::from(dirname),
            Rc::new(RefCell::new(value))
        );
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut shell = Shell::init();

    for line in input.lines() {
        // println!("{}", line);

        if line.starts_with('$') {
            // Possible Commands: [$, cd, /] or [$, ls]
            let commands: Vec<&str> = line.split(' ').collect();

            if commands[1] == "cd" {
                shell.cd(commands[2]);
            } else if commands[1] == "ls" {
                // don't need do anything for ls
            } else {
                println!("unknown commands {:?}", commands);
            }
        } else {
            // dir <dir_name>
            let outputs: Vec<&str> = line.split(' ').collect();

            if outputs[0] == "dir" {
                shell.mkdir(outputs[1]);
            } else {
                shell.touch(outputs[0].parse().unwrap_or(0), outputs[1])
            }
        }
    }

    let mut total_size = 0;

    let dirs = (*shell.root).borrow().get_children_dirs();
    for dir in dirs {
        let node = (*dir).borrow();

        println!("{} {}", node.name, node.size);
        if node.size < 100000 {
            total_size += node.size;
        }
    }

    if shell.root.borrow().size < 100000 {
        total_size += shell.root.borrow().size
    }

    Some(total_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
