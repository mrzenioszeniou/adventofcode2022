use anyhow::Context;

use crate::Day;

pub struct Day1 {
    calories_per_reindeer: Vec<Vec<u32>>,
}

impl Day1 {
    pub fn init() -> anyhow::Result<Self> {
        let mut calories_per_reindeer = vec![];
        let mut reindeer = vec![];
        for line in std::fs::read_to_string("res/day1.txt")?.split('\n') {
            if line.is_empty() {
                calories_per_reindeer.push(std::mem::take(&mut reindeer));
            } else {
                reindeer.push(line.parse::<u32>()?);
            }
        }

        calories_per_reindeer.sort_by_key(|r| r.iter().sum::<u32>());

        Ok(Self {
            calories_per_reindeer,
        })
    }

    pub fn part1(&self) -> anyhow::Result<u32> {
        self.calories_per_reindeer
            .iter()
            .map(|r| r.iter().sum())
            .max()
            .context("No reindeers?")
    }

    pub fn part2(&self) -> anyhow::Result<u32> {
        Ok(self
            .calories_per_reindeer
            .iter()
            .rev()
            .take(3)
            .map(|r| r.iter().sum::<u32>())
            .sum())
    }
}

impl Day for Day1 {
    const NAME: &'static str = "Day 1: Calorie Counting";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::init()?;
        Ok((day.part1()?.to_string(), day.part2()?.to_string()))
    }
}
