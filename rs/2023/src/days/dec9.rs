use std::fs;


pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file) => {
            let mut result = 0;

            for line in file.lines() {
                // parse sequence of numbers
                let numbers: Vec<i32> = line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
                // get the next number in sequence
                let mut differences: Vec<Vec<i32>> = Vec::new();
                differences.push(numbers.clone());
                while differences[differences.len() - 1].len() >= 2 && !(differences[differences.len() - 1][1] == 0 && differences[differences.len() - 1][0] == 0) {
                    let last_diff = &differences[differences.len() - 1];
                    let mut prev = last_diff[0];
                    let mut new_differences: Vec<i32> = Vec::new();
                    for num in last_diff[1..].iter() {
                        new_differences.push(*num - prev);
                        prev = num.clone();
                    }
                    differences.push(new_differences);
                }

                let next_elem = differences.into_iter().rfold(0, |acc, x| x[x.len() - 1] + acc);
                result += next_elem;
            }

            println!("D9P1: {}", result)
        },
        Err(e) => {
            println!("Bruh {}", e)
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file) => {
            let mut result = 0;

            for line in file.lines() {
                // parse sequence of numbers
                let numbers: Vec<i32> = line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
                // get the next number in sequence
                let mut differences: Vec<Vec<i32>> = Vec::new();
                differences.push(numbers.clone());
                while differences[differences.len() - 1].len() >= 2 && !(differences[differences.len() - 1][1] == 0 && differences[differences.len() - 1][0] == 0) {
                    let last_diff = &differences[differences.len() - 1];
                    let mut prev = last_diff[0];
                    let mut new_differences: Vec<i32> = Vec::new();
                    for num in last_diff[1..].iter() {
                        new_differences.push(*num - prev);
                        prev = num.clone();
                    }
                    differences.push(new_differences);
                }

                let next_elem = differences.into_iter().rfold(0, |acc, x| x[0] - acc);
                result += next_elem;
            }

            println!("D9P2: {}", result)
        },
        Err(e) => {
            println!("Bruh {}", e)
        }
    }
}