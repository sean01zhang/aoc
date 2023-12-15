use std::fs;

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
        },
    }
}

pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            let bytes = file_content.split_ascii_whitespace().collect::<Vec<&str>>().join("");
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
