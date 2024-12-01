use std::fs;

pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            let universe: Vec<&str> = file_content.lines().collect();
            let mut free_cols = vec![true; universe[0].len()];
            let mut free_rows = vec![true; universe.len()];
            let mut galaxy_coords : Vec<(usize, usize)> = Vec::new();

            // for each number, determine coordinates and which col/row it occupies
            for (i, line) in universe.iter().enumerate() {
                for (j, c) in line.as_bytes().iter().enumerate() {
                    let c = *c as char;
                    if c == '#' {
                        free_rows[i] = false;
                        free_cols[j] = false;
                    }
                }
            }

            // track where the galaxies are
            let mut sum_shortest_paths = 0;
            let mut i_shift = 0;
            for (i, line) in universe.iter().enumerate() {
                let mut j_shift = 0;
                for (j, c) in line.as_bytes().iter().enumerate() {
                    let c = *c as char;
                    if c == '#' {
                        // go down the list of current galaxy coords and get the distances
                        for (g_i, g_j) in galaxy_coords.iter() {
                            sum_shortest_paths += usize::abs_diff(i_shift, *g_i) 
                                + usize::abs_diff(j_shift, *g_j);
                        }

                        galaxy_coords.push((i_shift, j_shift));
                    }
                    j_shift += if free_cols[j] { 2 } else { 1 };
                }
                i_shift += if free_rows[i] { 2 } else { 1 };
            }

            println!("D11P1: {}", sum_shortest_paths)
        },
        Err(e) => {
            println!("Bruh {}", e)
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            let universe: Vec<&str> = file_content.lines().collect();
            let mut free_cols = vec![true; universe[0].len()];
            let mut free_rows = vec![true; universe.len()];
            let mut galaxy_coords : Vec<(usize, usize)> = Vec::new();

            // for each number, determine coordinates and which col/row it occupies
            for (i, line) in universe.iter().enumerate() {
                for (j, c) in line.as_bytes().iter().enumerate() {
                    let c = *c as char;
                    if c == '#' {
                        free_rows[i] = false;
                        free_cols[j] = false;
                    }
                }
            }

            // track where the galaxies are
            let mut sum_shortest_paths = 0;
            let mut i_shift = 0;
            for (i, line) in universe.iter().enumerate() {
                let mut j_shift = 0;
                for (j, c) in line.as_bytes().iter().enumerate() {
                    let c = *c as char;
                    if c == '#' {
                        // go down the list of current galaxy coords and get the distances
                        for (g_i, g_j) in galaxy_coords.iter() {
                            sum_shortest_paths += usize::abs_diff(i_shift, *g_i) 
                                + usize::abs_diff(j_shift, *g_j);
                        }

                        galaxy_coords.push((i_shift, j_shift));
                    }
                    j_shift += if free_cols[j] { 1000000 } else { 1 };
                }
                i_shift += if free_rows[i] { 1000000 } else { 1 };
            }

            println!("D11P2: {}", sum_shortest_paths)
        },
        Err(e) => {
            println!("Bruh {}", e)
        }
    }
}