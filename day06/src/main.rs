fn main() {
    let file = "input.txt";
    let file_content = std::fs::read_to_string(file).expect("Failed to read file");
    let lines = file_content.lines();
    let mut table: Vec<Vec<&str>> = lines
        .map(|line| line.split_whitespace().collect())
        .collect();

    let oper: Vec<&str> = table.pop().unwrap();
    let table: Vec<Vec<&str>> = (0..table[0].len())
        .map(|i| table.iter().map(|row| row[i]).collect())
        .collect();

    let mut res_1 = 0;
    for (i, row) in table.iter().enumerate() {
        let expr = row.join(oper[i]);
        res_1 += eval::eval(&expr).unwrap().as_i64().unwrap();
    }
    println!("Result 1: {res_1}");

    // The idea is readint it per column, and
    let lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
    let mut table_2 = lines;
    let table_length = table_2.len();

    // Iterate through the last line to get the operator indices, add one and put an "&" before each number
    let operator_line = table_2.last().unwrap();
    let mut operator_indices: Vec<usize> = Vec::new();
    for (i, ch) in operator_line.chars().enumerate() {
        if ch == '+' || ch == '*' {
            // println!("Found operator {ch} at index {i}");
            if i == 0 {
                continue;
            }
            operator_indices.push(i - 1);
        }
    }
    // println!("Operator indices: {operator_indices:?}");
    // Replace the index in the lines with "&" except for the last line
    for line in table_2.iter_mut().take(table_length - 1) {
        for &index in operator_indices.iter().rev() {
            if index < line.len() {
                // Replace the character at the given index with '&'
                line.replace_range(index..index + 1, "&");
            }
        }
    }
    // Replace every space with a "0"
    for line in table_2.iter_mut().take(table_length - 1) {
        *line = line.replace(' ', "0");
    }
    // Split the numbers by '&' and collect them into a vector of strings
    let numbers = table_2
        .iter()
        .take(table_length - 1)
        .map(|line| {
            line.split('&')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    // Transpose the numbers vector to get columns
    let numbers: Vec<Vec<String>> = (0..numbers[0].len())
        .map(|i| numbers.iter().map(|row| row[i].clone()).collect())
        .collect();

    let mut cephalopod_numbers: Vec<Vec<i64>> = Vec::new();
    for number_row in numbers.iter() {
        // println!("{number_row:?}");
        let mut numbers_parsed: Vec<i64> = Vec::new();
        for number in 0..number_row.iter().map(|s| s.len()).max().unwrap() {
            let mut number_str = String::new();
            for n in number_row.iter() {
                let ch = n.chars().nth(number).unwrap_or(' ');
                if ch == '0' {
                    continue;
                }
                number_str.push(ch);
            }
            let number_parsed: i64 = number_str.trim().parse().unwrap_or(0);
            if number_parsed != 0 {
                numbers_parsed.push(number_parsed);
            }
        }
        cephalopod_numbers.push(numbers_parsed.into_iter().rev().collect());
    }

    // println!("numbers: {numbers:?}");

    let mut result_cephalopod = 0;

    for (i, row) in cephalopod_numbers.iter().enumerate() {
        let expr = row
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(oper[i]);
        let result_eval = eval::eval(&expr);
        result_cephalopod += result_eval.unwrap().as_i64().unwrap();
    }

    println!("Cephalopod result: {result_cephalopod}");
}
