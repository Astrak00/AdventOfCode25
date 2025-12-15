use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Id([u8; 3]);

type NodeList = HashMap<Id, Vec<Id>>;

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Convert the byte array to a string slice
        let s = std::str::from_utf8(&self.0).unwrap_or("???");
        write!(f, "{}", s)
    }
}

fn parse_input(input: &str) -> NodeList {
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let id_str = parts[0];
        let connections_str = parts[1];
        let id = Id([
            id_str.as_bytes()[0],
            id_str.as_bytes()[1],
            id_str.as_bytes()[2],
        ]);
        let connections: Vec<Id> = connections_str
            .split(" ")
            .map(|conn| {
                let bytes = conn.as_bytes();
                Id([bytes[0], bytes[1], bytes[2]])
            })
            .collect();
        nodes.insert(id, connections);
    }
    nodes
}

fn part1(nodes: NodeList, visited: &mut HashMap<Id, bool>, current: Id) -> i32 {
    if current == Id(*b"out") {
        // println!("Reached 'out' node!");
        return 1;
    }
    let mut total_paths = 0;
    if let Some(neighbors) = nodes.get(&current) {
        for neighbor in neighbors {
            if !visited.get(neighbor).unwrap_or(&false) {
                visited.insert(*neighbor, true);
                let count = part1(nodes.clone(), visited, *neighbor);
                total_paths += count;
                visited.insert(*neighbor, false);
            }
        }
    }
    total_paths
}

fn part2_general(
    nodes: &NodeList,
    visited: &mut HashMap<Id, bool>,
    current: Id,
    target: Id,
) -> i32 {
    if current == target {
        return 1;
    }
    let mut total_paths = 0;
    if let Some(neighbors) = nodes.get(&current) {
        for neighbor in neighbors {
            if !visited.get(neighbor).unwrap_or(&false) {
                visited.insert(*neighbor, true);
                let count = part2_general(nodes, visited, *neighbor, target);
                total_paths += count;
                visited.insert(*neighbor, false);
            }
        }
    }
    total_paths
}

fn part2_from_py(
    start: Id,
    end: Id,
    visited: &mut HashMap<Id, bool>,
    distances_mem: &mut HashMap<Id, u128>,
    graph: &NodeList,
) -> u128 {
    if start == end {
        return 1;
    }
    if visited.get(&start).unwrap_or(&false) == &true || start == Id(*b"out") {
        return 0;
    }
    if distances_mem.contains_key(&start) {
        return *distances_mem.get(&start).unwrap();
    }
    visited.insert(start, true);
    //     total = sum([traverse(output, end, visited, scores) for output in graph[start]])
    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(&start) {
        for neighbor in neighbors {
            let count = part2_from_py(*neighbor, end, visited, distances_mem, graph);
            total_paths += count;
        }
    }
    visited.remove_entry(&start);
    distances_mem.insert(start, total_paths);
    total_paths
}

fn part2_destinations(nodes: &NodeList, order: Vec<Id>) -> u128 {
    let mut result_2_gen = 1;

    use rayon::prelude::*;

    let results: Vec<u128> = (0..(order.len() - 1))
        .into_par_iter()
        .map(|ord_idx| {
            part2_from_py(
                order[ord_idx],
                order[ord_idx + 1],
                &mut HashMap::new(),
                &mut HashMap::new(),
                nodes,
            )
        })
        .collect();

    for result_2 in &results {
        if *result_2 == 0 {
            return 0;
        } else {
            result_2_gen *= *result_2;
        }
    }

    result_2_gen
}
fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let nodes = parse_input(&input);
    let part1_result = part1(nodes.clone(), &mut HashMap::new(), Id(*b"you"));
    println!("Part 1: {}", part1_result);

    let input = std::fs::read_to_string("input_2.txt").expect("Failed to read input file");
    let nodes = parse_input(&input);
    // Instead of running the entire part 2 svr -> out passing through dac and fft,
    // We can split it into two parts:
    // 1. svr -> dac * dac -> fft * fft -> out
    // 2. svr -> fft * fft -> dac * dac -> out

    let part2_a = part2_destinations(
        &nodes,
        vec![Id(*b"svr"), Id(*b"dac"), Id(*b"fft"), Id(*b"out")],
    );
    println!("-----------------------------");
    let part2_b = part2_destinations(
        &nodes,
        vec![Id(*b"svr"), Id(*b"fft"), Id(*b"dac"), Id(*b"out")],
    );

    println!("Part 2: {}", part2_a + part2_b);
}
