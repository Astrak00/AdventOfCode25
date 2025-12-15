fn main() {
    // Read input file until a line that is empty
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut lines = input.lines();
    //
    let mut array_ranges: Vec<(i64, i64)> = Vec::new();
    let mut fresh_items = 0;
    loop {
        let line = lines.next();
        match line {
            None => break,
            Some(line) => {
                if line.is_empty() {
                    continue;
                }
                if line.contains("-") {
                    let parts: Vec<&str> = line.split("-").collect();
                    let start: i64 = parts[0].parse().expect("Failed to parse start of range");
                    let end: i64 = parts[1].parse().expect("Failed to parse end of range");
                    array_ranges.push((start, end));
                } else {
                    let number: i64 = line.parse().expect("Failed to parse number");
                    // Check if the number is in any of the ranges
                    let mut is_in_range = false;
                    for (start, end) in &array_ranges {
                        if number >= *start && number <= *end {
                            is_in_range = true;
                            break;
                        }
                    }
                    if is_in_range {
                        fresh_items += 1;
                    }
                }
            }
        }
    }
    println!("Number of fresh items (part1): {fresh_items}");

    // Part 2: COunt the number of items that are inside any of the ranges
    // The ranges may overlap, so we need to merge them first
    array_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    for range in array_ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(range);
        } else {
            let last_range = merged_ranges.last_mut().unwrap();
            if range.0 <= last_range.1 {
                last_range.1 = last_range.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        }
    }
    let mut total_covered = 0;
    for (start, end) in merged_ranges {
        total_covered += end - start + 1;
    }
    println!("Total covered items (part2): {total_covered}");
}
