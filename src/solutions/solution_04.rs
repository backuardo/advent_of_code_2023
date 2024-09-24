use crate::util::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    // Preprocess matches into a HashMap
    // We could use a Vec here since card numbers are sequential
    let card_matches: HashMap<i32, usize> = read_lines("inputs/input_04.txt")
        .unwrap()
        .flatten()
        .enumerate()
        .map(|(i, line)| {
            // Split line on '|'
            let parts: Vec<&str> = line.split('|').collect();

            // Parse winning numbers into hashset
            let winning_nums: HashSet<i32> = parts[0]
                .trim()
                .split_whitespace()
                .skip(1) // Skip "Card X:"
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            // Calculate number of matches
            let num_matches = parts[1]
                .trim()
                .split_whitespace()
                .filter(|num| {
                    num.parse::<i32>()
                        .ok()
                        // Check if we have a match
                        .map_or(false, |n| winning_nums.contains(&n))
                })
                .count();

            // Insert matches into map
            (i as i32 + 1, num_matches)
        })
        .collect();

    // Setup breadth-first search over won cards
    let mut total_cards = 0;
    let mut bfs_queue: VecDeque<i32> = VecDeque::from_iter(card_matches.keys().cloned());

    // Breadth-first search traversal
    while let Some(current_card) = bfs_queue.pop_front() {
        total_cards += 1;

        if let Some(&num_matches) = card_matches.get(&current_card) {
            for i in 1..=num_matches {
                let next_card = current_card + i as i32;
                if next_card <= card_matches.len() as i32 {
                    bfs_queue.push_back(next_card);
                }
            }
        }
    }

    println!("Solution: {}", total_cards);
}
