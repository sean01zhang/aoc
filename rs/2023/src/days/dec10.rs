use std::{fs};

fn seek(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn unseek(i: usize, width: usize) -> (usize, usize) {
    ((i % width), (i / width))
}

fn find_adj(bytes: &[u8], i: usize, width: usize) -> [usize; 2] {
    match bytes[i] as char {
        '|' => [i - width, i + width],
        '-' => [i + 1, i - 1],
        'L' => [i - width, i + 1],
        'J' => [i - width, i - 1],
        '7' => [i - 1, i + width],
        'F' => [i + width, i + 1],
        'S' => {
            println!("What the fuck");
            unreachable!()
        }
        '.' => unreachable!(),
        _ => {
            println!("What the fuck {}", bytes[i] as char);
            unreachable!()
        }
    }
}

pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            let bytes = file_content
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("");
            let bytes = bytes.as_bytes();
            let width = file_content.lines().next().unwrap().len();
            let i_s = bytes.iter().position(|x| *x as char == 'S').unwrap();
            let (x, y) = unseek(i_s, width);
            let (x, y) = (x as i32, y as i32);

            // find where to go next
            let candidates: Vec<usize> = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
                .into_iter()
                .filter(|(x, y)| {
                    if *x >= 0 && *y >= 0 {
                        let i_can = seek(*x as usize, *y as usize, width);
                        i_can < bytes.len()
                            && (*x as usize) < width
                            && bytes[i_can] != '.' as u8
                            && find_adj(bytes, i_can, width).contains(&i_s)
                    } else {
                        false
                    }
                })
                .map(|(x, y)| seek(x as usize, y as usize, width))
                .collect();

            println!("Candidates {:?}", candidates);

            let mut count = 1;
            let mut cur = candidates[0];
            let mut prev = i_s;
            while cur != i_s {
                println!("{} => {}", cur, bytes[cur] as char);
                let test = find_adj(bytes, cur, width)
                    .into_iter()
                    .filter(|x| *x != prev)
                    .collect::<Vec<usize>>();
                println!("Adjacents: {:?}", test);
                prev = cur;
                cur = test[0];
                count += 1;
            }

            println!("D10P1: {}", count / 2);
        }
        Err(e) => {
            println!("Bruh {}", e);
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            // p1
            let bytes = file_content
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("");
            let bytes = bytes.as_bytes();
            let width = file_content.lines().next().unwrap().len();
            let i_s = bytes.iter().position(|x| *x as char == 'S').unwrap();
            let (x, y) = unseek(i_s, width);
            let (x, y) = (x as i32, y as i32);

            // find where to go next
            let offsets: Vec<(i32, i32)> = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
                .into_iter()
                .filter(|(x, y)| {
                    if *x >= 0 && *y >= 0 {
                        let i_can = seek(*x as usize, *y as usize, width);
                        i_can < bytes.len()
                            && (*x as usize) < width
                            && bytes[i_can] != '.' as u8
                            && find_adj(bytes, i_can, width).contains(&i_s)
                    } else {
                        false
                    }
                })
                .collect();
            let candidates: Vec<usize> = offsets
                .iter()
                .map(|(x, y)| seek(*x as usize, *y as usize, width))
                .collect();

            println!("Candidates {:?}", candidates);
            let mut ouroboros = vec![candidates[0]];

            let mut prev = i_s;
            while ouroboros[ouroboros.len() - 1] != i_s {
                let test = find_adj(bytes, ouroboros[ouroboros.len() - 1], width)
                    .into_iter()
                    .filter(|x| *x != prev)
                    .collect::<Vec<usize>>();
                // println!("Adjacents: {:?}", test);
                prev = ouroboros[ouroboros.len() - 1];
                ouroboros.push(test[0]);
            }

            let mut is_enclosed = vec![false; bytes.len()];
            let mut is_out = true;
            let mut last_joint = '-';
            for (i, c) in bytes.iter().enumerate() {
                if i % width == 0 {
                    is_out = true;
                }
                let mut cur_char = *c as char;
                // get actual pipe instead of S
                if cur_char == 'S' {
                    cur_char = if candidates[0] as i32 == i_s as i32 + 1 {
                        if candidates[1] as i32 == i_s as i32 + width as i32 {
                            'F'
                        } else if candidates[1] as i32 == i_s as i32 - 1 {
                            '-'
                        } else {
                            'L'
                        }
                    } else if candidates[0] as i32 == i_s as i32 + width as i32 {
                        if candidates[1] as i32 == i_s as i32 - 1 {
                            '7'
                        } else {
                            '|'
                        }
                    } else {
                        'J'
                    }
                }

                if ouroboros.contains(&i) {
                    match cur_char {
                        '-' => (),
                        'F' => {
                            last_joint = 'F';
                            is_out = !is_out;
                        }
                        'L' => {
                            last_joint = 'L';
                            is_out = !is_out;
                        }
                        '7' => {
                            if last_joint == 'L' {
                                last_joint = '7'
                            } else {
                                is_out = !is_out;
                            }
                        }
                        'J' => {
                            if last_joint == 'F' {
                                last_joint = 'J'
                            } else {
                                is_out = !is_out;
                            }
                        }
                        _ => {
                            is_out = !is_out;
                        }
                    }
                } else if !is_out {
                    is_enclosed[i] = true
                }
            }

            // vis
            for (i, byte) in bytes.iter().enumerate() {
                let cur_c = *byte as char;

                if is_enclosed[i] {
                    print!("I")
                } else if ouroboros.contains(&i) {
                    print!(
                        "{}",
                        match cur_c {
                            '|' => '║',
                            '-' => '═',
                            'L' => '╚',
                            'J' => '╝',
                            '7' => '╗',
                            'F' => '╔',
                            'S' => 'S',
                            '.' => unreachable!(),
                            _ => {
                                println!("What the fuck {}", bytes[i] as char);
                                unreachable!()
                            }
                        }
                    );
                } else {
                    print!(".");
                }

                if i % width == width - 1 {
                    println!("")
                }
            }

            let total_enc = is_enclosed
                .iter()
                .fold(0, |acc, x| (if *x { acc + 1 } else { acc }));

            println!("D10P2: {}", total_enc);
        }
        Err(e) => {
            println!("Bruh {}", e)
        }
    }
}
