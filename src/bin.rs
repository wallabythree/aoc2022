use std::env;

use rudolf;

use solutions::day01;

fn main() {
    let session_key = env::var("AOC_SESSION").unwrap();

    let client = rudolf::Client::new(session_key);
    let input = client.get(2022, 1).unwrap();
    println!("{}", day01::part1(&input));
    println!("{}", day01::part2(&input));
}

