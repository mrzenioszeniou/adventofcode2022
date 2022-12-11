use std::str::FromStr;

use anyhow::{bail, Context};

use crate::Day;

#[derive(Clone, Debug)]
pub struct Day11 {
    monkeys: Vec<Monkey>,
}

impl Day11 {
    fn load(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            monkeys: std::fs::read_to_string(path)?
                .split("\n\n")
                .map(Monkey::from_str)
                .collect::<Result<_, _>>()?,
        })
    }

    fn part1(&mut self) -> u64 {
        let mut inspections = vec![0; self.monkeys.len()];

        for _ in 0..20 {
            (0..self.monkeys.len()).for_each(|i| {
                for mut item in std::mem::take(&mut self.monkeys[i].items) {
                    item.worry = self.monkeys[i].operation.apply(item.worry);
                    item.worry /= 3;

                    let throw_to = if item.worry % self.monkeys[i].divisor == 0 {
                        self.monkeys[i].throw_true
                    } else {
                        self.monkeys[i].throw_false
                    };

                    self.monkeys[throw_to].items.push(item);

                    inspections[i] += 1;
                }
            });
        }

        inspections.sort();

        inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
    }

    fn part2(&mut self) -> usize {
        let test_product = self.monkeys.iter().map(|m| m.divisor).product::<u64>();

        let mut inspections = vec![0; self.monkeys.len()];

        for _ in 0..10_000 {
            (0..self.monkeys.len()).for_each(|i| {
                for mut item in std::mem::take(&mut self.monkeys[i].items) {
                    item.worry = self.monkeys[i].operation.apply(item.worry);
                    item.worry %= test_product;

                    let throw_to = if item.worry % self.monkeys[i].divisor == 0 {
                        self.monkeys[i].throw_true
                    } else {
                        self.monkeys[i].throw_false
                    };

                    self.monkeys[throw_to].items.push(item);

                    inspections[i] += 1;
                }
            });
        }

        inspections.sort();

        inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
    }
}

impl Day for Day11 {
    const NAME: &'static str = "Day 11: Monkey in the Middle 🏈🐒";

    fn solve() -> anyhow::Result<(String, String)> {
        let mut day = Self::load("res/day11.txt")?;

        Ok((day.clone().part1().to_string(), day.part2().to_string()))
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    divisor: u64,
    throw_true: usize,
    throw_false: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id = sscanf::scanf!(
            lines.next().context("Not enough lined")?,
            "Monkey {}:",
            usize
        )
        .map_err(|e| anyhow::format_err!("Couldn't parse monkey id: {e}"))?;
        let items = parse_items(id, lines.next().context("Not enough lines")?)?;
        let operation = lines.next().context("Not enough lines")?.parse()?;
        let divisor = parse_divisor(lines.next().context("Not enough lines")?)?;
        let throw_true = parse_throw(lines.next().context("Not enough lines")?)?;
        let throw_false = parse_throw(lines.next().context("Not enough lines")?)?;

        Ok(Self {
            // id,
            items,
            operation,
            divisor,
            throw_true,
            throw_false,
        })
    }
}

#[derive(Clone, Debug)]
struct Item {
    worry: u64,
}

#[derive(Clone, Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Sqr,
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Self::Add(v) => old + v,
            Self::Mul(v) => old * v,
            Self::Sqr => old * old,
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .trim()
            .strip_prefix("Operation: new = ")
            .context("Unexpected operation line")?;

        Ok(if trimmed == "old * old" {
            Self::Sqr
        } else if let Some(v) = sscanf::scanf!(trimmed, "old + {}", u64).ok() {
            Self::Add(v)
        } else if let Some(v) = sscanf::scanf!(trimmed, "old * {}", u64).ok() {
            Self::Mul(v)
        } else {
            bail!("Couldn't parse '{s}' as operation")
        })
    }
}

fn parse_items(_initial_monkey: usize, s: &str) -> anyhow::Result<Vec<Item>> {
    Ok(s.trim()
        .strip_prefix("Starting items: ")
        .context("Unexpected item line")?
        .split(',')
        .map(|n| {
            n.trim().parse().map(|worry| Item {
                // initial_monkey,
                worry,
            })
        })
        .collect::<Result<_, _>>()?)
}

fn parse_divisor(s: &str) -> anyhow::Result<u64> {
    Ok(s.trim()
        .strip_prefix("Test: divisible by ")
        .context("Unexpected divisor line")?
        .parse()?)
}

fn parse_throw(s: &str) -> anyhow::Result<usize> {
    let trimmed = s.trim();

    if let Some(s) = trimmed.strip_prefix("If true: throw to monkey ") {
        Ok(s.parse()?)
    } else if let Some(s) = trimmed.strip_prefix("If false: throw to monkey ") {
        Ok(s.parse()?)
    } else {
        bail!("Unexpected throw line")
    }
}
