use rayon::prelude::*;
use std::vec;

// Create a macro to calculate the area between two points
macro_rules! calculate_area {
    ($point_a:expr, $point_b:expr) => {
        (($point_a.0 - $point_b.0).abs() + 1) as u64 * (($point_a.1 - $point_b.1).abs() + 1) as u64
    };
}

fn calculate_area(point_a: (i32, i32), point_b: (i32, i32)) -> u64 {
    ((point_a.0 - point_b.0).abs() + 1) as u64 * ((point_a.1 - point_b.1).abs() + 1) as u64
}

fn part1(parsed: &[(i32, i32)]) -> u64 {
    let mut max_area: u64 = 0;
    let mut points_used = ((0, 0), (0, 0));
    // Only check each pair once (avoid checking both (a,b) and (b,a))
    for i in 0..parsed.len() {
        for j in (i + 1)..parsed.len() {
            let area = calculate_area(parsed[i], parsed[j]);
            if area > max_area {
                max_area = area;
                points_used = (parsed[i], parsed[j]);
            }
        }
    }
    println!(
        "Points used: ({}, {}), ({}, {})",
        points_used.0.0, points_used.0.1, points_used.1.0, points_used.1.1
    );
    max_area
}

fn part2_precompute(parsed: &[(i32, i32)]) -> Vec<Vec<(i32, i32)>> {
    let max_y = parsed.iter().map(|(_, y)| *y).max().unwrap_or(0);
    let grid_height = max_y + 1;

    // let mut grid: Vec<Vec<bool>> = vec![vec![false; grid_width as usize]; grid_height as usize];
    let mut parsed_per_line_vec: Vec<Vec<(i32, i32)>> = vec![vec![]; grid_height as usize];

    // Iterate though all the points and create a path between them
    let mut previous_point = parsed[parsed.len() - 1];
    for point in parsed {
        // grid[point.1 as usize][point.0 as usize] = true;
        parsed_per_line_vec[point.1 as usize].push(*point);
        // From the previous point, create a path to the current point in the grid
        for x in previous_point.0.min(point.0)..=previous_point.0.max(point.0) {
            for y in previous_point.1.min(point.1)..=previous_point.1.max(point.1) {
                parsed_per_line_vec[y as usize].push((x, y));
                // grid[y as usize][x as usize] = true;
            }
        }
        previous_point = *point;
    }
    for row in parsed_per_line_vec.iter_mut() {
        // Deduplicate points
        row.sort_unstable();
        row.dedup();
    }

    // let mut output: String = String::new();
    // let mut file = std::fs::File::create("output_grid.txt").expect("Failed to create grid file");
    // for y in 0..grid_height {
    //     for x in 0..grid_width {
    //         if parsed_per_line_vec[y as usize].contains(&(x, y)) {
    //             output.push('#');
    //         } else {
    //             output.push('.');
    //         }
    //     }
    //     output.push('\n');
    // }
    // std::io::Write::write_all(&mut file, output.as_bytes()).expect("Failed to write grid to file");
    parsed_per_line_vec
}

fn part2_check(parsed: &[(i32, i32)], parsed_per_line_vec: &[Vec<(i32, i32)>]) -> u64 {
    let mut max_area = 0;
    let mut points_used = ((0, 0), (0, 0));

    let results = parsed
        .par_iter()
        .enumerate()
        .flat_map(|(id_a, point_a)| {
            parsed
                .par_iter()
                .rev()
                .enumerate()
                .filter(move |(id_b, _)| id_a < *id_b)
                .filter_map(move |(_, point_b)| {
                    let area = calculate_area!(*point_a, *point_b);
                    if area > max_area
                        && check_includes_sides_in_square(*point_a, *point_b, parsed_per_line_vec)
                    {
                        Some((area, (*point_a, *point_b)))
                    } else {
                        None
                    }
                })
        })
        .max_by_key(|(area, _)| *area);

    if let Some((area, points)) = results {
        max_area = area;
        points_used = points;
    }
    println!(
        "Points used: ({}, {}), ({}, {})",
        points_used.0.0, points_used.0.1, points_used.1.0, points_used.1.1
    );
    max_area
}

