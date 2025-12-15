use std::clone;

struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Debug for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Point3D {{ x: {}, y: {}, z: {} }}",
            self.x, self.y, self.z
        )
    }
}

impl std::cmp::PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl clone::Clone for Point3D {
    fn clone(&self) -> Self {
        Point3D {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

fn euclidean_distance(p1: &Point3D, p2: &Point3D) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let dz = p2.z - p1.z;
    ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
}

const MAX_NUM_CONNECTIONS: usize = 1000;

fn main() {
    let input = "input.txt";
    let contents = std::fs::read_to_string(input).expect("Failed to read input file");
    let mut points: Vec<Point3D> = contents
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|num| num.trim().parse().expect("Invalid number"))
                .collect();
            Point3D {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    // Calculate the distance between all pairs of points
    let mut distances: Vec<(f64, [Point3D; 2])> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = euclidean_distance(&points[i], &points[j]);
            distances.push((dist, [points[i].clone(), points[j].clone()]));
        }
    }
    // Sort distances in descending order
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits: Vec<Vec<Point3D>> = Vec::new();

    // Perform MAX_NUM_CONNECTIONS connections between the closest points, and store them in circuits
    for (i, connection) in distances.iter().enumerate() {
        let local_last_connected_pair = connection.1.clone();

        // If one of the points is already in a circuit, add the other point to that circuit
        let mut added = false;
        for circuit in &mut circuits {
            if circuit.contains(&connection.1[0]) && !circuit.contains(&connection.1[1]) {
                circuit.push(connection.1[1].clone());
                if let Some(idx) = points.iter().position(|p| p == &connection.1[1]) {
                    points.remove(idx);
                }
                added = true;
                break;
            } else if circuit.contains(&connection.1[1]) && !circuit.contains(&connection.1[0]) {
                circuit.push(connection.1[0].clone());
                if let Some(idx) = points.iter().position(|p| p == &connection.1[0]) {
                    points.remove(idx);
                }
                added = true;
                break;
            } else if circuit.contains(&connection.1[0]) && circuit.contains(&connection.1[1]) {
                added = true;
                break;
            }
        }
        // If neither point is in a circuit, create a new circuit
        if !added {
            circuits.push(vec![connection.1[0].clone(), connection.1[1].clone()]);
            if let Some(idx) = points.iter().position(|p| p == &connection.1[0]) {
                points.remove(idx);
            }
            if let Some(idx) = points.iter().position(|p| p == &connection.1[1]) {
                points.remove(idx);
            }
        }

        // If there are points in more than one circuit, merge the circuits
        let mut merged = false;
        for i in 0..circuits.len() {
            for j in (i + 1)..circuits.len() {
                if circuits[i].iter().any(|p| circuits[j].contains(p)) {
                    let mut new_circuit = circuits[i].clone();
                    for p in &circuits[j] {
                        if !new_circuit.contains(p) {
                            new_circuit.push(p.clone());
                        }
                    }
                    circuits[i] = new_circuit;
                    circuits.remove(j);
                    merged = true;
                    break;
                }
            }
            if merged {
                break;
            }
        }
        if i == MAX_NUM_CONNECTIONS - 1 {
            // Sort the circuits by length in descending order
            circuits.sort_by_key(|b| std::cmp::Reverse(b.len()));
            let mut result_pt1 = 1;
            // Print the 3 longest circuits
            for (i, circuit) in circuits.iter().take(3).enumerate() {
                // println!("Circuit {} with length {}", i + 1, circuit.len(),);
                result_pt1 *= circuit.len();
            }
            println!("Part 1 result: {}", result_pt1);
        }
        if circuits.len() == 1 && points.is_empty() {
            println!(
                "Part 2 result: Distance between last connected pair: {} * {} = {}",
                local_last_connected_pair[0].x,
                local_last_connected_pair[1].x,
                (local_last_connected_pair[0].x * local_last_connected_pair[1].x)
            );
            break;
        }
    }
}
