use std::collections::HashSet;

use anyhow::{bail, Context};

use crate::Day;

pub struct Day3 {
    rucksacks: Vec<Vec<char>>,
}

impl Day3 {
    fn init() -> anyhow::Result<Self> {
        Ok(Self {
            rucksacks: std::fs::read_to_string("res/day3.txt")?
                .split_whitespace()
                .map(|line| line.to_string().chars().collect())
                .collect(),
        })
    }

    fn part1(&self) -> anyhow::Result<u32> {
        let mut priorities = 0;

        for rucksack in self.rucksacks.iter() {
            let mid = rucksack.len() / 2;

            let mut visited = HashSet::new();

            for (idx, item) in rucksack.iter().enumerate() {
                if idx < mid {
                    visited.insert(*item);
                } else if visited.contains(item) {
                    priorities += char_to_prio(*item)?;
                    break;
                }
            }
        }

        Ok(priorities)
    }

    fn part2(&self) -> anyhow::Result<u32> {
        let mut priorities = 0;

        for group in self.rucksacks.chunks(3) {
            let common_items = group
                .iter()
                .map(|r| r.iter().collect::<HashSet<_>>())
                .reduce(|acc, item| acc.intersection(&item).cloned().collect())
                .context("No common rucksack items")?;

            if common_items.len() != 1 {
                bail!("1 common item expected but found: {common_items:?}");
            }

            priorities += char_to_prio(*common_items.into_iter().next().unwrap())?;
        }

        Ok(priorities)
    }
}

impl Day for Day3 {
    const NAME: &'static str = "Day 3: Rucksack Reorganization 🎒 ♻️";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Day3::init()?;

        Ok((day.part1()?.to_string(), day.part2()?.to_string()))
    }
}

fn char_to_prio(c: char) -> anyhow::Result<u32> {
    match c {
        'a'..='z' => Ok(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(c as u32 - 'A' as u32 + 27),
        _ => anyhow::bail!("'{c}' is not a valid rucksack item"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_to_prio_test() {
        assert_eq!(char_to_prio('a').unwrap(), 1);
        assert_eq!(char_to_prio('c').unwrap(), 3);
        assert_eq!(char_to_prio('z').unwrap(), 26);
        assert_eq!(char_to_prio('A').unwrap(), 27);
        assert_eq!(char_to_prio('D').unwrap(), 30);
        assert_eq!(char_to_prio('Z').unwrap(), 52);
        assert!(char_to_prio('0').is_err());
        assert!(char_to_prio('5').is_err());
        assert!(char_to_prio('9').is_err());
    }

    #[test]
    fn example() {
        let day = Day3 {
            rucksacks: [
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]
            .into_iter()
            .map(|r| r.to_string().chars().collect())
            .collect(),
        };

        assert_eq!(day.part1().unwrap(), 157);
    }
}
