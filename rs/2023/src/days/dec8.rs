use std::{collections::HashMap, fs};


struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let mut nodes: HashMap<&str, Node> = HashMap::new();

            let mut lines = input.lines();
            let turns = lines.next().unwrap();
            // skip empty line
            lines.next();

            // process input into a graph
            for line in lines {
                nodes.insert(&line[0..3], Node{left: &line[7..10], right: &line[12..15]});
            }

            let mut cur_node = "AAA";
            let mut steps = 0;
            while cur_node != "ZZZ" {
                for turn in turns.chars() {
                    steps += 1;
                    cur_node = match turn {
                        'L' => nodes[cur_node].left,
                        'R' => nodes[cur_node].right,
                        _ => unreachable!()
                    };

                    if cur_node == "ZZZ" {
                        break;
                    } 
                }
            }

            println!("D8P1: {}", steps);
        },
        Err(e) => {
            println!("Bruh {:?}", e)
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let mut nodes: HashMap<&str, Node> = HashMap::new();

            let mut lines = input.lines();
            let turns = lines.next().unwrap();
            // skip empty line
            lines.next();

            let mut starting_points: Vec<&str> = Vec::new();
            // process input into a graph
            for line in lines {
                nodes.insert(&line[0..3], Node{left: &line[7..10], right: &line[12..15]});
                if line.as_bytes()[2] == 'A' as u8 {
                    starting_points.push(&line[0..3]);
                }
            }

            println!("Starting points: {:?}", starting_points);

            let mut steps_taken = 1;
            for starting_point in starting_points {
                let mut cur_node = starting_point;
                let mut steps = 0;
                while !cur_node.ends_with('Z') {
                    for turn in turns.chars() {
                        steps += 1;
                        cur_node = match turn {
                            'L' => nodes[cur_node].left,
                            'R' => nodes[cur_node].right,
                            _ => unreachable!()
                        };

                        if cur_node.ends_with('Z') {
                            break;
                        } 
                    }
                }

                steps_taken = lcm(steps_taken, steps)
            }

            println!("D8P2: {}", steps_taken);
        },
        Err(e) => {
            println!("Bruh {:?}", e)
        }
    }
}

