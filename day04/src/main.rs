use rayon::prelude::*;

fn check_adjacent(grid: &[Vec<bool>], row: usize, col: usize) -> bool {
    // If there are less than 4 adjacent '@', return true
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let mut count = 0;
    for (dr, dc) in directions.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
            && grid[new_row as usize][new_col as usize]
        {
            count += 1;
        }
    }
    count < 4
}

// fn print_grid(grid: &[Vec<bool>]) {
//     println!("Current grid state:");
//     for row in grid {
//         for &cell in row {
//             if cell {
//                 print!("@");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

fn main() {
    // Read the input file "input.txt"
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    // Create a grid from the input, where '@' represents a filled cell and '.' represents an empty cell
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    // let positions_with_at: Vec<(usize, usize)> = grid
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(r, row)| {
    //         row.iter()
    //             .enumerate()
    //             .filter_map(move |(c, &cell)| if cell { Some((r, c)) } else { None })
    //     })
    //     .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let valid_count_pt1: i32 = (0..rows)
        .into_par_iter()
        .map(|r| {
            // clone the positions vector for this thread/iteration so we don't need a mutable borrow
            let mut row_count = 0;
            for c in 0..cols {
                if grid[r][c] && check_adjacent(&grid, r, c) {
                    row_count += 1;
                }
            }
            row_count
        })
        .sum();
    println!("Result for part 1: {valid_count_pt1}");

    // print_grid(&grid);

    let mut count_pt2 = 0;
    loop {
        let removed_positions: Vec<(usize, usize)> = (0..rows)
            .into_par_iter()
            .map(|r| {
                let mut local_removed = Vec::new();
                for c in 0..cols {
                    if grid[r][c] && check_adjacent(&grid, r, c) {
                        local_removed.push((r, c));
                    }
                }
                local_removed
            })
            .flatten()
            .collect();

        for (r, c) in &removed_positions {
            grid[*r][*c] = false;
        }
        count_pt2 += removed_positions.len();
        // print_grid(&grid);

        if removed_positions.is_empty() {
            break;
        }
    }

    println!("Result for part 2: {count_pt2}");
}
