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

fn populate_directory(input: &str) -> HashMap<PathBuf, Vec<File>> {
    let mut directories: HashMap<PathBuf, Vec<File>> = HashMap::new();
    let mut current_directory = PathBuf::new();
    input.lines().for_each(|line| {
        let tokens: Vec<&str> = line.split(" ").collect();
        match tokens[0] {
            "$" => {
                if tokens[1] == "cd" {
                    if tokens[2] == ".." {
                        current_directory = current_directory.parent().unwrap().to_owned();
                    } else {
                        current_directory = current_directory.join(tokens[2]);
                    }
                }
            }
            "dir" => directories
                .entry(current_directory.clone())
                .or_insert_with(Vec::new)
                .push(File::directory(&current_directory, tokens[1])),
            size => directories
                .entry(current_directory.clone())
                .or_insert_with(Vec::new)
                .push(File::file(
                    &current_directory,
                    tokens[1],
                    size.parse::<usize>().unwrap(),
                )),
        }
    });
    directories
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let directories = populate_directory(input);
    let directory_sizes = directory_sizes(Path::new("/"), &directories);
    directory_sizes
        .into_iter()
        .filter_map(|(_, size)| if size < 100000 { Some(size) } else { None })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let directories = populate_directory(input);
    let directory_sizes = directory_sizes(Path::new("/"), &directories);
    let threshold = 30000000 - (70000000 - directory_sizes.get(&PathBuf::from("/")).unwrap());
    directory_sizes
        .into_iter()
        .filter_map(|(_, size)| if size > threshold { Some(size) } else { None })
        .min()
        .unwrap()
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
7214296 k";
    assert_eq!(part1(input), 95437);
}
