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
    let mut num_bad = 0;

    for index in 0..(report.len() - 2) {
        let num1 = report[index].parse::<u32>().unwrap();
        let num2 = report[index+1].parse::<u32>().unwrap();
        let num3 = report[index+2].parse::<u32>().unwrap();
        let diff12 = num1.abs_diff(num2);
        let diff23 = num2.abs_diff(num3);
        let diff13 = num1.abs_diff(num3);

        print!("{num1} {num2} | ");

        // special case: front, can always remove easily
        if index == 0 && num1 == num2 || diff12 > 3 {
            num_bad += 1;
        }
        // special case: back, can always remove easily
        else if index == (report.len() - 3) && num2 == num3 || diff23 > 3 {
                num_bad += 1;
        }
        // normal case: middle
        else if num1 == num2 {
            num_bad += 1; // recoverable
        } else if diff12 > 3 {
            if diff13 > 3 {
                return false;
            } else {
                if num1 < num3 && increasing {
                    num_bad += 1;
                } else if num1 > num3 && decreasing {
                    num_bad += 1;
                } else {
                    return false;
                }
            }
        } else { // good?
            print!("here");
            if num1 < num2 {
                if decreasing {
                    if num1 == num3 {
                        return false;
                    }
                    num_bad += 1;
                } else {
                    increasing = true;
                    print!("increasing");
                }
            } else {
                if increasing {
                    if num1 == num3 {
                        return false;
                    }
                    num_bad += 1;
                } else {
                    decreasing = true;
                    print!("decreasing");
                }
            }
        }
    }

    return num_bad < 2;
}
