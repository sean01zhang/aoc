use std::fs;

#[derive(Debug)]
struct Round {
    blue: i32,
    red: i32,
    green: i32
}

impl Round {
    fn add(&mut self, color: char, val: i32) -> usize {
        match color {
            'r' => {self.red += val; 3 }
            'g' => {self.green += val; 5 }
            'b' => {self.blue += val; 4 }
            _ => {println!("BRUH") ;0}
        }
    }

    fn illegal(&self) -> bool {
        self.red > 12 || self.green  > 13 || self.blue > 14
    }

    fn reset(&mut self) {
        self.red = 0;
        self.green = 0;
        self.blue = 0;
    }
}

pub fn p1(filepath : &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let mut sum_ids = 0;

            // break up the lines
            for (line_num, line) in input.split("\n").enumerate() {
                let chars: Vec<char> = line.chars().collect();

                let mut i = 7 + (line_num + 1).to_string().len();
                let mut fwd = i;
                let mut r : Round = Round{blue: 0, red: 0, green:0};

                while !r.illegal() {
                    if fwd >= line.len() {
                        sum_ids += line_num + 1;
                        break;
                    }

                    match chars[fwd] {
                        ' ' => {
                            let val = line[i..fwd].parse().unwrap();
                            fwd = fwd + r.add(chars[fwd+1], val) + 1;
                            i = fwd + 2 
                        },
                        ',' => fwd = i + 1,
                        ';' => {
                            r.reset();
                            fwd = i + 1;
                        }
                        _ => fwd += 1
                    }

                }
            }

            println!("D2P1: {}", sum_ids)
        },
        Err(err) => {
            println!("Bruh {:?}", err)
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let mut sum_ids = 0;

            // break up the lines
            for (line_num, line) in input.split("\n").enumerate() {
                let chars: Vec<char> = line.chars().collect();

                let mut i = 7 + (line_num + 1).to_string().len();
                let mut fwd = i;

                let mut max_r = 0;
                let mut max_g = 0;
                let mut max_b = 0;

                loop {
                    if fwd >= line.len() {
                        sum_ids += max_r * max_g * max_b;
                        break;
                    }

                    match chars[fwd] {
                        ' ' => {
                            let val = line[i..fwd].parse().unwrap();
                            fwd += 1 + match chars[fwd+1] {
                                'r' => {
                                    max_r = i32::max(val, max_r); 3
                                }
                                'g' => {
                                    max_g = i32::max(val, max_g); 5
                                }
                                'b' => {
                                    max_b = i32::max(val, max_b); 4
                                }
                                _ => {
                                    println!("Bruh"); 0
                                }
                            };

                            i = fwd + 2 
                        },
                        ',' => fwd = i + 1,
                        ';' => {
                            fwd = i + 1;
                        }
                        _ => fwd += 1
                    }

                }
            }

            println!("D2P2: {}", sum_ids)
        },
        Err(err) => {
            println!("Bruh {:?}", err)
        }
    }
}