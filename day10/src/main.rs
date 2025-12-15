use std::{
    any::TypeId,
    collections::{BTreeSet, HashMap, HashSet},
    result,
};

use itertools::Itertools;

struct Machine {
    desired_state: Vec<bool>,
    buttons: Vec<Vec<i32>>,
    joltage: Vec<i32>,
}

impl Machine {
    fn new(desired_state: Vec<bool>, buttons: Vec<Vec<i32>>, joltage: Vec<i32>) -> Self {
        Machine {
            desired_state,
            buttons,
            joltage,
        }
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Desired State: {:?}, Buttons: {:?}, Joltage: {:?}",
            self.desired_state, self.buttons, self.joltage
        )
    }
}

fn find_number_presses_turning_on(machine: &Machine) -> i32 {
    // Placeholder function
    // We have to test how many presses of buttons are needed to reach the desired state, assuming we start from all false.
    let start = vec![false; machine.desired_state.len()];

    // We need to find all the possible combinations of button presses.
    // Something like python's itertools.combinations: Return r length subsequences of elements from the input iterable
    // WE iterate though all the possible number of lengths of combinations from 1 to the number of buttons + 1
    for button_presses_count in 1..=machine.buttons.len() + 1 {
        // For each length, we need to find all the possible combinations of buttons
        let combinations = machine.buttons.iter().combinations(button_presses_count);

        for combination in combinations {
            // For each combination, we need to apply the button presses to the start state
            let mut current_state = start.clone();

            for button in combination {
                for &index in button {
                    current_state[index as usize] = !current_state[index as usize];
                }
            }

            // After applying the button presses, we need to check if we reached the desired state
            if current_state == machine.desired_state {
                // println!(
                //     "Reached desired state with {} button presses",
                //     button_presses_count
                // );
                return button_presses_count as i32;
            }
        }
    }

    0
}

