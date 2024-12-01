use std::{fs, collections::HashMap};


// idea: we start with an array
// given a sequence of numbers, S and a length of string, N:
// how many possibilities are there?
// IF:
// There is a ?:
//   IF:
//   Sum(S) + len(S) - 1 == N : There is 1 combination
//   Sum(S) + len(S) - 1 >= N : There are 0 combinations
//   Otherwise, we have decision to make:
//   1) Can we pop a number? => There is combo(S - 1, N-?) + combo(S, N-1)
//   2) Otherwise => combo(S, N-1)
// There is a #
//   We read subsequent hashes and then pop S => combo(S-1, N-?)
// There is a .
//   => combo(S, N-1)
fn get_combos(input: &str, sequence: Vec<i32>) -> i32 {
    let mut combos: HashMap<(i32, i32), i32> = HashMap::new();
    let input_bytes = input.as_bytes();

    combos.insert((-1, -1), 1);
    for (i_n, &input_byte) in input_bytes.iter().enumerate() {
        combos.insert((-1, i_n as i32), 1);
    }

    // strategy:
    for i_s in 0..sequence.len() {
        let mut is_used = false;
        for (i_n, &input_byte) in input_bytes.iter().enumerate() {
            let cur_char = input_byte as char;
            let cur_pair = (i_s as i32, i_n as i32);
            // is first in sequence
            let is_first = usize::min(i_n, 1) as i32;

            println!("{}", input);
            for (i, c) in input_bytes.iter().enumerate() {
                if i_n == i {
                    print!("^");
                } else {
                    print!(" ");
                }
            }
            println!("");

            // make sure the number of letters is less than the sum of all hashes required (plus gaps)
            if i_n + 1 < sequence[0..=i_s].iter().sum::<i32>() as usize + i_s {
                println!("-----");
                continue;
            }

            // assume i_s is positive, non-negative
            // assume i_n is greater than sum(S[0..i_s]) + i_s
            let &case_dot = combos.get(&(i_s as i32, i_n as i32 - 1)).unwrap_or(&0);
            println!("Get ({}, {})", i_s, i_n as i32 - 1);
            match cur_char {
                '?' => {
                    // check what char before the hashes are
                    let char_before = if i_n < sequence[i_s] as usize { '.' } else {input_bytes[(i_n - sequence[i_s] as usize)] as char};

                    // read backwards and see if we satisfy number
                    let mut prefix = input_bytes[(i_n + 1 - sequence[i_s] as usize)..i_n].iter();
                    if prefix.all(|x| *x as char == '#' || *x as char == '?') && !is_used && char_before != '#' {
                        // invariant: i_s - 1 must be non-neg
                        let &case_hash = combos.get(&(i_s as i32 - 1, i32::max(i_n as i32 - sequence[i_s] - is_first, -1))).unwrap_or(&0);
                        println!("Get ({}, {})", i_s as i32 - 1, i_n as i32 - sequence[i_s] - is_first);
                        combos.insert(cur_pair, case_hash + case_dot);
                        println!("Insert ({}, {}), {}", cur_pair.0, cur_pair.1, case_hash + case_dot);
                    } else {
                        // if we can't insert hash here, then just dot.
                        combos.insert(cur_pair, case_dot);
                        println!("Insert dot ({}, {}), {}", cur_pair.0, cur_pair.1, case_dot);
                    }
                }
                '.' => {
                    combos.insert(cur_pair, case_dot);
                    println!("Insert ({}, {}), {}", cur_pair.0, cur_pair.1, case_dot);
                }
                '#' => {
                    // if it's a hash, we read backwards and see if we can satisfy the number
                    let prefix = &input_bytes[(i_n + 1 - sequence[i_s] as usize)..i_n];
                    println!("Prefix: {:?} / is_used: {}", &input[(i_n + 1 - sequence[i_s] as usize)..i_n], is_used);
                    if prefix.iter().all(|x| *x as char == '#' || *x as char == '?') && !is_used {
                        // invariant: i_s - 1 must be non-neg
                        // let &case_hash = combos.get(&(i_s as i32 - 1, i_n as i32 - sequence[i_s] - is_first)).unwrap_or(&0);
                        let &case_hash = combos.get(&(i_s as i32 - 1, i32::max(i_n as i32 - sequence[i_s] - is_first, -1))).unwrap_or(&0);
                        println!("Get ({}, {})", i_s as i32 - 1, i_n as i32 - sequence[i_s] - is_first);
                        combos.insert(cur_pair, case_hash);
                        println!("Insert ({}, {}), {}", cur_pair.0, cur_pair.1, case_hash);
                        println!("{}, {:?}", prefix.iter().all(|x| *x as char == '#'), &prefix);
                        is_used = prefix.iter().all(|x| *x as char == '#');
                    } else {
                        // if we can't pop, then this is impossible.
                        combos.insert(cur_pair, 0);
                    }
                }
                _ => unreachable!()
            };

            println!("-----");
        }
    }

    println!("{}, {}", input_bytes.len() - 1, sequence.len() - 1);
    println!("{:?}", combos);
    // return 1

    return *combos.get(&(sequence.len() as i32 - 1, input_bytes.len() as i32 - 1)).unwrap();
}

pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            let mut result = 0;
            for line in file_content.lines() {
                let mut splitted = line.split_ascii_whitespace();
                let line = splitted.next().unwrap();
                let sequence = splitted.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

                //
                let test = get_combos(line, sequence);
                result += test;
                println!("{} - {}", line, test);
            }

            println!("Final Answer {}", result)
        },
        Err(e) => {
            println!("BRUH, {}", e)
        }
    }
}
