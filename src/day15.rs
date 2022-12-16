use std::collections::{HashMap, HashSet};

use anyhow::bail;

use crate::Day;

#[derive(Debug)]
pub struct Day15 {
    readings: HashMap<(isize, isize), usize>,
    beacons: HashSet<(isize, isize)>,
}

impl Day15 {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut beacons = HashSet::new();
        let mut readings = HashMap::new();

        for line in std::fs::read_to_string(path)?.lines() {
            let (a, b, c, d) = sscanf::sscanf!(
                line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                isize,
                isize,
                isize,
                isize,
            )
            .map_err(|err| anyhow::format_err!("Couldn't parse line: {err}"))?;

            readings.insert((b, a), b.abs_diff(d) + c.abs_diff(a));
            beacons.insert((d, c));
        }

        Ok(Self { beacons, readings })
    }

    fn line_exclusion(&self, line: isize) -> Vec<(isize, isize)> {
        let mut ranges = vec![];

        for (sensor, radius) in self.readings.iter() {
            let Some(remaining_radius) = radius
                .checked_sub(sensor.0.abs_diff(line)) else {

                    continue;
                };

            let mut new_ranges = vec![(
                sensor.1 - remaining_radius as isize,
                sensor.1 + remaining_radius as isize,
            )];

            for range in self
                .beacons
                .iter()
                .chain(self.readings.keys())
                .filter(|b| b.0 == line)
                .map(|(_, j)| (*j, *j))
                .chain(ranges.iter().cloned())
            {
                let mut i = 0;
                while i < new_ranges.len() {
                    let subranges = subtract_range(new_ranges[i], range);

                    if subranges == new_ranges[i..i + 1] {
                        i += 1;
                    } else {
                        new_ranges.remove(i);
                        new_ranges.extend(subranges);
                    }
                }
            }

            ranges.extend(new_ranges.into_iter());
        }

        ranges.sort();

        ranges
    }

    fn part1(&self, line: isize) -> isize {
        self.line_exclusion(line)
            .iter()
            .map(|(min, max)| max - min + 1)
            .sum()
    }

    // FIXME: slow
    fn part2(&self, max: isize) -> anyhow::Result<isize> {
        for line in 0..=max {
            let ranges = self.line_exclusion(line);

            for pair in ranges.windows(2) {
                if pair[0].1 + 2 == pair[1].0
                    && !self.readings.contains_key(&(line, pair[0].1 + 1))
                    && !self.beacons.contains(&(line, pair[0].1 + 1))
                {
                    return Ok((pair[0].1 + 1) * 4000000 + line);
                }
            }
        }

        bail!("No gap was found")
    }
}

impl Day for Day15 {
    const NAME: &'static str = "Day 15: Beacon Exclusion Zone 📡 🚫";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load("res/day15.txt")?;

        Ok((
            day.part1(2_000_000).to_string(),
            day.part2(4_000_000)?.to_string(),
        ))
    }
}

fn subtract_range(
    (mut min_j, mut max_j): (isize, isize),
    (other_min_j, other_max_j): (isize, isize),
) -> Vec<(isize, isize)> {
    let mut subranges = vec![];

    if min_j <= other_min_j && other_max_j <= max_j {
        subranges.push((min_j, other_min_j - 1));
        subranges.push((other_max_j + 1, max_j));
    } else {
        if other_min_j <= min_j && min_j <= other_max_j {
            min_j = other_max_j + 1;
        }

        if other_min_j <= max_j && max_j <= other_max_j {
            max_j = other_min_j - 1
        }

        subranges.push((min_j, max_j));
    }

    subranges.retain(|(min, max)| min <= max);

    subranges
}
