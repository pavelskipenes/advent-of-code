use std::{fmt::Display, rc::Rc};

use indextree::{Arena, NodeId};
impl TryFrom<&'static str> for Command {
    type Error = Error;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        // "cd arg"
        // "ls"
        match &value[0..=1] {
            "cd" => {
                // strip off "cd "
                let cd = ChangeDir::try_from(consume(value, "cd "))?;
                Ok(Command::Cd(cd))
            }
            "ls" => Ok(Command::Ls),
            _ => Err(Error::Command(ErrorParsingDetails {
                input: value,
                was_about_to_parse: Some(&value[0..=1]),
                expected: vec!["cd".into(), "ls".into()],
            })),
        }
    }
}
impl TryFrom<&'static str> for Line {
    type Error = Error;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        // "$..." -> Command
        // "_" -> Not Command

        let first_char = &s.chars().peekable().next().expect("empty input");

        if let '$' = *first_char {
            let cmd = Command::try_from(consume(s, "$ "))?;
            Ok(Line::CommandInput(cmd))
        } else {
            Ok(Line::CommandOutput(FileNode::from(s)))
        }
    }
}
impl TryFrom<&'static str> for ChangeDir {
    type Error = Error;

    fn try_from(input: &'static str) -> Result<Self, Self::Error> {
        let mut peek = input.chars().peekable();
        let (first, second) = (peek.next(), peek.next());
        match (first, second) {
            (Some('.'), Some('.')) => Ok(Self::Up),
            (Some('/'), None) => Ok(Self::Root),
            (None, None) => Err(Error::Command(ErrorParsingDetails {
                input,
                was_about_to_parse: None,
                expected: vec!["..".into(), "/".into(), "<some dir name>".into()],
            })),
            (_, _) => Ok(Self::Dir(input)),
        }
    }
}
impl ChangeDir {
    fn get_name(&self) -> &'static str {
        match self {
            ChangeDir::Up => todo!("get parent name"),
            ChangeDir::Root => todo!("get root"),
            ChangeDir::Dir(dir) => dir,
        }
    }
}
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.size, self.name)
    }
}
impl From<&'static str> for File {
    fn from(s: &'static str) -> Self {
        // 62596 h.lst -> size = 62596, name = "h.lst"
        let mut size = String::new();
        for character in s.chars() {
            match character {
                '0'..='9' => size.push(character),
                ' ' => break,
                _ => todo!("convert to fallible function"),
            }
        }
        Self {
            size: size
                .parse::<usize>()
                .expect("failed to parse a string into a number"), // should never happen
            name: &s[size.len() + 1..],
            parent: None,
        }
    }
}
impl FileNode {
    fn get_name(&self) -> &'static str {
        match self {
            FileNode::Directory(a) => a.name,
            FileNode::File(a) => a.name,
        }
    }

    fn is_directory(&self) -> bool {
        match &self {
            FileNode::Directory(_) => true,
            FileNode::File(_) => false,
        }
    }

    fn is_file(&self) -> bool {
        !self.is_directory()
    }

    fn set_parent(&mut self, parent_node: NodeId) {
        match self {
            FileNode::Directory(dir) => dir.parent = Some(parent_node),
            FileNode::File(file) => file.parent = Some(parent_node),
        }
    }
}
impl From<&'static str> for FileNode {
    fn from(s: &'static str) -> Self {
        // dir e
        // 29116 f.txt
        let first_four_characters = &s[0..=3];
        match first_four_characters {
            // hmm... we don't know the contents of the directory just yet
            // construct it from path or make a tree like structure maybe?
            "dir " => Self::Directory(Directory::from(&s[4..])),
            _ => Self::File(File::from(s)),
        }
    }
}
impl Directory {
    fn from(s: &'static str) -> Self {
        Self {
            name: s,
            size: None,
            children: vec![],
            parent: None,
        }
    }
}

