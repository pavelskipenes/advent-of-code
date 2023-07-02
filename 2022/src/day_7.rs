use std::{fmt::Display, ops::Deref, rc::Rc};

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

    fn set_parent(&mut self, parent_node: Rc<FileNode>) {
        match self {
            FileNode::Directory(dir) => dir.parent = Some(parent_node),
            FileNode::File(file) => file.parent = Some(parent_node),
        }
    }

    fn get_parent(&self) -> Option<Rc<FileNode>> {
        match self {
            FileNode::Directory(dir) => dir.parent.clone(),
            FileNode::File(file) => file.parent.clone(),
        }
    }
    fn get_children(&self) -> Vec<Rc<FileNode>> {
        match self {
            FileNode::Directory(dir) => dir.children.clone(),
            FileNode::File(_) => panic!("requested children but this is a file"),
        }
    }

    fn get_directory(&self) -> &Directory {
        match self {
            FileNode::Directory(dir) => dir,
            FileNode::File(_) => panic!("requested directory but this is a file"),
        }
    }
    fn get_file(&self) -> &File {
        match self {
            FileNode::Directory(_) => panic!("requested file but this is a directory"),
            FileNode::File(file) => file,
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
            children: vec![],
            parent: None,
        }
    }

    fn get_size(&self) -> usize {
        let mut sum = 0;
        for child in &self.children {
            sum += match child.deref() {
                FileNode::Directory(dir) => dir.get_size(),
                FileNode::File(file) => file.size,
            }
        }
        sum
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
    parent: Option<Rc<FileNode>>,
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: &'static str,
    children: Vec<Rc<FileNode>>,
    parent: Option<Rc<FileNode>>,
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

#[derive(Clone)]
struct Tree {
    nodes: Vec<FileNode>,
    current_working_directory: Option<Rc<FileNode>>,
}

impl Tree {
    fn new(root: Directory) -> Self {
        let root = FileNode::Directory(root);
        let nodes = vec![root];

        let tmp = Self {
            nodes,
            current_working_directory: None,
        };
        Self {
            nodes: tmp.nodes.clone(),
            current_working_directory: tmp.get_root().map(Rc::new),
        }
    }

    fn get_root(self) -> Option<FileNode> {
        self.nodes.first().cloned()
    }

    fn add_child(&mut self, file_node: FileNode) {
        // add node to the list, link parent and children together
        let file_node = match file_node {
            FileNode::Directory(dir) => FileNode::Directory(Directory {
                name: dir.name,
                children: dir.children,
                parent: Some(self.current_working_directory.as_ref().unwrap().clone()),
            }),
            FileNode::File(file) => FileNode::File(File {
                name: file.name,
                size: file.size,
                parent: Some(self.current_working_directory.as_ref().unwrap().clone()),
            }),
        };

        self.nodes.push(file_node);
    }

    fn change_working_directory(self, target: ChangeDir) -> Self {
        let next_working_directory = match target {
            ChangeDir::Dir(target_name) => {
                // check if target is child in current working directory
                let children = self
                    .current_working_directory
                    .clone()
                    .unwrap()
                    .get_children();
                let target = children
                    .iter()
                    .find(|child| child.get_name() == target_name);

                
                match target {
                    Some(target) => target.clone(),
                    None => {
                        return Self {
                            nodes: self.nodes.clone(),
                            current_working_directory: self.get_root().map(Rc::new),
                        }
                    }
                }
            }
            ChangeDir::Up => {
                // Check if current working directory has a parent
                let result = match self
                    .current_working_directory
                    .as_ref()
                    .unwrap()
                    .get_parent()
                {
                    Some(parent) => parent,
                    None => {
                        return Self {
                            nodes: self.clone().nodes,
                            current_working_directory: self
                                .get_root()
                                .map(Rc::new),
                        }
                    }
                };
                result
            }
            ChangeDir::Root => {
                let result = self.nodes.first().cloned().unwrap();
                Rc::new(result)
            }
        };

        Self {
            nodes: self.nodes,
            current_working_directory: Some(next_working_directory),
        }
    }
}

fn generate_file_structure(input: &'static str) -> Result<(), Error> {
    let mut tree = Tree::new(Directory {
        name: "/",
        children: vec![],
        parent: None,
    });

    // parse lines
    for line in input.lines() {
        let parsed_line = Line::try_from(line)?;

        tree = match parsed_line {
            Line::CommandOutput(new_node) => {
                tree.add_child(new_node);
                tree
            }
            Line::CommandInput(command) => match command {
                Command::Ls => tree,
                Command::Cd(target) => tree.change_working_directory(target),
            },
        };
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::ChangeDir;

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
        let what_is_this = ChangeDir::try_from("asdasd");

        let dir = match what_is_this {
            Ok(dir) => dir,
            Err(_) => unreachable!(""),
        };

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
