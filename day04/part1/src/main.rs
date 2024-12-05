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
        for (k, _letter) in line.iter().enumerate() {
            num_xmas += num_sequences(&word_search, "XMAS", (j, k))
        }
    }

    println!("{}", num_xmas);
    Ok(())
}

// start at the value given by coords and see if you can word search to find the sequence
// because we're only finding straight up-down-left-right-diagonal words, we only need to 
// check the perimeter around the first value in the sequence
fn num_sequences(search_data: &Vec<Vec<char>>, sequence: &str, coords: (usize, usize)) ->  u32 {
    let mut count = 0;
    println!("\nstarting coords: {:?}", coords);
    
    if search_data[coords.0][coords.1] != sequence.chars().nth(0).unwrap() {
        return 0;
    }

    // matches first char in sequence. now find a list of the secondth characters, to get 
    // trajectories
    for coords2 in adjacent_next(search_data, coords, sequence.chars().nth(1).unwrap()) {
        let dir = (coords2.0 as isize - coords.0 as isize, coords2.1 as isize - coords.1 as isize);
        let mut new_coords = (coords2.0 as isize + dir.0, coords2.1 as isize + dir.1);
        let mut is_seq = true;

        println!("coords2: {:?}, dir: {:?}", coords2, dir);
        
        // keep going in that direction
        for letter in sequence[2..].chars() {
            println!("new coords: {:?}", new_coords);
            if new_coords.0 < 0 || new_coords.1 < 0 || new_coords.0 > (search_data.len() as isize - 1) || new_coords.1 > (search_data[0].len() as isize - 1) {
                is_seq = false;
                print!("ooga, sequence.len() as isize - 1: {}", (sequence.len() as isize - 1));
            }

            if is_seq && search_data[new_coords.0 as usize][new_coords.1 as usize] != letter {
                print!("booga");
                is_seq = false;
            } else {
                new_coords = (new_coords.0 + dir.0, new_coords.1 + dir.1);
            }
        }

        if is_seq { count += 1; }
    }

    return count;
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
