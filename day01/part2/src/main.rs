// `cargo run ../input.txt`

// for cmd-line args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line

fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // open file and set up buff reader
    // 8KB buffer size by default.
    // Or init with `with_capacity(num_bytes, file)` instead of `new()`
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  

    // useful vars
    let mut similarity_score: u32 = 0;
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    // parse the input into left and right vecs
    for line in reader.lines() {
        let lstr = line.unwrap().clone();

        let mut left_right = lstr.split_whitespace();
        left_list.push(left_right.next().unwrap().parse::<u32>().unwrap());
        right_list.push(left_right.next().unwrap().parse::<u32>().unwrap());
    }

    // sort, to enable easy comparison
    left_list.sort();
    right_list.sort();

    for left_value in left_list {
        similarity_score += left_value * count_in_sorted_list(left_value, &right_list);
    }

    println!("{}", similarity_score);
    Ok(())
}

fn count_in_sorted_list(key: u32, list: &Vec<u32>) -> u32 {
    let mut count = 0;
    let mut return_next = false;

    for element in list {
        if *element == key {
            count += 1;
            return_next = true;
        } else if return_next {
            return count;
        }
    }

    // handles 0-counts and counting a value that is at the end
    // of the list
    return count;
}