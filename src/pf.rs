use std::collections::{BTreeSet, HashMap, HashSet};

pub fn a_star<S, N, H>(start: S, end: S, nexts: N, heur: H) -> Option<(Vec<S>, usize)>
where
    S: Clone + std::hash::Hash + PartialEq + Eq + PartialOrd + Ord,
    N: Fn(&S) -> HashSet<(S, usize)>,
    H: Fn(&S) -> usize,
{
    let mut prevs: HashMap<S, S> = HashMap::new();
    let mut dists: HashMap<S, usize> = HashMap::from([(start.clone(), 0)]);
    let mut to_visit: BTreeSet<(usize, S)> = BTreeSet::from([(0, start.clone())]);

    while let Some((_, mut curr)) = to_visit.pop_first() {
        if curr == end {
            let mut path = vec![curr.clone()];
            while curr != start {
                curr = prevs.get(&curr).unwrap().clone();
                path.push(curr.clone());
            }
            path.reverse();
            return Some((path, *dists.get(&end).unwrap()));
        }

        for (next, cost) in nexts(&curr) {
            let dist = cost + *dists.get(&curr).unwrap();

            if *dists.get(&next).unwrap_or(&usize::MAX) > dist {
                dists.insert(next.clone(), dist);
                prevs.insert(next.clone(), curr.clone());
                to_visit.insert((dist + heur(&next), next));
            }
        }
    }

    None
}

pub fn neighbours_usize(
    pos: &(usize, usize),
    n: Option<usize>,
    m: Option<usize>,
) -> Vec<(usize, usize)> {
    let mut ret = vec![];

    if let Some(up) = pos.0.checked_sub(1) {
        if n.map_or(true, |n| up < n) {
            ret.push((up, pos.1));
        }
    }

    if let Some(down) = pos.0.checked_add(1) {
        if n.map_or(true, |n| down < n) {
            ret.push((down, pos.1));
        }
    }

    if let Some(left) = pos.1.checked_sub(1) {
        if m.map_or(true, |m| left < m) {
            ret.push((pos.0, left));
        }
    }

    if let Some(right) = pos.1.checked_add(1) {
        if m.map_or(true, |m| right < m) {
            ret.push((pos.0, right));
        }
    }

    ret
}
