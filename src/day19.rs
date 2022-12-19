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

        Ok((day.part1().to_string(), day.part2().to_string()))
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

    fn part2(&self) -> u32 {
        let mut max = 0;

        for (idx, blueprint) in self.blueprints.iter().take(3).enumerate() {
            let mut cache = HashMap::new();

            let geodes = Self::naive(State::new(32), blueprint, &mut cache);

            println!("[{}] {blueprint:?}: {geodes} geodes", idx + 1);

            max = std::cmp::max(max, geodes);
        }

        max
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

        // We have 5 eventualities
        // we'll wait until we can construct an ore bot
        // we'll wait until we can construct a clay bot
        // we'll wait until we can construct an obsidian bot
        // we'll wait until we can construct a geode bot
        // none of the above are possible because we'd run out the timer, so we just produce until
        // the timer's up

        let mut hoard = true;

        // Try with increasing production
        for resource in Resource::iter() {
            if let Some(with_new_bot) = state.construct(resource, blueprints) {
                // Try substate
                max = std::cmp::max(max, Self::naive(with_new_bot, blueprints, cache));

                // We have managed to produce something, so no need to run out the clock
                hoard = false;
            }
        }

        // If there's nothing we can construct with the remaining time, run out the clock
        if hoard {
            max = std::cmp::max(
                max,
                state
                    .produce(state.timeout)
                    .stockpiled
                    .get(&Resource::Geode)
                    .cloned()
                    .unwrap_or_default(),
            );
        }

        // Cache results
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
    timeout: u32,
}

impl State {
    fn new(timeout: u32) -> Self {
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

        let mut required_minutes = 0;

        for (req_resource, req_amount) in blueprint.iter() {
            let missing = req_amount.saturating_sub(
                self.stockpiled
                    .get(req_resource)
                    .cloned()
                    .unwrap_or_default(),
            );

            if missing == 0 {
                continue;
            }

            let production = self
                .production
                .get(req_resource)
                .cloned()
                .unwrap_or_default();

            if production > 0 {
                required_minutes = std::cmp::max(required_minutes, missing.div_ceil(production));
            } else {
                return None;
            }
        }

        if required_minutes >= self.timeout {
            None
        } else {
            let mut ret = self.produce(required_minutes + 1);

            blueprint.iter().for_each(|(req_res, req_cnt)| {
                *ret.stockpiled.get_mut(req_res).unwrap() -= *req_cnt
            });

            *ret.production.entry(resource).or_default() += 1;

            Some(ret)
        }
    }

    fn produce(&self, secs: u32) -> Self {
        Self {
            production: self.production.clone(),
            stockpiled: self
                .production
                .iter()
                .map(|(res, amount)| {
                    (
                        *res,
                        *amount * secs + self.stockpiled.get(res).cloned().unwrap_or_default(),
                    )
                })
                .collect(),
            timeout: self.timeout - secs,
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
