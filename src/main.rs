use std::{env, io::{self, Read}, process::exit};

fn parse_str_to_i32(str: &String) -> Vec<i32> {
    let mut parsed_str = vec![];
    for word in str.split_whitespace() {
        parsed_str.push(word.parse().unwrap());
    }
    parsed_str
}

fn get_skyscraper_rules() -> Vec<i32> {
    let args: Vec<String> = env::args().collect();
    parse_str_to_i32(&args[1])
}

fn get_skyscraper_grid() -> Vec<i32> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    parse_str_to_i32(&buf)
}

fn check_rules(skyscraper_rules: &Vec<i32>) {
    if skyscraper_rules.len() % 4 != 0 {
        println!("invalid rules");
        exit(1);
    }
}

fn check_grid(skyscraper_rules: &Vec<i32>, skyscraper_grid: &Vec<i32>) {
    if (skyscraper_rules.len() / 4) * (skyscraper_rules.len() / 4) != skyscraper_grid.len() {
        println!("invalid grid");
        exit(1);
    }
}

fn main() {
    let skyscraper_rules = get_skyscraper_rules();
    check_rules(&skyscraper_rules);
    let skyscraper_grid = get_skyscraper_grid();
    check_grid(&skyscraper_rules, &skyscraper_grid);
}
