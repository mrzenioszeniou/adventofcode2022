use std::{collections::VecDeque, str::FromStr};

use anyhow::bail;

use crate::Day;

pub struct Day10 {
    reg_x: i32,
    active: Option<(usize, Op)>,
    cycle: usize,
    ops: VecDeque<Op>,
}

impl Day10 {
    fn load(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            reg_x: 1,
            active: None,
            cycle: 1,
            ops: std::fs::read_to_string(path)?
                .lines()
                .map(Op::from_str)
                .collect::<Result<VecDeque<_>, _>>()?,
        })
    }

    fn both_parts(&mut self) -> (i32, String) {
        let mut check = 20;
        let mut strength = 0;
        let mut screen = String::new();

        while self.active.is_some() || !self.ops.is_empty() {
            if self.active.is_none() {
                self.active = self.ops.pop_front().map(|op| (op.cycles(), op));
            }

            // part 1
            if check == self.cycle {
                strength += self.cycle as i32 * self.reg_x;
                check += 40;
            }

            // part 2
            let pixel = ((self.cycle - 1) % 40) as i32;
            if self.reg_x - 1 <= pixel && pixel <= self.reg_x + 1 {
                screen.push('#');
            } else {
                screen.push('.');
            }

            if self.cycle % 40 == 0 {
                screen.push('\n');
            }

            if let Some((rem_cycles, op)) = self.active.as_mut() {
                *rem_cycles -= 1;
                if *rem_cycles == 0 {
                    match op {
                        Op::Noop => {}
                        Op::Addx(v) => self.reg_x += *v,
                    }
                    self.active = None;
                }
            }

            self.cycle += 1;
        }

        (strength, screen)
    }
}

impl Day for Day10 {
    const NAME: &'static str = "Day 10: Cathode-Ray Tube 💡 📡";

    fn solve() -> anyhow::Result<(String, String)> {
        let mut day = Self::load("res/day10.txt")?;
        let (part1, part2) = day.both_parts();

        Ok((part1.to_string(), format!("\n{part2}")))
    }
}

enum Op {
    Noop,
    Addx(i32),
}

impl Op {
    fn cycles(&self) -> usize {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();

        if s == "noop" {
            Ok(Self::Noop)
        } else if let Some(v) = s
            .strip_prefix("addx ")
            .map(str::parse)
            .transpose()
            .ok()
            .flatten()
        {
            Ok(Self::Addx(v))
        } else {
            bail!("Couldn't parse '{s}' as an op");
        }
    }
}
