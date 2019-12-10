use arrayvec::ArrayString;
use std::collections::{HashMap, HashSet};

type OrbitNode = ArrayString<[u8; 4]>;
type OrbitGraph = HashMap<OrbitNode, HashSet<OrbitNode>>;

#[aoc_generator(day6)]
pub fn get_orbits(orbits: &str) -> OrbitGraph {
    let mut neighbours = HashMap::new();

    for (orbited, orbit) in orbits.lines().map(|x| {
        let mut foo = x.split(")");
        (foo.next().unwrap(), foo.next().unwrap())
    }) {
        let orbited = OrbitNode::from(orbited).unwrap();
        let orbit = OrbitNode::from(orbit).unwrap();
        neighbours
            .entry(orbited)
            .or_insert_with(HashSet::new)
            .insert(orbit);
        neighbours
            .entry(orbit)
            .or_insert_with(HashSet::new)
            .insert(orbited);
    }
    neighbours
}

fn get_lengths<'a, 'c: 'a>(
    graph: &'a OrbitGraph,
    start: &'c str,
    stop_at: Option<&'c str>,
) -> HashMap<OrbitNode, u64> {
    let mut lens = HashMap::with_capacity(graph.len());
    let mut current_set = HashSet::new();
    let start = OrbitNode::from(start).unwrap();
    let stop_at = stop_at.map(|s| OrbitNode::from(s).unwrap());
    current_set.insert(&start);
    lens.insert(start, 0);
    let mut curr_len = 0;

    let mut visited = HashSet::with_capacity(graph.len());

    loop {
        let mut new_set = HashSet::with_capacity(current_set.len());
        for item in current_set {
            if !visited.contains(&item) {
                if let Some(childs) = graph.get(item) {
                    for child in childs {
                        if !visited.contains(&item) {
                            new_set.insert(child);
                        }
                    }
                }
                visited.insert(item);
                lens.insert(*item, curr_len);
            }
        }
        if let Some(stop_at) = stop_at {
            if visited.contains(&&stop_at) {
                break;
            }
        }
        if new_set.is_empty() {
            break;
        }
        current_set = new_set;
        curr_len += 1;
    }
    lens
}

#[aoc(day6, part1)]
pub fn count_orbits(graph: &OrbitGraph) -> u64 {
    let lens = get_lengths(graph, "COM", None);
    lens.values().copied().sum()
}

#[aoc(day6, part2)]
pub fn hops_to_santa(graph: &OrbitGraph) -> u64 {
    let lens = get_lengths(graph, "YOU", Some("SAN"));
    lens.get("SAN").expect("santa not found") - 2
}
