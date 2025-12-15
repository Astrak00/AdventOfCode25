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

fn part2_general(nodes: NodeList, visited: &mut HashMap<Id, bool>, current: Id, target: Id) -> i32 {
    if current == target {
        return 1;
    }
    let mut total_paths = 0;
    if let Some(neighbors) = nodes.get(&current) {
        for neighbor in neighbors {
            if !visited.get(neighbor).unwrap_or(&false) {
                visited.insert(*neighbor, true);
                let count = part2_general(nodes.clone(), visited, *neighbor, target);
                total_paths += count;
                visited.insert(*neighbor, false);
            }
        }
    }
    total_paths
}

fn part2(
    nodes: NodeList,
    visited: &mut HashMap<Id, bool>,
    current: Id,
    mut _visited_dac: bool,
    mut _visited_fft: bool,
) -> i32 {
    if current == Id(*b"out") && _visited_dac && _visited_fft {
        // println!("Reached 'out' node!");
        return 1;
    } else if current == Id(*b"dac") {
        _visited_dac = true;
    } else if current == Id(*b"fft") {
        _visited_fft = true;
    }
    let mut total_paths = 0;
    if let Some(neighbors) = nodes.get(&current) {
        for neighbor in neighbors {
            if !visited.get(neighbor).unwrap_or(&false) {
                visited.insert(*neighbor, true);
                let count = part2(
                    nodes.clone(),
                    visited,
                    *neighbor,
                    _visited_dac,
                    _visited_fft,
                );
                total_paths += count;
                visited.insert(*neighbor, false);
            }
        }
    }
    total_paths
}

fn part2_destinations(nodes: NodeList, order: Vec<Id>) -> i32 {
    let mut result_2_gen = 1;

    use rayon::prelude::*;

    let results: Vec<i32> = (0..(order.len() - 1))
        .into_par_iter()
        .map(|ord_idx| {
            println!(
                "Calculating paths from {} to {}",
                order[ord_idx],
                order[ord_idx + 1]
            );
            part2_general(
                nodes.clone(),
                &mut HashMap::new(),
                order[ord_idx],
                order[ord_idx + 1],
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
        nodes.clone(),
        vec![Id(*b"svr"), Id(*b"dac"), Id(*b"fft"), Id(*b"out")],
    );
    println!("-----------------------------");
    let part2_b = part2_destinations(
        nodes.clone(),
        vec![Id(*b"svr"), Id(*b"fft"), Id(*b"dac"), Id(*b"out")],
    );

    println!("Part 2: {}", part2_a + part2_b);
}
