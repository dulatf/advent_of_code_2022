use simple_error::SimpleError;

use crate::solution::*;

pub struct Day07 {}

#[derive(Debug)]
enum Command {
    ChangeDirectoryToRoot,
    ChangeDirectoryUp,
    ChangeDirectoryDown(String),
    List,
}
impl Command {
    fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[1] == "cd" {
            if parts[2] == "/" {
                Some(Command::ChangeDirectoryToRoot)
            } else if parts[2] == ".." {
                Some(Command::ChangeDirectoryUp)
            } else {
                Some(Command::ChangeDirectoryDown(parts[2].to_owned()))
            }
        } else if parts[1] == "ls" {
            Some(Command::List)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Content {
    Directory(String),
    File(String, usize),
}
impl Content {
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "dir" {
            Some(Content::Directory(parts[1].to_owned()))
        } else {
            if let Ok(size) = parts[0].parse::<usize>() {
                Some(Content::File(parts[1].to_owned(), size))
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
enum LineType {
    Input(Command),
    Output(Content),
}

#[derive(Debug)]
struct Directory {
    pub name: String,
    pub sub_directories: Vec<Box<Directory>>,
    pub files: Vec<File>,
    pub parent: Option<*mut Directory>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            sub_directories: Vec::new(),
            files: Vec::new(),
            parent: None,
        }
    }
    pub fn new_with_parent(name: &str, parent: &mut Directory) -> Self {
        Self {
            name: name.to_owned(),
            sub_directories: Vec::new(),
            files: Vec::new(),
            parent: Some(parent),
        }
    }
}

#[derive(Debug)]
struct File {
    pub _name: String,
    pub size: usize,
}

fn is_command(line: &str) -> bool {
    line.chars().next().unwrap() == '$'
}

fn process_lines(lines: &Vec<&str>) -> std::result::Result<Vec<LineType>, SimpleError> {
    let processed: std::result::Result<Vec<LineType>, SimpleError> = lines
        .iter()
        .map(|line| {
            if is_command(line) {
                if let Some(command) = Command::from_line(line) {
                    return Ok(LineType::Input(command));
                } else {
                    return Err(SimpleError::new(format!("Invalid command: {}", line)));
                }
            } else {
                if let Some(content) = Content::from_line(line) {
                    return Ok(LineType::Output(content));
                } else {
                    return Err(SimpleError::new(format!("Invalid output: {}", line)));
                }
            }
        })
        .collect();
    processed
}

const CUTOFF: usize = 100000;
fn accumulate_directory_sizes(
    node: &Directory,
    sizes: &mut Vec<usize>,
    cutoff: Option<usize>,
) -> usize {
    let this_size = node
        .files
        .iter()
        .map(|file| file.size)
        .reduce(|acc, x| acc + x)
        .unwrap_or(0);
    let sub_sizes = node
        .sub_directories
        .iter()
        .map(|subdir| accumulate_directory_sizes(subdir, sizes, cutoff))
        .reduce(|acc, x| acc + x)
        .unwrap_or(0);

    let combined_size = this_size + sub_sizes;
    if let Some(cutoff) = cutoff {
        if combined_size <= cutoff {
            sizes.push(combined_size);
        }
    } else {
        sizes.push(combined_size);
    }
    combined_size
}

impl Solution for Day07 {
    fn part1(&self) -> Result<()> {
        println!("Day 07 - Part 1");
        let input = std::fs::read_to_string("data/day07.txt")?;
        let lines: Vec<&str> = input.lines().collect();
        let processed = process_lines(&lines)?;

        let root = replay_commands(processed)?;

        let mut sizes: Vec<usize> = Vec::new();
        let _ = accumulate_directory_sizes(&root, &mut sizes, Some(CUTOFF));
        let combined_sizes = sizes.into_iter().reduce(|acc, x| acc + x).unwrap_or(0);
        println!("Total size: {}", combined_sizes);
        Ok(())
    }
    fn part2(&self) -> Result<()> {
        const DISK_SIZE: usize = 70000000;
        const REQUIRED_SIZE: usize = 30000000;
        println!("Day 07 - Part 2");
        let input = std::fs::read_to_string("data/day07.txt")?;
        let lines: Vec<&str> = input.lines().collect();
        let processed = process_lines(&lines)?;
        let root = replay_commands(processed)?;

        let mut sizes: Vec<usize> = Vec::new();
        let used_size = accumulate_directory_sizes(&root, &mut sizes, None);
        if (DISK_SIZE - used_size) > REQUIRED_SIZE {
            println!("Enough space available, no need to delete anything.");
            return Ok(());
        } else {
            let need_to_delete = REQUIRED_SIZE - (DISK_SIZE - used_size);
            println!("Need to delete at least: {}", need_to_delete);
            let mut large_enough_directories: Vec<usize> =
                sizes.into_iter().filter(|x| *x >= need_to_delete).collect();
            large_enough_directories.sort();
            println!(
                "Smallest size that can be deleted: {}",
                large_enough_directories.first().unwrap_or(&0)
            );
        }
        Ok(())
    }
}

fn replay_commands(processed: Vec<LineType>) -> Result<Directory> {
    let mut root = Directory::new("/");
    let mut cwd = &mut root;
    for line in processed {
        match line {
            LineType::Input(Command::ChangeDirectoryToRoot) => {
                cwd = &mut root;
            }
            LineType::Input(Command::ChangeDirectoryUp) => {
                if let Some(parent) = cwd.parent {
                    cwd = unsafe { &mut *parent };
                } else {
                    return Err(Box::new(SimpleError::new(format!(
                        "Current directory '{}' has no parent.",
                        cwd.name
                    ))));
                }
            }
            LineType::Input(Command::ChangeDirectoryDown(new_dir)) => {
                if let Some(subdir) = cwd
                    .sub_directories
                    .iter_mut()
                    .find(|subdir| subdir.name == new_dir)
                {
                    cwd = &mut *subdir;
                } else {
                    return Err(Box::new(SimpleError::new(format!(
                        "Current directory '{}' has no subdirectory '{}'.",
                        cwd.name, new_dir
                    ))));
                }
            }
            LineType::Input(Command::List) => {}
            LineType::Output(Content::Directory(name)) => {
                let new_directory = Box::new(Directory::new_with_parent(&name, cwd));
                cwd.sub_directories.push(new_directory);
            }
            LineType::Output(Content::File(name, size)) => {
                cwd.files.push(File {
                    _name: name,
                    size: size,
                });
            }
        }
    }
    Ok(root)
}
