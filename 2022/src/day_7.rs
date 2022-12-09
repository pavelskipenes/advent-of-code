use std::fmt::Display;
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
            _ => Err(Error::Command(ErrorDetails {
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
            Ok(Line::CommandOutput(Node::from(s)))
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
            (None, None) => Err(Error::Command(ErrorDetails {
                input,
                was_about_to_parse: None,
                expected: vec!["..".into(), "/".into(), "<some dir name>".into()],
            })),
            (_, _) => Ok(Self::Dir(input)),
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
        }
    }
}
impl Node<'_> {
    fn get_name(&self) -> &'static str {
        match self {
            Node::Directory(a) => a.name,
            Node::File(a) => a.name,
        }
    }

    fn insert(&mut self, node: Node) -> Result<(), ()> {
        match self {
            Node::Directory(dir) => {
                dir.insert(node);
                Ok(())
            }
            Node::File(_) => Err(()),
        }
    }
}
impl From<&'static str> for Node<'_> {
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
impl Directory<'_> {
    fn cd(&mut self, to: &ChangeDir) -> Option<&mut Self> {
        match to {
            ChangeDir::Up => todo!("We actually don't have info about our parent"),
            ChangeDir::Root => todo!("We don't have info about where is root."),
            ChangeDir::Dir(name) => match self.get_node_mut(name) {
                Some(a) => match a {
                    Node::Directory(directory) => Some(directory),
                    Node::File(_) => None,
                },
                None => None,
            },
        }
    }

    fn get_size(&self) -> usize {
        self.nodes
            .iter()
            .map(|node| match node {
                Node::Directory(directory) => directory.get_size(),
                Node::File(file) => file.size,
            })
            .sum()
    }
    fn get_node(&self, node_name: &str) -> Option<&Node> {
        for node in &self.nodes {
            let found = match node {
                Node::Directory(a) => a.name == node_name,
                Node::File(a) => a.name == node_name,
            };
            if found {
                return Some(node);
            }
        }
        None
    }
    fn from(s: &'static str) -> Self {
        Self {
            name: s,
            nodes: vec![],
            parent: None,
        }
    }
    fn get_node_mut(&mut self, node_name: &str) -> Option<&mut Node> {
        for node in &mut self.nodes {
            let found = match node {
                Node::Directory(a) => a.name == node_name,
                Node::File(a) => a.name == node_name,
            };
            if found {
                return Some(node);
            }
        }
        None
    }
    fn insert(&mut self, node: Node) {
        self.nodes.push(node);
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
enum Error {
    Command(ErrorDetails),
}
#[derive(Debug)]
struct ErrorDetails {
    input: &'static str,
    was_about_to_parse: Option<&'static str>,
    expected: Vec<String>,
}
#[derive(Debug)]
enum Line {
    CommandInput(Command),
    CommandOutput(Node),
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
#[derive(Debug)]
enum Node<'a> {
    Directory(Directory<'a>),
    File(File),
}

#[derive(Debug)]
struct File {
    name: &'static str,
    size: usize,
}

#[derive(Debug)]
struct Directory<'a> {
    name: &'static str,
    nodes: Vec<Node<'a>>,
    parent: Option<&'a Directory<'a>>,
}

fn run(input: &'static str) -> usize {
    let mut root: Directory = Directory {
        name: "/",
        nodes: vec![],
        parent: None,
    };

    // let cwd: CurrentWorkingDirectory = CurrentWorkingDirectory::from_root(root);
    // let cwd = vec![];

    for line in input.lines() {
        let parsed_line = Line::try_from(line);

        if let Err(why) = &parsed_line {
            match why {
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

                    eprintln!("{}", output);
                }
            }
        }

        match parsed_line.ok().unwrap() {
            Line::CommandInput(command) => match command {
                Command::Cd(next_dir) => todo!("cwd.cd(next_dir)"),
                Command::Ls => (),
            },
            Line::CommandOutput(new_node) => root.nodes.push(new_node),
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day_7::run;

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

    const EXAMPLE_INPUT2: [&str; 2] = [
        r"$ cd /
$ ls
dir a
dir d
$ cd a
$ ls
dir e
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
5626152 d.ext
7214296 k",
        "",
    ];

    const ANSWER: [usize; 2] = [0, 0];
    const EXAMPLE_ANSWER: [[usize; 2]; 2] = [[48_381_165, 0], [0, 0]];

    #[test]
    fn example() {
        for (input, expected) in EXAMPLE_INPUT2.iter().zip(EXAMPLE_ANSWER.iter()) {
            assert_eq!(run(input), expected[0]);
            // assert_eq!(todo(input), expected[1]);
        }
    }

    #[test]
    #[ignore]
    fn problem() {
        assert_eq!(run(INPUT), ANSWER[0]);
        // assert_eq!(todo(INPUT), ANSWER[1]);
    }
}
