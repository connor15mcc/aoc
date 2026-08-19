use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn main() {
    let input = include_str!("../../day07_part1.txt");
    let result = part_one(input);
    println!("Part 1: {}", result.unwrap());

    let input = include_str!("../../day07_part2.txt");
    let result = part_two(input);
    println!("Part 2: {}", result.unwrap());
}

type Path = Vec<String>;

#[derive(Debug)]
struct Filesystem {
    directories: HashSet<Path>,
    files: HashMap<Path, Size>,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            directories: HashSet::from([Path::new()]),
            files: HashMap::new(),
        }
    }

    fn from_observed(commands: &[Command]) -> Result<Filesystem, String> {
        let mut fs = Filesystem::new();
        let mut cwd = Path::new();

        for command in commands {
            match command {
                Command::ChangeDirectory { destination } => match destination {
                    Destination::Root => cwd.clear(),
                    Destination::Parent => {
                        cwd.pop().expect("can't go beyond the root dir");
                    }
                    Destination::Child(dir) => {
                        cwd.push(dir.to_string());
                        fs.directories.insert(cwd.clone());
                    }
                },
                Command::List { output } => {
                    for listing in output {
                        let mut path = cwd.clone();
                        match listing {
                            DirectoryListing::File { name, size } => {
                                path.push(name.clone());
                                fs.files.insert(path.clone(), *size);
                            }
                            DirectoryListing::Directory { name } => {
                                path.push(name.clone());
                                fs.directories.insert(cwd.clone());
                            }
                        }
                    }
                }
            }
        }
        Ok(fs)
    }

    fn directory_sizes(self) -> HashMap<Path, Size> {
        let mut sizes: HashMap<_, _> = self.directories.into_iter().map(|path| (path, 0)).collect();
        for (file_path, size) in self.files {
            for depth in 0..file_path.len() {
                *sizes.entry(file_path[..depth].to_vec()).or_default() += size;
            }
        }

        sizes
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory { destination: Destination },
    List { output: Vec<DirectoryListing> },
}

impl<'a> FromIterator<&'a str> for Command {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut lines = iter.into_iter();
        let command = lines
            .next()
            .expect("must begin with a command")
            .strip_prefix("$ ")
            .expect("commands are displayed with `$ `");

        let command = command.split_whitespace().collect::<Vec<_>>();
        match command.as_slice() {
            ["ls"] => Command::List {
                output: lines
                    .map(|line| DirectoryListing::from_str(line).expect("valid `ls` output"))
                    .collect(),
            },
            ["cd", dest] => Command::ChangeDirectory {
                destination: Destination::from_str(dest)
                    .expect("`cd` commands have valid arguments"),
            },
            _ => unreachable!("invalid command line"),
        }
    }
}

#[derive(Debug)]
enum Destination {
    Child(String),
    Parent,
    Root,
}

impl FromStr for Destination {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".." => Ok(Destination::Parent),
            "/" => Ok(Destination::Root),
            child => Ok(Destination::Child(child.to_string())),
        }
    }
}

#[derive(Debug)]
enum DirectoryListing {
    File { name: String, size: Size },
    Directory { name: String },
}

impl FromStr for DirectoryListing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        match parts.as_slice() {
            ["dir", name] => Ok(DirectoryListing::Directory {
                name: name.to_string(),
            }),
            [size, name] => Ok(DirectoryListing::File {
                name: name.to_string(),
                size: size.parse().expect("files have u32 size"),
            }),
            _ => Err(format!("unexpected directory listing: `{}`", s)),
        }
    }
}

type Size = u32;

pub fn part_one(input: &str) -> Option<u32> {
    let commands = input
        .trim()
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .chunk_by(|_, next| !next.starts_with('$'))
        .map(|chunk| Command::from_iter(chunk.iter().copied()))
        .collect::<Vec<_>>();

    let fs =
        Filesystem::from_observed(&commands).expect("commands occurred for a possible filesystem");

    let sum = fs
        .directory_sizes()
        .iter()
        .filter_map(|(_dir, &size)| match size {
            i if i > 100_000 => None,
            _ => Some(size),
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_disk_space = 70_000_000;
    let space_for_update = 30_000_000;

    let commands = input
        .trim()
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .chunk_by(|_, next| !next.starts_with('$'))
        .map(|chunk| Command::from_iter(chunk.iter().copied()))
        .collect::<Vec<_>>();

    let fs =
        Filesystem::from_observed(&commands).expect("commands occurred for a possible filesystem");

    let directory_sizes = fs.directory_sizes();
    let total_disk_usage = directory_sizes
        .get(&Vec::new())
        .expect("root dir must have an associated size");
    let unused_space = total_disk_space - total_disk_usage;
    let space_to_create = space_for_update - unused_space;

    let mut possible_directories_for_del = {
        let mut sizes = directory_sizes.values().copied().collect::<Vec<_>>();
        sizes.sort_unstable();
        sizes.into_iter().filter(|&size| size > space_to_create)
    };

    possible_directories_for_del.next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
            $ cd /
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
        "#;
        let result = part_one(input);
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = r#"
            $ cd /
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
        "#;
        let result = part_two(input);
        assert_eq!(result, Some(24933642));
    }
}
