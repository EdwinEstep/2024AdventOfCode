// `cargo run ../input.txt`

// for cmd-line args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line

use regex::Regex;

fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  
    let mut mul_sum: u32 = 0;
    let mut ignore = false;

    // regex to catch all forms of "mul(X,Y)"
    let re1 = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don't\(\))").unwrap();
    let re2 = Regex::new(r"\d+").unwrap();

    // use regex on each line to get the mullllllls
    for line in reader.lines() {
        let lstr = line.unwrap().clone();

        let matches: Vec<_> = re1.find_iter(&lstr).map(|m| m.as_str()).collect();

        for thing in matches {
            print!("{thing} ");

            match thing {
                "do()" => ignore = false,
                "don't()" => ignore = true,
                _ => if (!ignore) {
                    let nums: Vec<_> = re2.find_iter(thing).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
                    mul_sum += nums[0] * nums[1];
                },
            }
        }

        println!("");
    }

    println!("{}", mul_sum);
    Ok(())
}
