fn part1(lines: &mut Vec<String>) -> (usize, Vec<String>) {
    let mut ray_positions: Vec<usize> = Vec::new();
    let board_width = lines[0].len() as u32;
    let board_height = lines.len() as u32;
    // Find the start position of the ray, at the first line (where an 'S' is located)
    for (index, ch) in lines[0].chars().enumerate() {
        if ch == 'S' {
            ray_positions.push(index);
        }
    }

    let mut count_splits = 0;
    for line_idx in 1..board_height {
        // let line_idx = 1;
        // Split the ray if '^' is found under the ray
        let mut new_ray_positions: Vec<usize> = Vec::new();
        for &ray_local in ray_positions.iter() {
            if lines[line_idx as usize].chars().nth(ray_local) == Some('^') {
                let mut has_split = false;
                if ray_local > 0 && !new_ray_positions.contains(&(ray_local - 1)) {
                    new_ray_positions.push(ray_local - 1);
                    has_split = true;
                }
                if ray_local < (board_width - 1) as usize
                    && !new_ray_positions.contains(&(ray_local + 1))
                {
                    new_ray_positions.push(ray_local + 1);
                    has_split = true;
                }
                if has_split {
                    count_splits += 1;
                }
            } else {
                new_ray_positions.push(ray_local);
            }
        }
        ray_positions = new_ray_positions;

        // Propagación del rayo
        let mut line = lines[line_idx as usize].clone();
        for &ray_local in ray_positions.iter() {
            line.replace_range(ray_local..ray_local + 1, "|");
        }
        lines[line_idx as usize] = line;
    }

    (count_splits, lines.clone())
}

fn print_board(lines: &Vec<String>) {
    for line in lines.iter() {
        println!("{}", line);
    }
}

fn part2(lines: &mut Vec<String>) -> usize {
    // Part 2 consists on conting the number of possible ray paths
    // To do this, we will use a dynamic programming approach
    let board_width = lines[0].len();
    let board_height = lines.len();
    let mut dp = vec![vec![0usize; board_width]; board_height];
    // Initialize the first row
    for (index, ch) in lines[0].chars().enumerate() {
        if ch == 'S' {
            dp[0][index] = 1;
        }
    }

    for row in 1..board_height {
        for col in 0..board_width {
            let ch = lines[row].chars().nth(col).unwrap();
            if ch == '^' {
                // The ray can come from the left or right
                if col > 0 {
                    dp[row][col - 1] += dp[row - 1][col];
                }
                if col < board_width - 1 {
                    dp[row][col + 1] += dp[row - 1][col];
                }
            } else {
                // The ray continues straight down
                dp[row][col] += dp[row - 1][col];
            }
        }
    }
    let mut total_paths = 0;
    for &paths in dp[board_height - 1].iter() {
        total_paths += paths;
    }
    total_paths
}

fn main() {
    let file = "input.txt";
    let file_content = std::fs::read_to_string(file).expect("Failed to read file");
    let mut lines = file_content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let (count_splits, mut lines) = part1(&mut lines);
    print_board(&lines);

    println!("Total splits: {}", count_splits);

    let total_paths = part2(&mut lines);

    println!("Total possible ray paths: {}", total_paths);
}
