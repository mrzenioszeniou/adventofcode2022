use std::collections::VecDeque;

use anyhow::{bail, Context};

use crate::Day;

#[derive(Clone, Debug)]
pub struct Day5 {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl Day5 {
    #[allow(clippy::while_let_on_iterator)]
    fn load(file: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(file)?;
        let mut lines = content.split('\n');

        let mut stacks = vec![];
        let mut stack = VecDeque::default();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            for crt in line.split_ascii_whitespace() {
                if crt.len() != 1 {
                    bail!("Crates should have 1-char-long name");
                };
                let crt = crt.chars().next().unwrap();
                stack.push_front(crt);
            }

            stacks.push(std::mem::take(&mut stack));
        }

        let mut moves = vec![];
        while let Some(line) = lines.next() {
            let (n, from, to) = sscanf::scanf!(line, "move {} from {} to {}", usize, usize, usize)
                .map_err(|err| anyhow::Error::msg(err.to_string()))?;

            moves.push(Move {
                n,
                from: from - 1,
                to: to - 1,
            });
        }

        Ok(Self { stacks, moves })
    }

    fn part1(&mut self) -> anyhow::Result<String> {
        for (i, mov) in self.moves.iter().enumerate() {
            for _ in 0..mov.n {
                let crt = self.stacks[mov.from]
                    .pop_back()
                    .context(format!("Stack empty can't pop ({i}: {mov:?})"))?;

                self.stacks[mov.to].push_back(crt);
            }
        }

        Ok(self
            .stacks
            .iter()
            .map(|s| s.back().cloned().unwrap_or(' '))
            .collect())
    }

    fn part2(&mut self) -> anyhow::Result<String> {
        for mov in self.moves.iter() {
            let stack_size = self.stacks[mov.from].len();
            let mut crts = self.stacks[mov.from].split_off(stack_size - mov.n);
            self.stacks[mov.to].append(&mut crts);
        }

        Ok(self
            .stacks
            .iter()
            .map(|s| s.back().cloned().unwrap_or(' '))
            .collect())
    }
}

impl Day for Day5 {
    const NAME: &'static str = "Day 5: Supply Stacks 🏗️ 📦";

    fn solve() -> anyhow::Result<(String, String)> {
        let mut day = Self::load("res/day5.txt")?;

        Ok((day.clone().part1()?, day.part2()?))
    }
}

#[derive(Clone, Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}
