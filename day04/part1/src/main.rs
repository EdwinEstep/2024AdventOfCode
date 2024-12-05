// `cargo run ../input.txt`

use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);

    let mut num_xmas: u32 = 0;
    let mut word_search: Vec<Vec<char>> = vec![];

    // parse & print
    for line in reader.lines() {
        word_search.push(line?.clone().chars().collect());
    }

    // find xmas!
    for (j, line) in word_search.iter().enumerate() {
        for (k, letter) in line.iter().enumerate() {
            // check if letter starts a sequence
            if is_sequence(&word_search, "XMAS", (j, k)) {
                num_xmas += 1;
            }
        }
    }

    println!("{}", num_xmas);
    Ok(())
}

// start at the value given by coords and see if you can word search to find the sequence
// because we're only finding straight up-down-left-right-diagonal words, we only need to 
// check the perimeter around the first value in the sequence
fn is_sequence(search_data: &Vec<Vec<char>>, sequence: &str, coords: (usize, usize)) ->  bool {
    if search_data[coords.0][coords.1] != sequence.chars().nth(0).unwrap() {
        return false;
    }

    println!("{:?}", coords);

    // matches first char in sequence. now find a list of the secondth characters, to get 
    // trajectories
    for coords2 in adjacent_next(search_data, coords, sequence.chars().nth(1).unwrap()) {
        // let direction = coords - coords2;
        // let new_coords = coords2.try_into().unwrap() + direction;
        
        // // keep going in that direction
        // for letter in sequence[2..] {
        //     if search_data[new_coords.0][new_coords.1] != letter {
        //         return false;
        //     } else {
        //         new_coords = new_coords + direction;
        //     }
        // }
    }

    return true;
}

// returns a list of coordinates to search for next in the series
fn adjacent_next(search_data: &Vec<Vec<char>>, coords: (usize, usize), next_letter: char) ->  Vec<(usize, usize)> {
    let max_row = search_data.len() - 1;
    let max_col = search_data[0].len() - 1;

    // avoid bad array accesses
    let top = if coords.0 > 0 {
        coords.0 - 1
    } else {
        0
    };
    let bottom = if coords.0 < max_row {
        coords.0 + 1
    } else {
        max_row
    };
    let left = if coords.1 > 0 {
        coords.1 - 1
    } else {
        0
    };
    let right = if coords.1 < max_col {
        coords.1 + 1
    } else {
        max_col
    };

    let mut next_searches: Vec<(usize, usize)> = vec![];
    for j in top..=bottom {
        for k in left..=right {
            if search_data[j][k] == next_letter && coords != (j, k) {
                next_searches.push((j, k));
            }
        }
    }

    return next_searches;
}
