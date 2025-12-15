use rayon::prelude::*;

// const SOME_CONSTANT: i32 = 12;

fn max_possible_number(digits: Vec<i32>) -> i32 {
    // Obtain the highest 2 digit number from the given digits, without changing their order.
    // It can be any two digits, not necessarily adjacent
    let mut max_number = -1;
    let len = digits.len();
    for i in 0..len {
        for j in i + 1..len {
            let number = digits[i] * 10 + digits[j];
            if number > max_number {
                max_number = number;
            }
        }
    }
    max_number
}

fn max_possible_number_n(digits: Vec<i32>, n: usize) -> u64 {
    // Obtain the highest 12 digit number from the given digits, without changing their order.
    let mut max_number: u64 = 0;
    let len = digits.len();
    let k = n;

    if len < k {
        return 0;
    }

    let mut current_pos = 0;
    for i in 0..k {
        let remaining = k - 1 - i;
        let end_search = len - 1 - remaining;

        let mut best_digit = -1;
        let mut best_index = 0;

        for j in current_pos..=end_search {
            if digits[j] > best_digit {
                best_digit = digits[j];
                best_index = j;
                if best_digit == 9 {
                    break;
                }
            }
        }

        max_number = max_number * 10 + best_digit as u64;
        current_pos = best_index + 1;
    }

    max_number
}

fn main() {
    // Limit the number of threads used by Rayon to 4
    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(1)
    //     .build_global()
    //     .unwrap();

    // Read input from the file input.txt
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    // Parse the input into lines
    let lines: Vec<&str> = input.lines().collect();

    // Each of the lines is made up of individual numbers, store them in a vector of vector of i32
    let numbers: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Failed to parse digit") as i32)
                .collect()
        })
        .collect();

    // Print the numbers for verification
    let (sum_pt1, sum_pt2): (u128, u128) = numbers
        .par_iter()
        .map(|row| {
            let max_number = max_possible_number_n(row.clone(), 2) as u128;
            let max_number_12 = max_possible_number_n(row.clone(), 12) as u128;
            // println!(
            //     "Row: {:?}, Max 2-digit number: {}, Max 12-digit number: {}",
            //     row, max_number, max_number_12
            // );
            (max_number, max_number_12)
        })
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("Total sum of max possible numbers: {}", sum_pt1);
    println!("Total sum of max possible numbers (12 digits): {}", sum_pt2);

    // let mut total_sum_pt1: u128 = 0;
    // let mut total_sum_pt2: u128 = 0;
    // for row in &numbers {
    //     let max_number = max_possible_number(row.clone());
    //     total_sum_pt1 += max_number as u128;
    //     let max_number_12 = max_possible_number_12(row.clone());
    //     total_sum_pt2 += max_number_12 as u128;
    //     // println!("Max possible number: {}", max_number);
    // }
    // println!("Total sum of max possible numbers: {}", total_sum_pt1);
    // println!(
    //     "Total sum of max possible numbers (12 digits): {}",
    //     total_sum_pt2
    // );
}
