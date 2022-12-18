use std::collections::{BTreeSet, HashSet};

use crate::Day;

#[derive(Debug)]
pub struct Day18 {
    points: HashSet<(isize, isize, isize)>,
    min: (isize, isize, isize),
    max: (isize, isize, isize),
}

impl Day18 {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut points = HashSet::new();

        let mut min = (isize::MAX, isize::MAX, isize::MAX);
        let mut max = (isize::MIN, isize::MIN, isize::MIN);

        for line in std::fs::read_to_string(path)?.lines() {
            let pos = sscanf::scanf!(line, "{},{},{}", isize, isize, isize)
                .map_err(|e| anyhow::Error::msg(e.to_string()))?;

            min.0 = std::cmp::min(min.0, pos.0);
            min.1 = std::cmp::min(min.1, pos.1);
            min.2 = std::cmp::min(min.2, pos.2);
            max.0 = std::cmp::max(max.0, pos.0);
            max.1 = std::cmp::max(max.1, pos.1);
            max.2 = std::cmp::max(max.2, pos.2);

            points.insert(pos);
        }

        Ok(Self { points, min, max })
    }

    fn part1(&self) -> anyhow::Result<usize> {
        let mut cnt = 0;

        for point in self.points.iter() {
            for neighbour in crate::pf::neighhbours_usize_3d(point) {
                if !self.points.contains(&neighbour) {
                    cnt += 1;
                }
            }
        }

        Ok(cnt)
    }

    fn part2(&self) -> anyhow::Result<usize> {
        let mut outside_points = HashSet::new();
        let mut to_visit = BTreeSet::from([self.max]);

        while let Some(point) = to_visit.pop_first() {
            for n in crate::pf::neighhbours_usize_3d(&point) {
                if !self.points.contains(&n)
                    && !outside_points.contains(&n)
                    && n.0 >= self.min.0 - 1
                    && n.0 <= self.max.0 + 1
                    && n.1 >= self.min.1 - 1
                    && n.1 <= self.max.1 + 1
                    && n.2 >= self.min.2 - 1
                    && n.2 <= self.max.2 + 1
                {
                    to_visit.insert(n);
                }
            }

            outside_points.insert(point);
        }

        let mut cnt = 0;
        for point in self.points.iter() {
            for n in crate::pf::neighhbours_usize_3d(point) {
                if outside_points.contains(&n) {
                    cnt += 1;
                }
            }
        }

        Ok(cnt)
    }
}

impl Day for Day18 {
    const NAME: &'static str = "Day 18: Boiling Boulders 🌋 💧";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load("res/day18.txt")?;

        Ok((day.part1()?.to_string(), day.part2()?.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::Day18;

    #[test]
    fn example() {
        let day = Day18::load("res/day18_example.txt").unwrap();

        assert_eq!(day.part1().unwrap(), 64);
        assert_eq!(day.part2().unwrap(), 58);
    }
}
