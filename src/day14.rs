use std::collections::HashSet;

use anyhow::Context;

use crate::Day;

#[derive(Clone)]
/// FIXME: slow
pub struct Day14 {
    cave: HashSet<(usize, usize)>,
    max_i: usize,
    max_j: usize,
    min_j: usize,
}

impl Day14 {
    fn _print_cave(&self) {
        for i in 0..=self.max_i + 2 {
            for j in self.min_j - 10..=self.max_j + 10 {
                if i == 0 && j == 500 {
                    print!("+");
                } else if self.cave.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!()
    }

    fn load(path: &str) -> anyhow::Result<Self> {
        let mut cave = HashSet::new();
        let mut max_i = 0;
        let mut max_j = 0;
        let mut min_j = usize::MAX;

        for line in std::fs::read_to_string(path)?.lines() {
            let mut point = None;
            for point_str in line.split(" -> ") {
                let mut split = point_str.split(',');
                let j = split.next().context("Unexpected point format")?.parse()?;
                let i = split.next().context("Unexpected point format")?.parse()?;

                if let Some((prev_i, prev_j)) = point {
                    if prev_i == i {
                        let start_j = std::cmp::min(prev_j, j);
                        let end_j = std::cmp::max(prev_j, j);
                        for j in start_j..=end_j {
                            cave.insert((i, j));
                        }
                    } else {
                        let start_i = std::cmp::min(prev_i, i);
                        let end_i = std::cmp::max(prev_i, i);
                        for i in start_i..=end_i {
                            cave.insert((i, j));
                        }
                    }
                }

                max_i = std::cmp::max(max_i, i);
                max_j = std::cmp::max(max_j, j);
                min_j = std::cmp::min(min_j, j);
                point = Some((i, j));
            }
        }

        Ok(Self {
            cave,
            max_i,
            max_j,
            min_j,
        })
    }

    fn part1(&mut self) -> usize {
        'sands: for sand in 0.. {
            let mut i = 0;
            let mut j = 500;

            loop {
                let below = i + 1;

                if !self.cave.contains(&(below, j)) {
                    // DOWN
                    i = below;
                } else if !self.cave.contains(&(below, j - 1)) {
                    // LEFT
                    i = below;
                    j -= 1;
                } else if !self.cave.contains(&(below, j + 1)) {
                    // RIGHT
                    i = below;
                    j += 1;
                } else {
                    self.cave.insert((i, j));
                    continue 'sands;
                }

                if j < self.min_j || j > self.max_j {
                    return sand;
                }
            }
        }

        unreachable!("wat");
    }

    fn part2(&mut self) -> usize {
        'sands: for sand in 0.. {
            let mut i = 0;
            let mut j = 500;

            if self.cave.contains(&(i, j)) {
                return sand;
            }

            loop {
                let below = i + 1;

                if !self.cave.contains(&(below, j)) {
                    // DOWN
                    i = below;
                } else if !self.cave.contains(&(below, j - 1)) {
                    // LEFT
                    i = below;
                    j -= 1;
                } else if !self.cave.contains(&(below, j + 1)) {
                    // RIGHT
                    i = below;
                    j += 1;
                } else {
                    self.cave.insert((i, j));
                    continue 'sands;
                }

                if i == self.max_i + 1 {
                    self.cave.insert((i, j));
                    continue 'sands;
                }
            }
        }

        unreachable!("wat");
    }
}

impl Day for Day14 {
    const NAME: &'static str = "Day 14: Regolith Reservoir ⏳ ⚠️";

    fn solve() -> anyhow::Result<(String, String)> {
        let mut day = Self::load("res/day14.txt")?;

        Ok((day.clone().part1().to_string(), day.part2().to_string()))
    }
}
