use std::collections::{BTreeMap, HashMap};

use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::Day;

pub struct Day19 {
    blueprints: Vec<HashMap<Resource, HashMap<Resource, u32>>>,
}

impl Day for Day19 {
    const NAME: &'static str = "Day 19: Not Enough Minerals 🤖 ⛏️ 🪨";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load("res/day19.txt")?;

        Ok((day.part1().to_string(), "bar".to_string()))
    }
}

impl Day19 {
    fn part1(&self) -> u32 {
        let mut sum = 0;

        for (idx, blueprint) in self.blueprints.iter().enumerate() {
            let mut cache = HashMap::new();

            let geodes = Self::naive(State::new(24), blueprint, &mut cache);

            println!("[{}] {blueprint:?}: {geodes} geodes", idx + 1);

            sum += (idx as u32 + 1) * geodes;
        }

        sum
    }

    fn load(path: &str) -> anyhow::Result<Self> {
        let mut blueprints = vec![];

        for line in std::fs::read_to_string(path)?.lines() {
            let (
                _,
                ore_to_ore,
                ore_to_clay,
                ore_to_obsidian,
                clay_to_obsidian,
                ore_to_geode,
                obsidian_to_geod
            ) = sscanf::sscanf!(
                line,
                "Blueprint {usize}: Each ore robot costs {u32} ore. Each clay robot costs {u32} ore. Each obsidian robot costs {u32} ore and {u32} clay. Each geode robot costs {u32} ore and {u32} obsidian."
            ).map_err(|e| anyhow::format_err!("{e}"))?;

            let blueprint = HashMap::from([
                (Resource::Ore, HashMap::from([(Resource::Ore, ore_to_ore)])),
                (
                    Resource::Clay,
                    HashMap::from([(Resource::Ore, ore_to_clay)]),
                ),
                (
                    Resource::Obsidian,
                    HashMap::from([
                        (Resource::Ore, ore_to_obsidian),
                        (Resource::Clay, clay_to_obsidian),
                    ]),
                ),
                (
                    Resource::Geode,
                    HashMap::from([
                        (Resource::Ore, ore_to_geode),
                        (Resource::Obsidian, obsidian_to_geod),
                    ]),
                ),
            ]);

            blueprints.push(blueprint);
        }

        Ok(Self { blueprints })
    }

    fn naive(
        state: State,
        blueprints: &HashMap<Resource, HashMap<Resource, u32>>,
        cache: &mut HashMap<State, u32>,
    ) -> u32 {
        if state.timeout == 0 {
            return state
                .stockpiled
                .get(&Resource::Geode)
                .cloned()
                .unwrap_or_default();
        } else if let Some(cached) = cache.get(&state) {
            // println!("Using cached solution for state {state:?}: {cached}");
            return *cached;
        }

        let mut max = 0;

        let mut hoard = false;

        // Try with increased production
        for resource in Resource::iter() {
            if let Some(without_extra_bot) = state.construct(resource, blueprints) {
                // Run production without the new bot
                let mut post_production = without_extra_bot.produce();
                // Add the new bot to the production
                *post_production.production.entry(resource).or_default() += 1;

                // Try substate
                max = std::cmp::max(max, Self::naive(post_production, blueprints, cache));
            } else if blueprints.get(&resource).unwrap().keys().all(|req| {
                state
                    .production
                    .get(req)
                    .map(|a| *a > 0)
                    .unwrap_or_default()
            }) {
                hoard = true;
            }
        }

        // Try being stingy if there was something we couldn't construct
        if hoard {
            max = std::cmp::max(max, Self::naive(state.produce(), blueprints, cache));
        }

        if let Some(cached) = cache.get_mut(&state) {
            if *cached < max {
                *cached = max;
            }
        } else {
            // println!("Solution for state {state:?}: {max}");
            cache.insert(state, max);
        }

        max
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    production: BTreeMap<Resource, u32>,
    stockpiled: BTreeMap<Resource, u32>,
    timeout: usize,
}

impl State {
    fn new(timeout: usize) -> Self {
        Self {
            production: BTreeMap::from([(Resource::Ore, 1)]),
            stockpiled: Default::default(),
            timeout,
        }
    }

    fn construct(
        &self,
        resource: Resource,
        blueprints: &HashMap<Resource, HashMap<Resource, u32>>,
    ) -> Option<Self> {
        let blueprint = blueprints.get(&resource)?;

        let mut next_stockpile = self.stockpiled.clone();

        for (req_resource, req_amount) in blueprint.iter() {
            next_stockpile.insert(
                *req_resource,
                self.stockpiled
                    .get(req_resource)
                    .cloned()
                    .unwrap_or_default()
                    .checked_sub(*req_amount)?,
            );
        }

        Some(Self {
            stockpiled: next_stockpile,
            production: self.production.clone(),
            timeout: self.timeout,
        })
    }

    fn produce(&self) -> Self {
        Self {
            production: self.production.clone(),
            stockpiled: self
                .production
                .iter()
                .map(|(res, amount)| {
                    (
                        *res,
                        *amount + self.stockpiled.get(res).cloned().unwrap_or_default(),
                    )
                })
                .collect(),
            timeout: self.timeout - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, EnumCount)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
