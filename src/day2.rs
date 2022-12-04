use std::str::FromStr;

use anyhow::Context;
use strum::IntoEnumIterator;

use crate::Day;

pub struct Day2 {
    rounds: Vec<(Shape, RoundResult)>,
}

#[derive(Debug, PartialEq, Eq, strum::EnumIter)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn cmp(&self, other: &Self) -> RoundResult {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => RoundResult::Draw,
            (Shape::Rock, Shape::Paper) => RoundResult::Loss,
            (Shape::Rock, Shape::Scissors) => RoundResult::Win,
            (Shape::Paper, Shape::Rock) => RoundResult::Win,
            (Shape::Paper, Shape::Paper) => RoundResult::Draw,
            (Shape::Paper, Shape::Scissors) => RoundResult::Loss,
            (Shape::Scissors, Shape::Rock) => RoundResult::Loss,
            (Shape::Scissors, Shape::Paper) => RoundResult::Win,
            (Shape::Scissors, Shape::Scissors) => RoundResult::Draw,
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(anyhow::Error::msg(format!("'{s}' is not a valid shape"))),
        }
    }
}

#[derive(Debug, PartialEq)]
enum RoundResult {
    Loss,
    Draw,
    Win,
}

impl RoundResult {
    fn points(&self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    fn to_shape(&self) -> Shape {
        match self {
            Self::Loss => Shape::Rock,
            Self::Draw => Shape::Paper,
            Self::Win => Shape::Scissors,
        }
    }
}

impl FromStr for RoundResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(anyhow::Error::msg(format!("'{s}' is not a valid result"))),
        }
    }
}

impl Day2 {
    fn init() -> anyhow::Result<Self> {
        let mut rounds = vec![];

        for line in std::fs::read_to_string("res/day2.txt")?.split('\n') {
            let mut split = line.split_whitespace();

            let shape_left = split.next().context("No left sign")?.parse()?;
            let shape_right = split.next().context("No right sign")?.parse()?;

            rounds.push((shape_left, shape_right));
        }

        Ok(Self { rounds })
    }

    fn both(&self) -> anyhow::Result<(u32, u32)> {
        let mut part1 = 0;
        let mut part2 = 0;

        for (theirs, result) in self.rounds.iter() {
            part1 += result.to_shape().points() + result.to_shape().cmp(theirs).points();

            let ours = Shape::iter()
                .find(|ours| &ours.cmp(theirs) == result)
                .context(format!(
                    "Couldn't find shape to match {theirs:?} {result:?}"
                ))?;

            part2 += ours.points() + result.points();
        }

        Ok((part1, part2))
    }
}

impl Day for Day2 {
    const NAME: &'static str = "Day 2: Rock Paper Scissors ✊✋✌️";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::init()?;

        let (part1, part2) = day.both()?;

        Ok((part1.to_string(), part2.to_string()))
    }
}
