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
    let mut dist_sum: i32 = 0;
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    // parse the input into left and right vecs
    for line in reader.lines() {
        let lstr = line.unwrap().clone();

        let mut left_right = lstr.split_whitespace();
        left_list.push(left_right.next().unwrap().parse::<i32>().unwrap());
        right_list.push(left_right.next().unwrap().parse::<i32>().unwrap());
    }

    // sort, to enable easy comparison
    left_list.sort();
    right_list.sort();

    // get the distances and done
    for index in 0..left_list.len() {
        dist_sum += (left_list[index] - right_list[index]).abs();
    }

    println!("{}", dist_sum);
    Ok(())
}