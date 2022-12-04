use crate::Day;

pub struct Day4 {
    sections: Vec<(Section, Section)>,
}

impl Day4 {
    fn load(file: &str) -> anyhow::Result<Self> {
        let mut sections = vec![];

        for line in std::fs::read_to_string(file)?.split_whitespace() {
            let (a, b, c, d) = sscanf::scanf!(line, "{}-{},{}-{}", u32, u32, u32, u32)
                .map_err(|err| anyhow::Error::msg(err.to_string()))?;

            let left = Section { start: a, end: b };

            let right = Section { start: c, end: d };

            sections.push((left, right));
        }

        Ok(Self { sections })
    }

    fn both(&self) -> anyhow::Result<(u32, u32)> {
        let mut n_contained = 0;
        let mut n_overlapping = 0;

        for (one, other) in self.sections.iter() {
            if one.contains(other) || other.contains(one) {
                n_contained += 1;
            }

            if !one.no_overlap(other) {
                n_overlapping += 1;
            }
        }

        Ok((n_contained, n_overlapping))
    }
}

impl Day for Day4 {
    const NAME: &'static str = "Day 4: Camp Cleanup ⛺️ 🧹";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Day4::load("res/day4.txt")?;

        let (part1, part2) = day.both()?;

        Ok((part1.to_string(), part2.to_string()))
    }
}

struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn no_overlap(&self, other: &Self) -> bool {
        self.end < other.start || self.start > other.end
    }
}
