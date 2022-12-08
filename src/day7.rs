use std::collections::{HashMap, HashSet};

use anyhow::{bail, Context};

use crate::Day;

pub struct Day7 {
    directories: HashMap<String, Directory>,
}

impl Day7 {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let lines = std::fs::read_to_string(path)?
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let mut current_directory = vec![];
        let mut directories: HashMap<String, Directory> = HashMap::new();

        let mut curr_line = 0;

        while curr_line < lines.len() {
            if let Some(dir) = lines[curr_line].strip_prefix("$ cd ") {
                if dir == ".." {
                    current_directory.pop();
                } else {
                    current_directory.push(format!("{dir}/"));
                }
                curr_line += 1;
            } else if lines[curr_line] == "$ ls" {
                curr_line += 1;
                while curr_line < lines.len() {
                    if let Some((size, filename)) =
                        sscanf::scanf!(lines[curr_line], "{} {}", u32, String).ok()
                    {
                        directories
                            .entry(current_directory.concat())
                            .or_default()
                            .files
                            .insert(filename, size);
                        curr_line += 1;
                    } else if let Some(subdir) = lines[curr_line].strip_prefix("dir ") {
                        let subdir = format!("{}{subdir}/", current_directory.concat());

                        directories
                            .entry(current_directory.concat())
                            .or_default()
                            .subdirectories
                            .insert(subdir);
                        curr_line += 1;
                    } else {
                        break;
                    }
                }
            } else {
                bail!("Unexpected command found: '{}'", lines[0]);
            }
        }

        Ok(Self { directories })
    }

    fn directory_size(&self, directory: &str) -> u32 {
        let Some(directory) = self.directories.get(directory) else {
            return 0
        };

        let mut size = directory.files.values().sum();

        for subdir in directory.subdirectories.iter() {
            size += self.directory_size(subdir);
        }

        size
    }

    fn both(&self) -> anyhow::Result<(u32, u32)> {
        let min_delete = 30000000 - (70000000 - self.directory_size("//"));

        let mut part1 = 0;
        let mut part2 = None;

        for dir in self.directories.keys() {
            if dir != "//" {
                let size = self.directory_size(dir);

                if size <= 100_000 {
                    part1 += size;
                }

                if size >= min_delete && part2.map(|prev| prev > size).unwrap_or(true) {
                    part2 = Some(size);
                }
            }
        }

        Ok((
            part1,
            part2.context("Couldn't find directory at least {min_delete} large")?,
        ))
    }
}

impl Day for Day7 {
    const NAME: &'static str = "Day 7: No Space Left On Device 💾 🔋";

    fn solve() -> anyhow::Result<(String, String)> {
        let (part1, part2) = Self::load("res/day7.txt")?.both()?;
        Ok((part1.to_string(), part2.to_string()))
    }
}

#[derive(Default)]
struct Directory {
    files: HashMap<String, u32>,
    subdirectories: HashSet<String>,
}
