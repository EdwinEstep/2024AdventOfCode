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
    let mut num_safe: u32 = 0;

    // parse the input into left and right vecs
    for line in reader.lines() {
        let lstr = line.unwrap().clone();
        let report: Vec<&str> = lstr.split_whitespace().collect();

        let safety = is_safe(&report);
        if safety {
            num_safe += 1;
            println!(" SAFE! ");
        } else {
            println!(" NOT! ");
        }
    }



    println!("{}", num_safe);
    Ok(())
}

fn is_safe(report: &Vec<&str>) ->  bool {
    let mut increasing: bool = false;
    let mut decreasing: bool = false;
    let mut safe: bool = true;

    for index in 0..(report.len()-1) {
        let num1 = report[index].parse::<u32>().unwrap();
        let num2 = report[index+1].parse::<u32>().unwrap();
        let diff = num1.abs_diff(num2);

        print!("{num1} {num2} | ");

        if num1 == num2 {
            safe = false;
        } else {
            if diff == 0 || diff > 3 {
                safe = false;
            }

            if num1 < num2 {
                increasing = true;

                if decreasing {
                    safe = false;
                }
            } else {
                decreasing = true;

                if increasing {
                    safe = false;
                }
            }
        }
    }

    return safe;
}