/// Effectively removes `eat` from `input` without copying
fn consume(input: &'static str, eat: &str) -> &'static str {
    for (input_ch, eat_ch) in input.chars().zip(eat.chars()) {
        assert_eq!(
            input_ch, eat_ch,
            "failed to consume '{}' from the start of '{}'",
            eat, input
        );
    }
    &input[eat.len()..]
}

#[derive(Debug)]
pub enum Error {
    Command(ErrorParsingDetails),
    DirectoryNotFound,
}

#[derive(Debug)]
pub struct ErrorParsingDetails {
    input: &'static str,
    was_about_to_parse: Option<&'static str>,
    expected: Vec<String>,
}
#[derive(Debug)]
enum Line {
    CommandInput(Command),
    CommandOutput(FileNode),
}
#[derive(Debug)]
enum Command {
    Ls,
    Cd(ChangeDir),
}
#[derive(Debug)]
enum ChangeDir {
    Dir(&'static str),
    Up,
    Root,
}
#[derive(Debug, Clone)]
pub enum FileNode {
    Directory(Directory),
    File(File),
}

#[derive(Debug, Clone)]
pub struct File {
    name: &'static str,
    size: usize,
    parent: Option<NodeId>,
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: &'static str,
    size: Option<usize>,
    children: Vec<NodeId>,
    parent: Option<NodeId>,
}

pub fn print_error(error: Error) {
    match error {
        Error::Command(details) => {
            let mut output = String::from("failed to create command\nExpected on of: [ ");
            for expected in &details.expected {
                output.push_str(&format!("'{}' ", expected.as_str()));
            }
            output.push_str("]\n");
            if let Some(attempt) = details.was_about_to_parse {
                output.push_str(&format!("attempted to parse '{}'\n", attempt));
            }
            output.push_str(&format!("full input: {}", details.input));
            println!("{}", output);
        }
        Error::DirectoryNotFound => {
            println!("Cannot change directory because target directory was not found");
        }
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if attempt is made to change directory into a child directory without knowing it is there.
pub fn generate_file_structure(input: &'static str) -> Result<(Arena<FileNode>, NodeId), Error> {
    // create a new file structure
    let mut tree = indextree::Arena::new();
    tree.reserve(input.lines().count()); // this will allocate too much but we want speed so its ok
    let root_directory = Directory {
        name: "/",
        size: None,
        children: vec![],
        parent: None,
    };

    let root_id = tree.new_node(FileNode::Directory(root_directory));
    let mut cwd_id = root_id;

    for line in input.lines() {
        let parsed_line = Line::try_from(line)?;

        match parsed_line {
            Line::CommandOutput(new_node) => {
                (tree, cwd_id) = add_new_node(&tree, new_node, cwd_id);
            }
            Line::CommandInput(command) => match command {
                Command::Ls => (),
                Command::Cd(target) => {
                    match target {
                        ChangeDir::Up => {
                            cwd_id = match tree.get(cwd_id) {
                                None => root_id,
                                Some(some_parent) => match some_parent.get() {
                                    FileNode::File(_) => unreachable!("parent is a file"),
                                    FileNode::Directory(some_parent_directory) => {
                                        match some_parent_directory.parent {
                                            None => {
                                                assert!(
                                                    cwd_id == root_id,
                                                    "only root can have no parents"
                                                );
                                                root_id
                                            }
                                            Some(some_parent_id) => some_parent_id,
                                        }
                                    }
                                },
                            }
                        }
                        ChangeDir::Root => cwd_id = root_id,
                        ChangeDir::Dir(target_name) => {
                            let mut found = false;
                            for child_id in cwd_id.children(&tree) {
                                match tree.get(child_id) {
                                    None => unreachable!("tree does not have child_id"),
                                    Some(child) => {
                                        if child.get().get_name() == target_name {
                                            cwd_id = child_id;
                                            found = true;
                                            break;
                                        }
                                    }
                                }
                            }

                            if !found {
                                return Err(Error::DirectoryNotFound); // child not found
                            }
                        }
                    }
                }
            },
        };
    }
    print!("{:#?}", root_id.debug_pretty_print(&tree));
    Ok((tree, root_id))
}

pub fn get_size(tree: &Arena<FileNode>, root: NodeId) -> usize {
    let tree = tree.clone();
    match tree.get(root).unwrap().get() {
        FileNode::File(file) => file.size,
        FileNode::Directory(dir) => {
            let mut size = 0;
            for &child in &dir.children {
                size += get_size(&tree, child);
            }
            size
        }
    }
}

pub fn set_size(tree: Arena<FileNode>, root: NodeId) -> Arena<FileNode> {
    // because of shitty borrow checker we cannot just read a value from an object
    // and act upon it. We need to clone the entire tree structure and read
    // whatever we're interested in and then we can modify create a new copy
    // from scratch. This will result in N copies just here.
    let mut new_tree = tree.clone();
    for child in root.children(&tree) {
        // for each directory, set new size in new tree
        match new_tree.get_mut(child).unwrap().get_mut() {
            FileNode::File(_) => (),
            FileNode::Directory(dir) => {
                dir.size = Some(get_size(&tree.clone(), child));
            }
        };
    }
    new_tree
}

/// add_new_node returns new tree and a node to current working directory
///
/// # Panics
///
/// Panics if current working directory does not exists in tree
/// Panics if current working directory is not a directory
fn add_new_node(
    tree: &Arena<FileNode>,
    mut new_node: FileNode,
    cwd: NodeId,
) -> (Arena<FileNode>, NodeId) {
    // create a copy of the tree
    let mut tmp_tree = tree.clone();

    //  set parent
    new_node.set_parent(cwd);
    let new_node_id = tmp_tree.new_node(new_node);

    // get current working directory and add the newly created node to it
    let tmp_tree = tmp_tree.clone();
    let current_working_directory = match tmp_tree.get(cwd) {
        None => panic!("cannot find current working directory by id in tree"),
        Some(current_working_directory) => current_working_directory,
    };
    let mut current_working_directory = match current_working_directory.get() {
        FileNode::File(_) => {
            panic!("current working directory is not a directory but a file")
        }
        FileNode::Directory(current_working_directory) => current_working_directory,
    }
    .clone();
    current_working_directory.children.push(new_node_id);

    // let cwd know the id of the new node
    let mut tmp_tree = tmp_tree.clone();
    cwd.append(new_node_id, &mut tmp_tree);

    // set children
    match tmp_tree.get_mut(cwd) {
        None => unreachable!("cwd is empty"),
        Some(directory) => match directory.get_mut() {
            FileNode::File(_) => unreachable!("cwd is a file not a directory"),
            FileNode::Directory(dir) => dir.children.push(new_node_id),
        },
    }

    // return modified tree
    (tmp_tree, cwd)
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../puzzle_input/day_7.txt");
    const EXAMPLE_INPUT: [&str; 2] = [
        r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        "",
    ];

    const ANSWER: [usize; 2] = [0, 0];
    const EXAMPLE_ANSWER: [[usize; 2]; 2] = [[48_381_165, 0], [0, 0]];

    #[test]
    #[ignore]
    fn example() {
        for (input, expected) in EXAMPLE_INPUT.iter().zip(EXAMPLE_ANSWER.iter()) {
            // assert_eq!(generate_file_structure(input), expected[0]);
            // assert_eq!(todo(input), expected[1]);
        }
    }

    #[test]
    #[ignore]
    fn problem() {
        // assert_eq!(generate_file_structure(INPUT), ANSWER[0]);
        // assert_eq!(todo(INPUT), ANSWER[1]);
    }
}
