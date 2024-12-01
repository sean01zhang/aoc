use std::{fs, collections::HashMap, cmp::Ordering};

#[derive(Debug)]
struct Hand {
    serial: String,
    cards: [i64; 5],
    meta_rank: i64,
    bid: i64,
    rank: i64
}

impl Hand {
    fn from(input_str: &str) -> Hand {
        let mut params = input_str.split_ascii_whitespace();
        let hand_string = params.next().unwrap();
        let bid = params.next().unwrap().parse::<i64>().unwrap();
        let mut cards = [0; 5];
        let mut unique_cards : HashMap<i64, i64> = HashMap::new();

        hand_string.as_bytes().into_iter().enumerate().for_each(|(i, c)| {
            let card_face = match *c as char {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                _ => *c as u8 - '2' as u8
            } as i64;

            cards[i] = card_face;
            unique_cards.insert(card_face, unique_cards.get(&card_face).unwrap_or(&0) + 1);
        });

        // determine the metarank
        let meta_rank: i64 = match unique_cards.len() {
            5 => 0, // high card
            4 => 1, // one pair
            3 => { // either two pair or 3 of a kind
                match *unique_cards.values().max().unwrap() {
                    2 => 2, // two pair
                    3 => 3, // three of a kind
                    _ => unreachable!()
                }
            },
            2 => { // either full house or 4 of a kind
                match *unique_cards.values().max().unwrap() {
                    3 => 4, // full house
                    4 => 5, // four of a kind
                    _ => unreachable!()
                }
            },
            1 => 6, // 5 of a kind
            _ => unreachable!()
        };

        let rank = cards.into_iter().rev().enumerate().fold(0, |acc, (i, x)| {
            acc + x * i64::pow(13, i as u32)
        }) + meta_rank * i64::pow(13, 5);

        Hand{cards, bid, rank, serial: String::from(hand_string), meta_rank}
    }

    fn from_p2(input_str: &str) -> Hand {
        let mut params = input_str.split_ascii_whitespace();
        let hand_string = params.next().unwrap();
        let bid = params.next().unwrap().parse::<i64>().unwrap();
        let mut cards = [0; 5];
        let mut unique_cards : HashMap<i64, i64> = HashMap::new();

        let mut num_jokers = 0;
        hand_string.as_bytes().into_iter().enumerate().for_each(|(i, c)| {
            let card_face = match *c as char {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'T' => 9,
                'J' => 0,
                _ => *c as u8 - '2' as u8 + 1,
            } as i64;

            cards[i] = card_face;
            if card_face != 0 {
                unique_cards.insert(card_face, unique_cards.get(&card_face).unwrap_or(&0) + 1);
            } else {
                num_jokers += 1;
            }
        });

        // determine the metarank
        let meta_rank: i64 = match unique_cards.len() {
            5 => 0, // high card
            4 => 1, // one pair
            3 => { // either two pair or 3 of a kind
                match *unique_cards.values().max().unwrap() + num_jokers {
                    2 => 2, // two pair
                    3 => 3, // three of a kind
                    _ => unreachable!()
                }
            },
            2 => { // either full house or 4 of a kind
                match *unique_cards.values().max().unwrap() + num_jokers {
                    3 => 4, // full house
                    4 => 5, // four of a kind
                    _ => unreachable!()
                }
            },
            1 => 6, // 5 of a kind
            0 => 6, // 5 Jokers
            _ => unreachable!()
        };

        let rank = cards.into_iter().rev().enumerate().fold(0, |acc, (i, x)| {
            acc + x * i64::pow(13, i as u32)
        }) + meta_rank * i64::pow(13, 5);

        Hand{cards, bid, rank, serial: String::from(hand_string), meta_rank}
    }
}

pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let mut hands: Vec<Hand> = Vec::new();

            for line in input.lines() {
                hands.push(Hand::from(line));
            }

            hands.sort_by(|a, b| {
                if a.rank == b.rank {
                    Ordering::Equal
                } else if a.rank < b.rank {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let mut other_result = 0;
            for (i, hand) in hands.iter().enumerate() {
                other_result += (i + 1) as i64 * hand.bid;
            }

            let result = hands.into_iter().enumerate().fold(0, |acc, (i, hand)| {
                println!("HAND {}: {:?}", i+1, hand);
                return ((i + 1) as i64) * hand.bid + acc
            });

            println!("D7P1: {} / {}", result, other_result)
        },
        Err(e) => {
            println!("Bruh {}", e)
        }
    }
}

pub fn p2(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(input) => {
            let mut hands: Vec<Hand> = Vec::new();

            for line in input.lines() {
                hands.push(Hand::from_p2(line));
            }

            hands.sort_by(|a, b| {
                if a.rank == b.rank {
                    Ordering::Equal
                } else if a.rank < b.rank {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let mut result = 0;
            for (i, hand) in hands.iter().enumerate() {
                result += (i + 1) as i64 * hand.bid;
            }

            println!("D7P2: {}", result)
        },
        Err(e) => {
            println!("Bruh {}", e)
        }
    }
}