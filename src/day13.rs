use std::{cmp::Ordering, iter::Peekable, str::Chars};

use anyhow::{bail, Context};

use crate::Day;

pub struct Day13 {
    packets: Vec<Packet>,
}

impl Day13 {
    fn load(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            packets: std::fs::read_to_string(path)?
                .lines()
                .filter(|l| !l.is_empty())
                .map(|line| Packet::parse(&mut line.chars().peekable()))
                .collect::<anyhow::Result<Vec<_>>>()?,
        })
    }

    fn part1(&self) -> anyhow::Result<usize> {
        Ok(self
            .packets
            .chunks(2)
            .enumerate()
            .filter(|(_, pair)| pair[0] < pair[1])
            .map(|(i, _)| i + 1)
            .sum())
    }

    fn part2(&mut self) -> anyhow::Result<usize> {
        let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
        let divider_6 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

        self.packets.push(divider_2.clone());
        self.packets.push(divider_6.clone());

        self.packets.sort();

        let idx_2 = self
            .packets
            .iter()
            .position(|p| p == &divider_2)
            .context("where divider? 🦧")?
            + 1;

        let idx_6 = self
            .packets
            .iter()
            .position(|p| p == &divider_6)
            .context("where divider? 🦧")?
            + 1;

        Ok(idx_2 * idx_6)
    }
}

impl Day for Day13 {
    const NAME: &'static str = "Day 13: Distress Signal 🛟 📻";

    fn solve() -> anyhow::Result<(String, String)> {
        let mut day = Self::load("res/day13.txt")?;

        Ok((day.part1()?.to_string(), day.part2()?.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(s: &mut Peekable<Chars>) -> anyhow::Result<Self> {
        match s.next() {
            Some('[') => {
                let mut subpackets = vec![];

                while s.peek() != Some(&']') {
                    subpackets.push(Self::parse(s)?);
                    if s.peek() == Some(&',') {
                        let _ = s.next();
                    }
                }

                let _ = s.next();

                Ok(Self::List(subpackets))
            }
            Some(digit @ '0'..='9') => {
                let mut num_str = digit.to_string();
                while s.peek().map(|c| c.is_alphanumeric()).unwrap_or_default() {
                    num_str.push(s.next().context("Next should return a numeric")?);
                }
                Ok(Self::Int(
                    num_str.parse().context("Couldn't parse integer")?,
                ))
            }
            _ => bail!("Couldn't parse packet"),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(left), Packet::Int(right)) => left.cmp(right),
            (Packet::List(left), Packet::List(right)) => {
                if let Some(in_order) = left
                    .iter()
                    .zip(right.iter())
                    .map(|(l, r)| l.cmp(r))
                    .find(|o| o.is_ne())
                {
                    return in_order;
                }

                left.len().cmp(&right.len())
            }

            (Packet::Int(left), Packet::List(_)) => {
                Packet::List(vec![Packet::Int(*left)]).cmp(other)
            }
            (Packet::List(_), Packet::Int(right)) => {
                self.cmp(&Packet::List(vec![Packet::Int(*right)]))
            }
        }
    }
}