fn check_raycast(point: (i32, i32), parsed_per_line_vec: &[Vec<(i32, i32)>]) -> bool {
    let y = point.1 as usize;

    // Out of bounds check
    if y >= parsed_per_line_vec.len() {
        return false;
    }

    let walls_on_line = &parsed_per_line_vec[y];

    // Use binary search to find the first wall at or after our point's x-coordinate
    // Since walls_on_line is sorted, this is O(log n) instead of O(n)
    let idx = walls_on_line.partition_point(|&(wall_x, _)| wall_x < point.0);

    // Check if point is exactly on a wall
    if idx < walls_on_line.len() && walls_on_line[idx].0 == point.0 {
        return true;
    }

    // Count wall segments to the right by detecting gaps
    let mut crossings = 0;
    let mut prev_x = None;

    for &(wall_x, _) in &walls_on_line[idx..] {
        // Start a new segment if this is the first wall or there's a gap
        if prev_x.is_none() || wall_x > prev_x.unwrap() + 1 {
            crossings += 1;
        }
        prev_x = Some(wall_x);
    }

    // Odd number of crossings means inside the polygon
    crossings & 1 == 1
}

fn check_includes_sides_in_square(
    point_a: (i32, i32),
    point_b: (i32, i32),
    parsed_per_line_vec: &[Vec<(i32, i32)>],
) -> bool {
    let min_x = point_a.0.min(point_b.0);
    let max_x = point_a.0.max(point_b.0);
    let min_y = point_a.1.min(point_b.1);
    let max_y = point_a.1.max(point_b.1);

    // The idea is checking the 4 corners first, and if all 4 corners are inside the shape,
    // then we check the lines that form the square do not intersect the walls of the shape.
    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];

    // Check corners first using the ray casting method
    for corner in &corners {
        let y_index = corner.1 as usize;
        if parsed_per_line_vec[y_index].binary_search(corner).is_ok() {
            continue; // On the wall is considered inside
        }
        if !check_raycast(*corner, parsed_per_line_vec) {
            return false; // Corner is outside
        }
    }

    // Check top and bottom edges by also tracing rays
    for x in min_x..=max_x {
        if !check_raycast((x, min_y), parsed_per_line_vec) {
            return false;
        }
        if !check_raycast((x, max_y), parsed_per_line_vec) {
            return false;
        }
    }

    // Check left and right edges
    for y in min_y..=max_y {
        if !check_raycast((min_x, y), parsed_per_line_vec) {
            return false;
        }
        if !check_raycast((max_x, y), parsed_per_line_vec) {
            return false;
        }
    }

    true
}

fn main() {
    // Limit the number of threads to avoid oversubscription
    rayon::ThreadPoolBuilder::new()
        .num_threads(14)
        .build_global()
        .unwrap();

    let file = "input.txt";
    let content = std::fs::read_to_string(file).expect("Failed to read file");
    let parsed: Vec<(i32, i32)> = content
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let first = parts
                .next()
                .and_then(|s| s.trim().parse::<i32>().ok())
                .expect("Failed to parse first number");
            let second = parts
                .next()
                .and_then(|s| s.trim().parse::<i32>().ok())
                .expect("Failed to parse second number");
            (first, second)
        })
        .collect();

    // let min_x = parsed.iter().map(|(x, _)| *x).min().unwrap_or(0);
    // let min_y = parsed.iter().map(|(_, y)| *y).min().unwrap_or(0);

    // // Substract min_x and min_y from all points to normalize to (0,0)
    // let parsed: Vec<(i32, i32)> = parsed.iter().map(|(x, y)| (x - min_x, y - min_y)).collect();

    // Find the largest area between two points ( a square )
    let result = part1(&parsed);
    println!("Largest area: {result}");

    // Part 2: delimit the area, delimited by the point_a and point_b being the edges.
    // So, for the input (1,1), (1,3), (5,3), (5,2), (1,2)
    // The possible area, instead of being infinite, is now limited to the shape formed by those points.

    let parsed_per_line_vec = part2_precompute(&parsed);
    let result2 = part2_check(&parsed, &parsed_per_line_vec);
    println!("Part 2 result: {result2}");
}