// Returns all the possible combinations of button presses for a given machine, and the number of presses needed to reach that state
fn possible_combinations(machine: &Machine) -> i32 {
    if machine.joltage == vec![10, 11, 11, 5, 10, 5] {
        print!("");
    }
    if machine.joltage.iter().all(|&x| x == 0) {
        return 0;
    }
    if machine.joltage.iter().any(|&x| x < 0) {
        return i32::MAX;
    }

    let mut results = Vec::new();
    let start = vec![false; machine.desired_state.len()];

    // Create the "goal" state by checking if the joltage is even or odd, and marking true the odd ones
    let desired_state = machine
        .joltage
        .iter()
        .map(|&x| x % 2 != 0)
        .collect::<Vec<bool>>();

    // We need to find all the possible combinations of button presses.
    // We iterate though all the possible number of lengths of combinations from 1 to the number of buttons + 1
    for button_presses_count in 1..=machine.buttons.len() + 1 {
        // For each length, we need to find all the possible combinations of buttons
        let combinations = machine.buttons.iter().combinations(button_presses_count);

        for combination in combinations.clone() {
            // For each combination, we need to apply the button presses to the start state
            let mut current_state = start.clone();

            for &button in &combination {
                for &index in button {
                    current_state[index as usize] = !current_state[index as usize];
                }
            }

            // After applying the button presses, we need to check if we reached the desired state
            if current_state == desired_state {
                // println!(
                //     "Reached desired state with {} button presses",
                //     button_presses_count
                // );
                results.push((combination, button_presses_count as i32));
            }
        }
    }
    if results.is_empty() {
        return i32::MAX;
    }

    let mut least_presses = i32::MAX;
    for (combination, _) in results.iter() {
        let mut modified_desired_state = machine.joltage.clone();
        for &button in combination {
            for &index in button {
                modified_desired_state[index as usize] -= 1;
            }
        }
        if modified_desired_state.iter().any(|&x| x < 0) {
            continue;
        }
        // println!(
        //     "Combination: {:?}, Modified Desired State: {:?}",
        //     combination, modified_desired_state
        // );

        modified_desired_state = modified_desired_state
            .iter()
            .map(|&x| x / 2)
            .collect::<Vec<i32>>();

        // We now have to find the number of buttons that will sum up to each value in the modified desired state.
        let modified_machine = Machine::new(
            machine.desired_state.clone(),
            machine.buttons.clone(),
            modified_desired_state,
        );
        let presses = possible_combinations(&modified_machine);

        let local_least_presses = presses * 2 + combination.len() as i32;

        least_presses = least_presses.min(local_least_presses);
        if least_presses != local_least_presses {
            println!(
                "New least presses found: {} (previously {}) with combination {:?}",
                least_presses, local_least_presses, combination
            );
        }
    }
    least_presses
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let desired_state = parts[0][1..parts[0].len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<bool>>();
        let joltage = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let buttons: Vec<Vec<i32>> = parts[1..parts.len() - 1]
            .to_vec()
            .iter()
            .map(|l| {
                l[1..l.len() - 1]
                    .split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        let machine = Machine::new(desired_state, buttons, joltage);
        machines.push(machine);
    }

    machines
}

fn generate_output_combinations(
    buttons: &Vec<Vec<i32>>,
) -> HashMap<BTreeSet<i32>, Vec<Vec<Vec<i32>>>> {
    let mut patterns: HashMap<BTreeSet<i32>, Vec<Vec<Vec<i32>>>> = HashMap::new();

    for num_presses in 0..=buttons.len() {
        let combinations = buttons.iter().combinations(num_presses);
        for presses in combinations {
            let mut pattern: BTreeSet<i32> = BTreeSet::new();

            // XOR all buttons in this combination
            for &button in &presses {
                for &index in button {
                    if pattern.contains(&index) {
                        pattern.remove(&index);
                    } else {
                        pattern.insert(index);
                    }
                }
            }

            // Convert presses to Vec<Vec<i32>>
            let presses_vec: Vec<Vec<i32>> = presses.iter().map(|b| b.to_vec()).collect();

            patterns.entry(pattern).or_default().push(presses_vec);
        }
    }

    patterns
}

fn part2(machine: &Machine) -> i32 {
    // Obtain the possible combinations of button presses, aka patterns
    let combinations = generate_output_combinations(&machine.buttons);

    fn get_minimum_presses(
        target: Vec<i32>,
        combinations: &HashMap<BTreeSet<i32>, Vec<Vec<Vec<i32>>>>,
    ) -> Option<i32> {
        if target.iter().all(|&x| x == 0) {
            return Some(0);
        }

        let goal_indicator = target
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x % 2 != 0 { Some(i as i32) } else { None })
            .collect::<BTreeSet<i32>>();

        let mut result: Option<i32> = None;
        for presses in combinations.get(&goal_indicator)?.iter() {
            let mut modified_target = target.clone();
            for button in presses {
                for &index in button {
                    modified_target[index as usize] -= 1;
                }
            }
            if target.iter().any(|&x| x < 0) {
                return None;
            }
            modified_target = modified_target.iter().map(|&x| x / 2).collect::<Vec<i32>>();
            let sub_result = get_minimum_presses(modified_target, combinations);
            match sub_result {
                Some(sub_presses) => {
                    let total_presses = sub_presses * 2 + presses.len() as i32;
                    result = match result {
                        Some(current_min) => Some(current_min.min(total_presses)),
                        None => Some(total_presses),
                    };
                }
                None => continue,
            }
        }
        result
    }

    get_minimum_presses(machine.joltage.clone(), &combinations).unwrap_or(1000000)
}

fn main() {
    // Read the input from a file
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    let machines = parse_input(&input);

    // for machine in &machines {
    //     println!("{}", machine);
    // }

    let mut result = 0;

    for machine in &machines {
        let presses = find_number_presses_turning_on(machine);
        result += presses;
    }
    println!(
        "Total number of button presses needed: {} \n--------------------------------------",
        result
    );

    let mut result_part2 = 0;
    for machine in &machines {
        let presses = part2(machine);
        result_part2 += presses;
    }
    println!(
        "Total number of button presses needed (Part 2): {}",
        result_part2
    );
}
