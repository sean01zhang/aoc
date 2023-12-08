use std::{fs};


pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input_str) => {
            let mut lines = input_str.lines();
            let times = lines.next().unwrap()[11..].split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap());
            let distances = lines.next().unwrap()[11..].split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap());

            let mut result = 1;
            for (time, dist) in times.zip(distances) {
                let lower = f32::ceil((time as f32 - f32::sqrt((time * time - 4 * (dist+1)) as f32)) / 2.) as i32;
                let upper = f32::floor((time as f32 + f32::sqrt((time * time - 4 * (dist+1)) as f32)) / 2.) as i32;
                result *= upper - lower + 1;
            }

            println!("D5P1: {}", result)
        }
        Err(e) => {
            println!("Bruh {:?}", e)
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input_str) => {
            let mut lines = input_str.lines();

            let mut time: i64 = 0;
            lines.next().unwrap()[11..].as_bytes().iter().for_each(|c| {
                if (*c as char).is_numeric() {
                    time = time * 10 + (c - '0' as u8) as i64
                }
            });
            let mut dist = 0;
            lines.next().unwrap()[11..].as_bytes().iter().for_each(|c| {
                if (*c as char).is_numeric() {
                    dist = dist * 10 + (c - '0' as u8) as i64
                }
            });

            let mut result = 1;
            let lower = f64::ceil((time as f64 - f64::sqrt((time * time - 4 * (dist+1)) as f64)) / 2.) as i64;
            let upper = f64::floor((time as f64 + f64::sqrt((time * time - 4 * (dist+1)) as f64)) / 2.) as i64;
            result *= upper - lower + 1;

            println!("D5P2: {}", result)
        }
        Err(e) => {
            println!("Bruh {:?}", e)
        }
    }
}