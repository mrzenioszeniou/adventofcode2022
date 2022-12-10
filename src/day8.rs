use std::collections::HashSet;

use anyhow::Context;

use crate::Day;

pub struct Day8 {
    trees: Vec<Vec<u8>>,
}

impl Day8 {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut trees = vec![];

        for line in std::fs::read_to_string(path)?.lines() {
            let mut tree_line = vec![];
            for c in line.chars() {
                tree_line.push(c.to_digit(10).context("Not a base 10 digit")? as u8);
            }

            trees.push(tree_line);
        }

        Ok(Self { trees })
    }

    fn part1(&self) -> usize {
        let mut visible_trees = vec![];

        for line in 0..self.trees.len() {
            let line_iterator = (0..self.trees[line].len()).map(|column| (line, column));
            visible_trees.append(&mut self.part1_line(line_iterator.clone()));
            visible_trees.append(&mut self.part1_line(line_iterator.rev()));
        }

        for column in 0..self.trees[0].len() {
            let column_iterator = (0..self.trees.len()).map(|line| (line, column));
            visible_trees.append(&mut self.part1_line(column_iterator.clone()));
            visible_trees.append(&mut self.part1_line(column_iterator.rev()));
        }

        visible_trees.into_iter().collect::<HashSet<_>>().len()
    }

    fn part2(&self) -> usize {
        let mut best = 0;
        let n = self.trees.len();
        let m = self.trees[0].len();
        for curr_i in 0..n {
            for curr_j in 0..m {
                let curr_height = self.trees[curr_i][curr_j];

                // north
                let north = self.part2_line(
                    (0..curr_i).rev().zip(std::iter::repeat(curr_j)),
                    curr_height,
                );

                // south
                let south =
                    self.part2_line((curr_i + 1..n).zip(std::iter::repeat(curr_j)), curr_height);

                // west
                let west = self.part2_line(
                    (0..curr_j)
                        .rev()
                        .zip(std::iter::repeat(curr_i))
                        .map(|(j, i)| (i, j)),
                    curr_height,
                );

                // east
                let east =
                    self.part2_line(std::iter::repeat(curr_i).zip(curr_j + 1..m), curr_height);

                let scenic_score = north * south * west * east;

                best = std::cmp::max(scenic_score, best);
            }
        }

        best
    }

    fn part1_line(&self, trees: impl Iterator<Item = (usize, usize)>) -> Vec<(usize, usize)> {
        let mut visible_trees = vec![];
        let mut max = None;

        for (i, j) in trees {
            let tree_height = self.trees[i][j];

            if max.map_or(true, |max| self.trees[i][j] > max) {
                visible_trees.push((i, j));
                max = Some(tree_height);
            }
        }

        visible_trees
    }

    fn part2_line(&self, trees: impl Iterator<Item = (usize, usize)>, height: u8) -> usize {
        let mut cnt = 0;

        for (i, j) in trees {
            cnt += 1;

            if self.trees[i][j] >= height {
                break;
            }
        }

        cnt
    }
}

impl Day for Day8 {
    const NAME: &'static str = "Day 8: Treetop Tree House 🌳🏡🌳";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load("res/day8.txt")?;

        Ok((day.part1().to_string(), day.part2().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let day = Day8::load("res/day8_example.txt").unwrap();

        assert_eq!(day.part1(), 21);
        assert_eq!(day.part2(), 8);
    }
}
