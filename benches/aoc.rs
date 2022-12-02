use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use rudolf;
use solutions::{day00, day01, day02, day03};

fn criterion_benchmark(c: &mut Criterion) {
    let session_key = env::var("AOC_SESSION").unwrap();
    let client = rudolf::Client::new(String::from(session_key));

    let mut input = String::from("0");

    c.bench_function(
        "day00-part1",
        |b| {
            b.iter(|| day00::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day00-part2",
        |b| {
            b.iter(|| day00::part1(black_box(&input)))
        }
    );

    input = client.get(2022, 1).unwrap();

    c.bench_function(
        "day01-part1",
        |b| { 
            b.iter(|| day01::part1(black_box(&input)))
        }
    );
    
    c.bench_function(
        "day01-part2",
        |b| {
            b.iter(|| day01::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 2).unwrap();

    c.bench_function(
        "day02-part1",
        |b| { 
            b.iter(|| day02::part1(black_box(&input)))
        }
    );
    
    c.bench_function(
        "day02-part2",
        |b| {
            b.iter(|| day02::part2(black_box(&input)))
        }
    );

    c.bench_function(
        "day03-part1",
        |b| { 
            b.iter(|| day03::part1(black_box(&input)))
        }
    );

    input = client.get(2022, 3).unwrap();
    
    c.bench_function(
        "day03-part2",
        |b| {
            b.iter(|| day03::part2(black_box(&input)))
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

