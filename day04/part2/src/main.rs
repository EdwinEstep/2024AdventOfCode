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
            println!("({j}, {k})");
            if is_x_mas(&word_search, (j, k)) {
                num_xmas += 1;
                println!(" is X-MAS");
            }
        }
    }

    println!("{}", num_xmas);
    Ok(())
}

// finds X-shaped MASes
fn is_x_mas(search_data: &Vec<Vec<char>>, coords: (usize, usize)) ->  bool {
    let max_row = search_data.len() - 1;
    let max_col = search_data[0].len() - 1;

    if search_data[coords.0][coords.1] != 'A' {
        return false;
    }

    // avoid bad array accesses
    if coords.0 == 0 || coords.0 == max_row {
        return false;
    } else if coords.1 == 0 || coords.1 == max_col {
        return false
    }

    let mut letters: Vec<char> = vec![];
    for j in vec![coords.0 - 1, coords.0 + 1] {
        for k in vec![coords.1 - 1, coords.1 + 1] {
            if search_data[j][k] == 'M' || search_data[j][k] == 'S' && coords != (j, k) {
                letters.push(search_data[j][k]);
            }
        }
    }

    let key: String = letters.iter().collect();
    println!("{key}");

    match key.as_str() {
        "MMSS" => {},
        "MSMS" => {},
        "SSMM" => {}, 
        "SMSM" => {},
        _ => return false,
    }

    return true;
}
