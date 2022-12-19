use std::env;

use solutions::day20::{part1, part2};

fn main() {
    let session_key = env::var("AOC_SESSION").unwrap();
    let client = rudolf_rs::Client::new(session_key);
    let input = client.get(2022, 19).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

