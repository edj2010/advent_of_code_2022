use advent_of_code::parse::{parsers, Parser};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[derive(Debug)]
enum File {
    File { name: PathBuf, size: usize },
    Directory { name: PathBuf },
}

impl File {
    fn directory(parent: &Path, name: &str) -> Self {
        Self::Directory {
            name: parent.join(name),
        }
    }

    fn file(parent: &Path, name: &str, size: usize) -> Self {
        Self::File {
            name: parent.join(name),
            size,
        }
    }
}

fn directory_sizes(
    directory: &Path,
    directories: &HashMap<PathBuf, Vec<File>>,
) -> HashMap<PathBuf, usize> {
    let (mut map, size) = directories.get(directory).unwrap().iter().fold(
        (HashMap::new(), 0_usize),
        |(mut map, this_size), file| match file {
            File::Directory { name } => {
                map.extend(directory_sizes(name, directories));
                let size = this_size + map.get(name).unwrap();
                (map, size)
            }
            File::File { name: _, size } => (map, this_size + size),
        },
    );
    map.insert(directory.to_path_buf(), size);
    map
}

#[derive(Debug, PartialEq, Eq)]
enum FileSpec {
    File { name: String, size: usize },
    Directory { name: String },
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Ls(Vec<FileSpec>),
    Cd(String),
}

fn populate_directory(commands: Vec<Command>) -> HashMap<PathBuf, Vec<File>> {
    let mut directories: HashMap<PathBuf, Vec<File>> = HashMap::new();
    let mut current_directory = PathBuf::new();
    commands.into_iter().for_each(|command| match command {
        Command::Cd(s) => {
            current_directory = if s.as_str() == ".." {
                current_directory.parent().unwrap().to_owned()
            } else {
                current_directory.join(s)
            }
        }
        Command::Ls(v) => v.into_iter().for_each(|file_spec| {
            directories
                .entry(current_directory.clone())
                .or_insert_with(Vec::new)
                .push(match file_spec {
                    FileSpec::Directory { name } => File::directory(&current_directory, &name),
                    FileSpec::File { name, size } => File::file(&current_directory, &name, size),
                })
        }),
    });
    directories
}

macro_rules! parse {
    ($input: ident) => {
        parsers::tag("$ ")
            .ignore(
                parsers::tag("ls\n")
                    .ignore(
                        parsers::tag("dir ")
                            .ignore(
                                parsers::many_chars(|c| c != '\n')
                                    .map(|name| FileSpec::Directory { name }),
                            )
                            .or(parsers::number()
                                .pair(" ", parsers::many_chars(|c| c != '\n'))
                                .map(|(size, name)| FileSpec::File { name, size }))
                            .many_lines("\n")
                            .map(|iter| Command::Ls(iter.collect::<Vec<FileSpec>>())),
                    )
                    .or(parsers::tag("cd ")
                        .ignore(parsers::many_chars(|c| c != '\n'))
                        .line("\n")
                        .map(|s| Command::Cd(s))),
            )
            .many()
            .parse($input)
            .finish()
            .unwrap()
            .collect::<Vec<Command>>()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let directories = populate_directory(parse!(input));
    let directory_sizes = directory_sizes(Path::new("/"), &directories);
    directory_sizes
        .into_iter()
        .filter_map(|(_, size)| if size < 100000 { Some(size) } else { None })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let directories = populate_directory(parse!(input));
    let directory_sizes = directory_sizes(Path::new("/"), &directories);
    let threshold = 30000000 - (70000000 - directory_sizes.get(&PathBuf::from("/")).unwrap());
    directory_sizes
        .into_iter()
        .filter_map(|(_, size)| if size > threshold { Some(size) } else { None })
        .min()
        .unwrap()
}

#[test]
fn simple() {
    let input = "$ cd /
";
    let result = parsers::tag("$ ")
        .ignore(
            parsers::tag("cd ")
                .ignore(parsers::many_chars(|c| c != '\n'))
                .line("\n")
                .map(|s| Command::Cd(s)),
        )
        .many()
        .parse(input)
        .finish()
        .unwrap()
        .collect::<Vec<Command>>();
    assert_eq!(result, vec![Command::Cd("/".to_owned())])
}

#[test]
fn test_part1() {
    let input = "$ cd /
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
7214296 k
";
    assert_eq!(part1(input), 95437);
}
