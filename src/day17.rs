use std::{
    collections::{BTreeSet, HashSet},
    time::Duration,
};

use anyhow::Context;

use crate::{dir::Dir, Day};

pub struct Day17 {
    jets: Vec<char>,
}

impl Day17 {
    fn load(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            jets: std::fs::read_to_string(path)?.chars().collect(),
        })
    }

    fn solve(&self, n_rocks: usize) -> anyhow::Result<usize> {
        // ↑
        // │
        // i  (i,j)
        // │
        // └──j───→
        let mut tiles: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut jet_idx = 0;

        for rock_id in 0..n_rocks {
            if rock_id % 1_000_000 == 0 {
                if let Some(edge) = Self::find_edge(&tiles) {
                    tiles = tiles
                        .into_iter()
                        .rev()
                        .take_while(|pos| pos.0 > edge)
                        .collect();

                    // println!("Pruned tiles down to {}:", tiles.len());

                    // if tiles.len() == 36 {
                    //     Self::_print(&Default::default(), &tiles);
                    // }
                }
            }

            let max_height = tiles.last().map(|(i, _)| *i + 1).unwrap_or_default();

            let mut rock = Self::new_rock(rock_id, max_height);

            loop {
                let jet_dir = match self.jets[jet_idx % self.jets.len()] {
                    '<' => Dir::West,
                    '>' => Dir::East,
                    jet => anyhow::bail!("Unexpected jet character '{jet}'"),
                };

                if let Some(shifted_rock) = Self::move_rock(&rock, &tiles, jet_dir) {
                    rock = shifted_rock;
                }

                jet_idx += 1;

                if let Some(shifted_rock) = Self::move_rock(&rock, &tiles, Dir::South) {
                    rock = shifted_rock;
                } else {
                    tiles.extend(rock);
                    break;
                }
            }
        }

        tiles
            .pop_last()
            .context("No tiles found")
            .map(|(i, _)| i + 1)
    }

    fn new_rock(id: usize, max_height: usize) -> HashSet<(usize, usize)> {
        match id % 5 {
            // ####
            0 => (2..6).map(|j| (max_height + 3, j)).collect(),

            //  #
            // ###
            //  #
            1 => HashSet::from([
                (max_height + 3, 3),
                (max_height + 4, 2),
                (max_height + 4, 3),
                (max_height + 4, 4),
                (max_height + 5, 3),
            ]),

            //   #
            //   #
            // ###
            2 => HashSet::from([
                (max_height + 3, 2),
                (max_height + 3, 3),
                (max_height + 3, 4),
                (max_height + 4, 4),
                (max_height + 5, 4),
            ]),

            // #
            // #
            // #
            // #
            3 => HashSet::from([
                (max_height + 3, 2),
                (max_height + 4, 2),
                (max_height + 5, 2),
                (max_height + 6, 2),
            ]),

            // ##
            // ##
            4 => HashSet::from([
                (max_height + 3, 2),
                (max_height + 3, 3),
                (max_height + 4, 2),
                (max_height + 4, 3),
            ]),

            id => panic!("Unexpected rock shape: {id}"),
        }
    }

    fn move_rock(
        rock: &HashSet<(usize, usize)>,
        tiles: &BTreeSet<(usize, usize)>,
        dir: Dir,
    ) -> Option<HashSet<(usize, usize)>> {
        let mut shifted_rock = HashSet::new();

        for rock_tile in rock {
            let shifted = match dir {
                Dir::North => (rock_tile.0 + 1, rock_tile.1),
                Dir::East => (
                    rock_tile.0,
                    if rock_tile.1 < 6 {
                        rock_tile.1 + 1
                    } else {
                        return None;
                    },
                ),
                Dir::South => (rock_tile.0.checked_sub(1)?, rock_tile.1),
                Dir::West => (rock_tile.0, rock_tile.1.checked_sub(1)?),
            };

            if tiles.contains(&shifted) {
                return None;
            } else {
                shifted_rock.insert(shifted);
            }
        }

        Some(shifted_rock)
    }

    fn _print(shape: &HashSet<(usize, usize)>, tiles: &BTreeSet<(usize, usize)>) {
        let max_i = std::cmp::max(
            tiles.last().map(|(i, _)| *i).unwrap_or_default(),
            shape.iter().map(|(i, _)| *i).max().unwrap_or_default(),
        );

        for i in (0..=max_i).rev().take(20) {
            print!("│");
            for j in 0..7 {
                if tiles.contains(&(i, j)) || shape.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("│");
        }
        println!("└───────┘\n");

        std::thread::sleep(Duration::from_millis(250));
    }

    fn find_edge(tiles: &BTreeSet<(usize, usize)>) -> Option<usize> {
        const MAX_FREE_SPACES: usize = 100;

        let (max_i, j) = tiles.last()?;

        let mut to_visit = BTreeSet::from([(max_i + 1, *j)]);
        let mut visited = BTreeSet::from([(max_i + 1, *j)]);

        while let Some(pos) = to_visit.pop_last() {
            for neighbour in crate::pf::neighbours_usize(&pos, Some(*max_i + 2), Some(7)) {
                if !tiles.contains(&neighbour) && !visited.contains(&neighbour) {
                    to_visit.insert(neighbour);
                }
            }

            visited.insert(pos);

            if visited.len() > MAX_FREE_SPACES {
                return None;
            }
        }

        let mut ret = [None; 7];

        while let Some(pos) = visited.pop_first() {
            if ret[pos.1].is_none() {
                ret[pos.1] = Some(pos.0);
                if ret.iter().all(Option::is_some) {
                    return ret.into_iter().flatten().min().map(|m| m - 2);
                }
            }
        }

        None
    }
}

impl Day for Day17 {
    const NAME: &'static str = "Day 17: Pyroclastic Flow 💨 🪨";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load("res/day17.txt")?;

        Ok((
            day.solve(2022)?.to_string(),
            day.solve(1_000_000_000_000)?.to_string(),
        ))
    }
}
