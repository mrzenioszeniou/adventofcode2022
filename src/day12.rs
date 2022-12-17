use std::collections::HashSet;

use anyhow::Context;

use crate::Day;

pub struct Day12 {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<char>>,
}

impl Day12 {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut start = None;
        let mut end = None;
        let mut map = vec![];

        for (i, line) in std::fs::read_to_string(path)?.lines().enumerate() {
            let mut row = vec![];

            for (j, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some((i, j));
                    row.push('a');
                } else if c == 'E' {
                    end = Some((i, j));
                    row.push('z');
                } else {
                    row.push(c);
                }
            }

            map.push(row);
        }

        Ok(Self {
            start: start.context("Couldn't find starting point")?,
            end: end.context("Couldn't find ending point")?,
            map,
        })
    }

    fn part1(&self) -> anyhow::Result<usize> {
        self.find_shortest_path(HashSet::from([self.start]))
            .context("Couldn't find path")
            .map(|(_, cost)| cost)
    }

    fn part2(&self) -> anyhow::Result<usize> {
        let mut starts = HashSet::new();

        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == 'a' {
                    starts.insert((i, j));
                }
            }
        }

        self.find_shortest_path(starts)
            .context("Couldn't find path")
            .map(|(_, len)| len)
    }

    fn find_shortest_path(
        &self,
        from: HashSet<(usize, usize)>,
    ) -> Option<(Vec<(usize, usize)>, usize)> {
        let heuristic =
            |curr: &(usize, usize)| self.end.0.abs_diff(curr.0) + self.end.1.abs_diff(curr.1);

        crate::pf::a_star(
            from,
            |pos| pos == &self.end,
            |&(i, j)| {
                let neighbours = crate::pf::neighbours_usize(
                    &(i, j),
                    Some(self.map.len()),
                    Some(self.map[0].len()),
                );

                let curr_height = self.map[i][j] as usize;
                let mut nexts: HashSet<((usize, usize), usize)> = HashSet::new();
                for (n_i, n_j) in neighbours.into_iter() {
                    let height: char = self.map[n_i][n_j];
                    if (height as usize).saturating_sub(curr_height) <= 1 {
                        nexts.insert(((n_i, n_j), 1));
                    }
                }

                nexts
            },
            heuristic,
        )
    }
}

impl Day for Day12 {
    const NAME: &'static str = "Day 12: Hill Climbing Algorithm 🥾🏔";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load("res/day12.txt")?;

        Ok((day.part1()?.to_string(), day.part2()?.to_string()))
    }
}
