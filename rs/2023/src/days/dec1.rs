use std::fs;

pub fn p1(filepath : &str) {
    match fs::read_to_string(filepath) {
        Ok(s) => {
            let mut total = 0;
            s.split("\n").for_each(|line| {
                let mut numbers = vec![];
                for char  in line.chars() {
                    if char >= '0' && char <= '9' {
                        numbers.push(char as i32 - '0' as i32);
                    } 
                }
                let line = 10 * numbers[0] + numbers[numbers.len() - 1];
                total += line;
            });
            println!("D1P1: {}", total)
        },
        Err(err) => {
            println!("bruh {:?}", err)
        }
    };


}
pub fn p2(filepath : &str) {
    match fs::read_to_string(filepath) {
        Ok(s) => {
            let mut total = 0;
            s.split("\n").for_each(|line| {
                let mut numbers = vec![];
                for (i , char ) in line.chars().enumerate() {
                    if char >= '0' && char <= '9' {
                        numbers.push(char as i32 - '0' as i32);
                    } else {
                        let out = match &line[i..] {
                            sub if sub.starts_with("one") => Some(1),
                            sub if sub.starts_with("two") => Some(2),
                            sub if sub.starts_with("three") => Some(3),
                            sub if sub.starts_with("four") => Some(4),
                            sub if sub.starts_with("five") => Some(5),
                            sub if sub.starts_with("six") => Some(6),
                            sub if sub.starts_with("seven") => Some(7),
                            sub if sub.starts_with("eight") => Some(8),
                            sub if sub.starts_with("nine") => Some(9),
                            sub if sub.starts_with("zero") => Some(0),
                            _ => None
                        };

                        match out {
                            Some(num) => {
                                numbers.push(num);
                            },
                            None => ()
                        }
                    }
                }
                let line = 10 * numbers[0] + numbers[numbers.len() - 1];
                total += line;
            });
            println!("D1P2: {}", total)
        },
        Err(err) => {
            println!("bruh {:?}", err)
        }
    };
}