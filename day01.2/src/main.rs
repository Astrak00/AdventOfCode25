fn main() {
    // reading input from a file
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    // processing the input
    let instructions: Vec<&str> = input.lines().collect();

    // splitting instructions into two parts, first letter and the rest is a number
    let parsed_instructions: Vec<(&str, i32)> = instructions
        .iter()
        .map(|line| {
            let (command, value) = line.split_at(1);
            (
                command,
                value.trim().parse::<i32>().expect("Failed to parse number"),
            )
        })
        .collect();

    let mut position = 50;
    let mut count_zeros_passed = 0;

    for (command, value) in parsed_instructions {
        match command {
            "L" => {
                for _ in 0..value {
                    position -= 1;
                    if position == 0 {
                        count_zeros_passed += 1;
                    } else if position < 0 {
                        position += 100;
                    }
                }
            }
            "R" => {
                for _ in 0..value {
                    position += 1;
                    if position >= 100 {
                        position = 0;
                        count_zeros_passed += 1;
                    }
                }
            }
            _ => panic!("Unknown command"),
        }
    }

    println!("------------------------------");

    println!(
        "Part 2: Number of times position returned to zero: {}",
        count_zeros_passed
    )
}
