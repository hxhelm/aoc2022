use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug)]
struct File {
    size: i32,
}

#[derive(Debug)]
struct FileSystem {
    cwd: String,
    files: HashMap<String, Vec<File>>,
}

impl FileSystem {
    fn cd(&mut self, dir: &str) {
        if dir == ".." {
            if self.cwd == "/" {
                return;
            }

            let mut vec = self.cwd.split('/').collect::<Vec<_>>();

            // remove last directory, keep / at end
            vec.remove(vec.len() - 2);

            self.cwd = vec.join("/");

            return;
        }

        if dir == "/" {
            self.cwd = dir.to_string();
            return;
        }

        self.cwd = [&self.cwd, dir, "/"].join("");
    }

    fn mkdir(&mut self, name: &str) {
        let directory_path = [&self.cwd, name, "/"].join("");

        if self.files.contains_key(&directory_path) {
            return;
        }

        self.files.insert(directory_path, vec![]);
    }

    fn touch(&mut self, filesize: i32) {
        let directory = self.files.get_mut(self.cwd.as_str()).unwrap();
        directory.push(File { size: filesize });
    }

    fn get_dir_size(&self, dir: &str) -> i32 {
        let mut size = 0;

        for (directory, files) in &self.files {
            if !directory.contains(dir) {
                continue;
            }

            size += files.iter().map(|file| file.size).sum::<i32>();
        }

        size
    }
}

#[derive(Debug)]
enum Instruction {
    ChangeDirectory(String),
    AddDirectory(String),
    AddFile(i32, String),
    Skip,
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    let (input, line_match) = alt((tag("$ "), tag("dir "), digit1))(input)?;

    let result = match line_match {
        "$ " => {
            if input == "ls" {
                Instruction::Skip
            } else {
                let dir = input.split(' ').last().unwrap();
                Instruction::ChangeDirectory(dir.to_string())
            }
        }
        "dir " => Instruction::AddDirectory(input.to_string()),
        _ => Instruction::AddFile(line_match.parse::<i32>().unwrap(), input.trim().to_string()),
    };
    Ok((input, result))
}

fn parse_instruction_lines(input: &str) -> (&str, Vec<Instruction>) {
    let instructions = input
        .lines()
        .map(|line| -> Instruction {
            let (_, instruction) = parse_line(line).unwrap();

            instruction
        })
        .collect();

    (input, instructions)
}

#[must_use]
pub fn part_one(input: &str) -> Option<i32> {
    let starting_working_directory = String::from("/");
    let mut files = HashMap::new();
    files.insert(starting_working_directory.clone(), vec![]);

    let mut filesystem = FileSystem {
        cwd: starting_working_directory,
        files,
    };

    let (_, lines) = parse_instruction_lines(input);

    for line in lines {
        match line {
            Instruction::ChangeDirectory(cmd) => filesystem.cd(&cmd),
            Instruction::AddDirectory(dir) => filesystem.mkdir(&dir),
            Instruction::AddFile(size, _) => filesystem.touch(size),
            Instruction::Skip => continue,
        };
    }

    let mut total_size = 0;
    for directory in filesystem.files.keys() {
        let dirsize = &filesystem.get_dir_size(directory);

        if *dirsize < 100_000 {
            total_size += *dirsize;
        }
    }

    Some(total_size)
}

#[must_use]
pub fn part_two(input: &str) -> Option<i32> {
    let starting_working_directory = String::from("/");
    let mut files = HashMap::new();
    files.insert(starting_working_directory.clone(), vec![]);

    let mut filesystem = FileSystem {
        cwd: starting_working_directory,
        files,
    };

    let (_, lines) = parse_instruction_lines(input);

    for line in lines {
        match line {
            Instruction::ChangeDirectory(cmd) => filesystem.cd(&cmd),
            Instruction::AddDirectory(dir) => filesystem.mkdir(&dir),
            Instruction::AddFile(size, _) => filesystem.touch(size),
            Instruction::Skip => continue,
        };
    }

    let total_space = 70_000_000;
    let needed_for_update = 30_000_000;

    let used = filesystem.get_dir_size("/");
    let available = total_space - used;
    let need_to_delete = needed_for_update - available;

    let mut current_dirsize = used;

    for directory in filesystem.files.keys() {
        let &dirsize = &filesystem.get_dir_size(directory);

        if dirsize < need_to_delete || dirsize > current_dirsize {
            continue;
        }

        current_dirsize = dirsize;
    }

    Some(current_dirsize)
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
        assert_eq!(part_two(&input), Some(24933642));
    }
}
