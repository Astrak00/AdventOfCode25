// This function check if the given ID is made uo of exactly one pair of numbers
// for example, 11 is valid, but 1234 or 111 are not.
// 1212 is also valid, because it contains one pair of 12s, as is 6464, because it contains one pair of 64s.
// 101 is not valid, because the pair of 1s is separated by a 0.
fn check_id_part1(id: u64) -> bool {
    let id_str = id.to_string();

    if id_str.len() % 2 == 1 {
        return false;
    } else {
        let split_num_idx = id_str.len() / 2;
        let first_half = &id_str[..split_num_idx];
        let second_half = &id_str[split_num_idx..];
        if first_half != second_half {
            return false;
        }
    }
    true
}

fn check_id_part2(id: u64) -> bool {
    let id_str = id.to_string();
    let max_seq_size = id_str.len() / 2;
    for seq_size in 1..=max_seq_size {
        // Divide the string into sequences of seq_size
        let seq_vector = id_str
            .as_bytes()
            .chunks(seq_size)
            .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap_or("".to_string()))
            .collect::<Vec<String>>();
        // If all the elements in the vector are the same, return true
        if seq_vector.windows(2).all(|w| w[0] == w[1]) {
            return true;
        }
    }

    false
}

fn main() {
    let start_time = std::time::Instant::now();
    // reading input from a file
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    // processing the input
    let instructions: Vec<&str> = input.split(",").collect();
    let mut valid_ids_part1 = Vec::new();
    let mut valid_ids_part2 = Vec::new();

    let ranges: Vec<(u64, u64)> = instructions
        .iter()
        .map(|range| {
            let start = range.split("-").next().unwrap().parse::<u64>().unwrap();
            let end = range.split("-").nth(1).unwrap().parse::<u64>().unwrap();

            (start, end)
        })
        .collect();

    for (start, end) in &ranges {
        for id in *start..=*end {
            if check_id_part1(id) {
                valid_ids_part1.push(id);
            }
            if check_id_part2(id) {
                valid_ids_part2.push(id);
            }
        }
    }
    let duration = start_time.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    println!(
        "The sum of the valid IDs is: {}",
        valid_ids_part1.iter().sum::<u64>()
    );
    println!(
        "The sum of the valid IDs for part 2 is: {}",
        valid_ids_part2.iter().sum::<u64>()
    );

    /////////////////////////////////////////////////
    /// USING RAYON FOR PARALLEL PROCESSING
    /////////////////////////////////////////////////
    use rayon::prelude::*;
    let start_time = std::time::Instant::now();
    let ranges: Vec<(u64, u64)> = instructions
        .iter()
        .map(|range| {
            let mut range = range.trim().split("-");
            let start = range.next().unwrap_or_default().parse::<u64>().unwrap();
            let end = range.next().unwrap_or_default().parse::<u64>().unwrap();

            (start, end)
        })
        .collect();

    // Parallel computation using rayon
    let (sum_part1, sum_part2): (u64, u64) = ranges
        .par_iter()
        .map(|(start, end)| {
            let mut local_sum_part1 = 0u64;
            let mut local_sum_part2 = 0u64;

            for id in *start..=*end {
                if check_id_part1(id) {
                    local_sum_part1 += id;
                }
                if check_id_part2(id) {
                    local_sum_part2 += id;
                }
            }

            (local_sum_part1, local_sum_part2)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    let duration = start_time.elapsed();
    println!("Time elapsed in parallel computation is: {:?}", duration);
    println!("The sum of the valid IDs is: {}", sum_part1);
    println!("The sum of the valid IDs for part 2 is: {}", sum_part2);
}
