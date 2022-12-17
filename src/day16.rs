use std::collections::{BTreeSet, HashMap, HashSet};

use anyhow::Context;

use crate::{pf, Day};

#[derive(Debug)]
pub struct Day16 {
    valves: HashMap<String, Valve>,
}

impl Day16 {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut valves = HashMap::new();

        for (line_idx, line) in std::fs::read_to_string(path)?.lines().enumerate() {
            let (valve_id, rate, _, _, _, tunnels) = sscanf::scanf!(
                line,
                "Valve {} has flow rate={}; tunnel{:/[s]?/} lead{:/[s]?/} to valve{:/[s]?/} {}",
                String,
                usize,
                String,
                String,
                String,
                String,
            )
            .map_err(|e| anyhow::format_err!("Line {}: {e}", line_idx + 1))?;

            let tunnels = tunnels.split(", ").map(|s| s.to_string()).collect();

            valves.insert(valve_id, Valve { tunnels, rate });
        }

        Ok(Self { valves })
    }

    fn part1(&self, elephant: bool) -> anyhow::Result<usize> {
        // println!("Idle Rate = {total_idle_rate}");

        let heur = |s: &State| {
            let mut estimation = 0;
            let mut closed_valves: BTreeSet<usize> = s
                .closed_valves
                .iter()
                .filter_map(|v| self.valves.get(v))
                .map(|v| v.rate)
                .filter(|rate| *rate > 0)
                .collect();

            for _ in 0..s.timer {
                if closed_valves.is_empty() {
                    break;
                }

                estimation += closed_valves.iter().sum::<usize>();

                closed_valves.pop_last();
                if elephant {
                    closed_valves.pop_last();
                }
            }

            // println!("  heur = {estimation}");

            estimation
        };

        let nexts = |s: &State| {
            let pressure_buildup = s
                .closed_valves
                .iter()
                .filter_map(|v| self.valves.get(v))
                .map(|v| v.rate)
                .sum::<usize>();
            let timer = s.timer - 1;

            let mut nexts = vec![];

            if pressure_buildup == 0 {
                return HashSet::from([(
                    State {
                        curr_valve: s.curr_valve.clone(),
                        elephant_valve: s.elephant_valve.clone(),
                        closed_valves: s.closed_valves.clone(),
                        timer,
                    },
                    pressure_buildup,
                )]);
            }

            if s.closed_valves.contains(&s.curr_valve)
                && self.valves.get(&s.curr_valve).unwrap().rate > 0
            {
                nexts.push(State {
                    curr_valve: s.curr_valve.clone(),
                    elephant_valve: s.elephant_valve.clone(),
                    closed_valves: {
                        let mut valves = s.closed_valves.clone();
                        valves.remove(&s.curr_valve);
                        valves
                    },
                    timer,
                });
            }

            for tunnel in self.valves.get(&s.curr_valve).unwrap().tunnels.iter() {
                nexts.push(State {
                    curr_valve: tunnel.clone(),
                    elephant_valve: s.elephant_valve.clone(),
                    closed_valves: s.closed_valves.clone(),
                    timer,
                });
            }

            if elephant {
                let mut elephant_nexts = vec![];

                if s.closed_valves.contains(&s.elephant_valve)
                    && self.valves.get(&s.elephant_valve).unwrap().rate > 0
                {
                    elephant_nexts.push(State {
                        curr_valve: s.curr_valve.clone(),
                        elephant_valve: s.elephant_valve.clone(),
                        closed_valves: {
                            let mut valves = s.closed_valves.clone();
                            valves.remove(&s.elephant_valve);
                            valves
                        },
                        timer,
                    });
                }

                for tunnel in self.valves.get(&s.elephant_valve).unwrap().tunnels.iter() {
                    elephant_nexts.push(State {
                        curr_valve: s.curr_valve.clone(),
                        elephant_valve: tunnel.clone(),
                        closed_valves: s.closed_valves.clone(),
                        timer,
                    });
                }

                let mut combined_nexts = HashSet::new();

                for our_next in nexts.iter() {
                    for elephant_next in elephant_nexts.iter() {
                        if s.curr_valve == our_next.curr_valve
                            && s.elephant_valve == elephant_next.elephant_valve
                            && s.curr_valve == s.elephant_valve
                        {
                            continue;
                        }

                        combined_nexts.insert((
                            State {
                                curr_valve: our_next.curr_valve.clone(),
                                elephant_valve: elephant_next.elephant_valve.clone(),
                                closed_valves: {
                                    let mut valves = our_next.closed_valves.clone();
                                    valves.retain(|v| elephant_next.closed_valves.contains(v));
                                    valves
                                },
                                timer,
                            },
                            pressure_buildup,
                        ));
                    }
                }

                combined_nexts
            } else {
                nexts.into_iter().map(|s| (s, pressure_buildup)).collect()
            }

            // println!("{:?}", s);
            // for (next, pressure) in nexts.iter() {
            //     println!("  {next:?} +{pressure} pressure");
            // }
        };

        let (states, _) = pf::a_star(
            HashSet::from([State {
                closed_valves: self.valves.keys().cloned().collect(),
                curr_valve: "AA".to_string(),
                elephant_valve: "AA".to_string(),
                timer: if elephant { 26 } else { 30 },
            }]),
            |s| s.timer == 0,
            nexts,
            heur,
        )
        .context(format!(
            "no solution found {} elephant",
            if elephant { "with" } else { "without" }
        ))?;

        let mut total_released_pressure = 0;

        for (_i, state) in states
            .into_iter()
            .enumerate()
            .take(if elephant { 26 } else { 30 })
        {
            let released_pressure = self
                .valves
                .iter()
                .filter(|(v_id, _)| !state.closed_valves.contains(*v_id))
                .map(|(_, v)| v.rate)
                .sum::<usize>();

            // println!("[{_i}] {released_pressure} pressure released with {state:?}");

            total_released_pressure += released_pressure;
        }

        Ok(total_released_pressure)
    }
}

impl Day for Day16 {
    const NAME: &'static str = "Day 16: Proboscidea Volcanium 🐘 🌋";

    fn solve() -> anyhow::Result<(String, String)> {
        let day = Self::load("res/day16.txt")?;

        // println!("{day:#?}");

        Ok((day.part1(false)?.to_string(), day.part1(true)?.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Valve {
    rate: usize,
    tunnels: HashSet<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    curr_valve: String,
    elephant_valve: String,
    closed_valves: BTreeSet<String>,
    timer: usize,
}
