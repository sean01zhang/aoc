use std::{fs, collections::HashMap};


pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let lines: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();

            let mut answer = 0;

            for i in 0..lines.len() {
                // if character, add to number
                let mut j = 0;
                let mut num_start = 0;
                let mut is_included = false;

                while j < lines[i].len() {
                    // get to first number
                    match lines[i][j] {
                        c if c.is_numeric() => {
                            // accumulate number
                            println!("num_start {}", num_start);
                            num_start = num_start * 10 + (c as i32 - '0' as i32);
                            // if still a candidate
                            if !is_included {
                                for xoffset in (if j == 0 {1} else {0})..=2 {
                                    for yoffset in (if i==0 {1} else {0})..=2 {
                                        // try to get adj char if it exists
                                        match lines.get(i+yoffset-1).and_then(|line| line.get(j + xoffset-1)) {
                                            Some(adj_char) => {
                                                println!("Candidate {} - {}", num_start, adj_char);
                                                if (xoffset != 1 || yoffset != 1) && *adj_char != '.' && !adj_char.is_numeric() {
                                                    is_included = true;
                                                }
                                            }
                                            None => ()
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            if is_included {
                                println!("Added {}", num_start);
                                answer += num_start;
                            } 
                            num_start = 0; 
                            is_included = false
                        }
                    }
                    j += 1; 
                }
                if is_included {
                    println!("Added {}", num_start);
                    answer += num_start;
                } 
            }

            println!("D3P1: {}", answer)
        },
        Err(err) => {
            println!("Bruh {:?}", err)
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let lines: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();

            let mut gear_ratios: HashMap<usize, Vec<i32>> = HashMap::new();

            for i in 0..lines.len() {
                // if character, add to number
                let mut j = 0;
                let mut num_start = 0;
                let mut is_included = false;
                let len_line= lines[i].len();

                let mut adjacents : Vec<usize> = Vec::new();

                while j < len_line {
                    // get to first number
                    match lines[i][j] {
                        c if c.is_numeric() => {
                            // accumulate number
                            println!("num_start {}", num_start);
                            num_start = num_start * 10 + (c as i32 - '0' as i32);
                            // if still a candidate
                            if !is_included {
                                for xoffset in (if j == 0 {1} else {0})..=2 {
                                    for yoffset in (if i==0 {1} else {0})..=2 {
                                        // try to get adj char if it exists
                                        match lines.get(i+yoffset-1).and_then(|line| line.get(j + xoffset-1)) {
                                            Some(adj_char) => {
                                                println!("Candidate {} - {}", num_start, adj_char);
                                                if (xoffset != 1 || yoffset != 1) && *adj_char == '*' {
                                                    let index = (i + yoffset - 1) * len_line + (j + xoffset);
                                                    adjacents.push(index);
                                                    is_included = true;
                                                }
                                            }
                                            None => ()
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            if is_included {
                                // for each adjacent
                                adjacents.iter().for_each(|key| {
                                    // add num_start to that key
                                    match gear_ratios.get_mut(key) {
                                        Some(nums) => {
                                            nums.push(num_start);
                                        },
                                        None => {
                                            gear_ratios.insert(*key, vec![num_start]);
                                        }
                                    }
                                });
                                adjacents.clear();
                                // println!("Added {}", num_start);
                                // answer += num_start;
                            } 
                            num_start = 0; 
                            is_included = false
                        }
                    }
                    j += 1; 
                }
                if is_included {
                    // for each adjacent
                    adjacents.iter().for_each(|key| {
                        // add num_start to that key
                        match gear_ratios.get_mut(key) {
                            Some(nums) => {
                                nums.push(num_start);
                            },
                            None => {
                                gear_ratios.insert(*key, vec![num_start]);
                            }
                        }
                    });
                    // adjacents.clear();
                    // answer += num_start;
                } 
            }

            // multiply and sum to get answer
            let mut answer = 0;
            for ratios in gear_ratios.values() {
                if ratios.len() == 2 {
                    answer += ratios[0] * ratios[1]
                }
            }

            println!("D3P2: {}", answer)
        },
        Err(err) => {
            println!("Bruh {:?}", err)
        }
    }
}