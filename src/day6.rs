use std::collections::HashSet;

use crate::Day;

pub struct Day6 {
    data: Vec<char>,
}

impl Day6 {
    fn load() -> anyhow::Result<Self> {
        let data = std::fs::read_to_string("res/day6.txt")?.chars().collect();

        Ok(Self { data })
    }

    fn part1(&self) -> anyhow::Result<usize> {
        for (idx, win) in self.data.windows(4).enumerate() {
            if win.iter().collect::<HashSet<_>>().len() == 4 {
                return Ok(idx + 4);
            }
        }

        anyhow::bail!("Couldn't find solution");
    }

    fn part2(&self) -> anyhow::Result<usize> {
        for (idx, win) in self.data.windows(14).enumerate() {
            if win.iter().collect::<HashSet<_>>().len() == 14 {
                return Ok(idx + 14);
            }
        }

        anyhow::bail!("Couldn't find solution");
    }
}

impl Day for Day6 {
    const NAME: &'static str = "Day 6: Tuning Trouble 📻 🪛";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load()?;

        Ok((day.part1()?.to_string(), day.part2()?.to_string()))
    }
}
