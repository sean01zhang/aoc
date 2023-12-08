use std::{fs, collections::HashSet};

pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let mut total = 0;

            // for each card
            input.lines().enumerate().for_each(|(i_lines, line)| {
                let chars = line[((i_lines+1).to_string().len() + 7)..].as_bytes();

                let mut is_reading_winning = true;
                let mut winning: HashSet<i32> = HashSet::new();
                let mut points = 0;

                let mut cur_number: Option<i32> = None;
                for c in chars {
                    if (*c as char).is_numeric() {
                        // process current number
                        cur_number = Some(match cur_number {
                            Some(num) => num * 10,
                            None => 0
                        } + (c - '0' as u8) as i32)
                    } else if (*c as char) == '|' {
                        is_reading_winning = false;
                    } else {
                        match cur_number {
                            Some(num) => {
                                if is_reading_winning {
                                    winning.insert(num);
                                } else if winning.contains(&num) {
                                    points = match points {
                                        0 => 1,
                                        _ => points * 2
                                    };
                                }
                            },
                            None => ()
                        }

                        // reset number
                        cur_number = None
                    }
                }
                match cur_number {
                    Some(num) => {
                        if winning.contains(&num) {
                            points = match points {
                                0 => 1,
                                _ => points * 2
                            };
                        }
                    },
                    None => ()
                }

                // get points on this card
                total += points;
            });

            println!("D4P1: {}", total);
        },
        Err(e) => {
            println!("Bruh {:?}", e)
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let mut card_instances: Vec<i32> = Vec::new();

            // for each card
            input.lines().enumerate().for_each(|(i_lines, line)| {
                // you have one instance of each card to begin with
                match card_instances.get(i_lines) {
                    None => card_instances.push(1),
                    Some(_) => card_instances[i_lines] += 1
                }

                let chars = line[((i_lines+1).to_string().len() + 7)..].as_bytes();

                let mut is_reading_winning = true;
                let mut winning: HashSet<i32> = HashSet::new();
                let mut n_matches = 0;

                let mut cur_number: Option<i32> = None;
                for c in chars {
                    if (*c as char).is_numeric() {
                        // process current number
                        cur_number = Some(match cur_number {
                            Some(num) => num * 10,
                            None => 0
                        } + (c - '0' as u8) as i32)
                    } else if (*c as char) == '|' {
                        is_reading_winning = false;
                    } else {
                        match cur_number {
                            Some(num) => {
                                if is_reading_winning {
                                    winning.insert(num);
                                } else if winning.contains(&num) {
                                    n_matches += 1;
                                    match card_instances.get(i_lines + n_matches) {
                                        None => card_instances.push(card_instances[i_lines]),
                                        Some(_) => card_instances[i_lines + n_matches] += card_instances[i_lines]
                                    }
                                }
                            },
                            None => ()
                        }

                        // reset number
                        cur_number = None
                    }
                }
                match cur_number {
                    Some(num) => {
                        if winning.contains(&num) {
                            n_matches += 1;
                            match card_instances.get(i_lines + n_matches) {
                                None => card_instances.push(card_instances[i_lines]),
                                Some(_) => card_instances[i_lines + n_matches] += card_instances[i_lines]
                            }
                        }
                    },
                    None => ()
                }
            });

            let total : i32 = card_instances.iter().sum();
            println!("D4P2: {}", total);
        },
        Err(e) => {
            println!("Bruh {:?}", e)
        }
    }
}