use std::collections::HashSet;

use anyhow::{bail, Context};

use crate::{dir::Dir, Day};

pub struct Day9;

impl Day9 {
    fn both_parts(path: &str) -> anyhow::Result<(usize, usize)> {
        // parse
        let mut moves: Vec<(Dir, usize)> = vec![];
        for line in std::fs::read_to_string(path)?.lines() {
            let mut split = line.split_whitespace();

            moves.push((
                split
                    .next()
                    .and_then(|d| d.parse().ok())
                    .context("Couldn't parse direction")?,
                split
                    .next()
                    .and_then(|d| d.parse().ok())
                    .context("Couldn't parse # of steps")?,
            ));
        }

        // solve
        let mut knots = vec![(0, 0); 10];
        let mut visited_part1 = HashSet::from([(0, 0)]);
        let mut visited_part2 = HashSet::from([(0, 0)]);

        for (dir, steps) in moves {
            let step = dir.forward();
            for _ in 0..steps {
                knots[0].0 += step.0;
                knots[0].1 += step.1;

                for knot in 1..knots.len() {
                    let diff_0 = knots[knot - 1].0 - knots[knot].0;
                    let diff_1 = knots[knot - 1].1 - knots[knot].1;

                    match (diff_0.abs(), diff_1.abs()) {
                        (0, 0) | (0, 1) | (1, 0) | (1, 1) => {}

                        (2, 0) => knots[knot].0 += diff_0.signum(),
                        (0, 2) => knots[knot].1 += diff_1.signum(),

                        (2, 1) | (1, 2) | (2, 2) => {
                            knots[knot].0 += diff_0.signum();
                            knots[knot].1 += diff_1.signum();
                        }

                        diffs => bail!("Found {diffs:?} - this shouldn't be possible"),
                    }
                }

                visited_part1.insert(knots[1]);
                visited_part2.insert(knots[9]);
            }
        }

        Ok((visited_part1.len(), visited_part2.len()))
    }
}

impl Day for Day9 {
    const NAME: &'static str = "Day 9: Rope Bridge 🪢🪢🪢";

    fn solve() -> anyhow::Result<(String, String)> {
        let (part1, part2) = Self::both_parts("res/day9.txt")?;

        Ok((part1.to_string(), part2.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            Day9::both_parts("res/day9_example_small.txt").unwrap(),
            (13, 1)
        );
        assert_eq!(
            Day9::both_parts("res/day9_example_large.txt").unwrap().1,
            36
        );
    }
}
