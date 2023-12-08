use std::{fs, collections::HashMap, time::Instant};

use rayon::iter::{IntoParallelIterator, ParallelIterator, IntoParallelRefMutIterator, IndexedParallelIterator};

#[derive(Debug)]
struct RangeMap {
    src_start: i64,
    dst_start: i64,
    n_range: i64
}


pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            let mut iter = file_content.lines();
            // get seed ids
            let mut seed_to_x : HashMap<i64, i64>= HashMap::new();
            let seed_ids = iter.next().unwrap()[7..].split(" ").map(|num_as_str| num_as_str.parse::<i64>().unwrap());
            seed_ids.for_each(|id| {
                seed_to_x.insert(id, id);
            });
            // skip space
            iter.next();

            let mut range_maps: Vec<RangeMap> = Vec::new();
            for line in iter {
                if line.len() == 0 { 
                    // convert all the numbers
                    let seed_to_x_old : HashMap<i64, i64>= seed_to_x.clone();
                    for range_map in &range_maps {
                        for (key, cur_val) in seed_to_x_old.iter() {
                            let xval = *cur_val - range_map.src_start;
                            if xval >= 0 && xval <= range_map.n_range {
                                seed_to_x.insert(*key, range_map.dst_start + xval);
                            }
                        }
                    }

                    println!("CONVERSION RD AFTER: {:?}", seed_to_x);

                    // clear range maps
                    range_maps.clear()
                } else if line.as_bytes()[line.len() - 1] != ':' as u8 {
                    let x: Vec<i64> = line.split(" ").map(|num_as_str| { num_as_str.parse::<i64>().unwrap()}).collect();
                    range_maps.push(RangeMap{dst_start: x[0], src_start: x[1], n_range: x[2]});
                    println!("Add RULE {} {:?}", line, range_maps);
                }             
            }

            let mut min_loc = i64::MAX;
            for value in seed_to_x.values() {
                min_loc = i64::min(min_loc, *value);
            }

            println!("{:?}", min_loc);
        },
        Err(err) => {
            println!("Bruh {:?}", err)
        }
    }
}

pub fn p2_stupid(filepath: &str) {
    let dur = Instant::now();
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            let mut iter = file_content.lines();

            let mut seed_ids: Vec<i64> = iter.next().unwrap()[7..].split(" ").map(|num_as_str| num_as_str.parse::<i64>().unwrap()).collect();

            let mut result : Vec<i64> = Vec::new();
            let mut total = 0;
            for pair in seed_ids.chunks(2) {
                let id = pair[0];
                let count = pair[1] as usize;
                result.resize(total + count, 0);

                result[total..(total+count)].par_iter_mut().enumerate().for_each(|(i, x)| {
                    *x = id + i as i64;
                });

                total += count;
            }

            // skip space
            iter.next();

            // println!("Begin Reading {:?}", result);
            println!("Begin Reading");

            let mut range_maps: Vec<RangeMap> = Vec::new();
            for line in iter {
                if line.len() == 0 { 
                    // convert all the numbers
                    result.par_iter_mut().for_each(|val| {
                        for range_map in &range_maps {
                            let offset= *val - range_map.src_start;
                            if offset >= 0 && offset < range_map.n_range {
                                *val = range_map.dst_start + offset;
                                break;
                            }
                        }
                    });

                    // clear range maps
                    range_maps.clear()
                } else if line.as_bytes()[line.len() - 1] != ':' as u8 {
                    let x: Vec<i64> = line.split(" ").map(|num_as_str| { num_as_str.parse::<i64>().unwrap()}).collect();
                    range_maps.push(RangeMap{dst_start: x[0], src_start: x[1], n_range: x[2]});
                }             
            }

            let mut min_loc = i64::MAX;
            for value in result {
                min_loc = i64::min(min_loc, value);
            }

            println!("{:?}", min_loc);
        },
        Err(err) => {
            println!("Bruh {:?}", err)
        }
    }

    println!("Complete in {} milliseconds", dur.elapsed().as_millis())
}