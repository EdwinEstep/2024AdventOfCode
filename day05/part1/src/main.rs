// `cargo run ../input.txt`

use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum InputSection {
    ORDERINGS,
    UPDATES,
}

use crate::InputSection::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);

    let mut mid_page_nums = 0;
    let mut orderings: Vec<Vec<u32>> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut input_section = ORDERINGS;

    // parse & print
    // I don't like how I'm separating the sections
    // for parsing.
    for line in reader.lines() {
        let line = line.unwrap().to_string().clone();

        match input_section {
            ORDERINGS => {
                if line != String::new() {
                    let splitted = line.split("|");
                    let mut rule: Vec<u32> = vec![];

                    for thing in splitted {
                        rule.push(thing.parse::<u32>().unwrap());
                    }
                    orderings.push(rule);
                } else {
                    input_section = UPDATES;
                }
            },
            UPDATES => {
                let splitted = line.split(",");
                let mut page: Vec<u32> = vec![];

                for thing in splitted {
                    page.push(thing.parse::<u32>().unwrap());
                }
                updates.push(page);
            },
        }
    }


    // the plan:  search through the pages in each update and get the list of 
    //   rules where the second element in the rule is in the page list.
    for update in &updates {
        let mut local_rules = vec![];

        for page in update {
            for rule in &orderings {
                if page == &rule[1] {
                    local_rules.push(rule);
                }
            }
        }

        let mut to_remove = vec![];
        for (index, rule) in local_rules.clone().iter().enumerate() {
            if !(update.contains(&rule[0])) {
                to_remove.push(index);
            }
        }

        for &index in to_remove.iter().rev() {
            local_rules.remove(index);
        }

        println!("local rules:  {:?}", local_rules);


        // now check rules
        let mut good = true;
        for rule in local_rules {
            if !passes_rule_check(rule, update) {
                good = false;
            }
        }

        if good {
            mid_page_nums += update[update.len()/2];
        }
    }

    println!("{:?}", orderings);
    println!("{:?}", updates);



    println!("{}", mid_page_nums);
    Ok(())
}


fn passes_rule_check(rule: &Vec<u32>, update: &Vec<u32>) -> bool {
    for page in update {
        if *page == rule[1] {
            return false;
        } else if *page == rule[0] {
            return true;
        }
    }

    return true;
}
