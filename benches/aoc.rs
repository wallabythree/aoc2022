use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use rudolf_rs;
use solutions::{day00, day01, day02, day03, day04, day05, day06, day07};

fn criterion_benchmark(c: &mut Criterion) {
    let session_key = env::var("AOC_SESSION").unwrap();
    let client = rudolf_rs::Client::new(String::from(session_key));

    c.bench_function(
        "day00part1",
        |b| {
            b.iter(|| day00::part1(black_box("0")))
        }
    );

    c.bench_function(
        "day00part2",
        |b| {
            b.iter(|| day00::part1(black_box("")))
        }
    );

    let mut input = client.get(2022, 1).unwrap();

    c.bench_function(
        "day01part1",
        |b| {
            b.iter(|| day01::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day01part2",
        |b| {
            b.iter(|| day01::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 2).unwrap();

    c.bench_function(
        "day02part1",
        |b| {
            b.iter(|| day02::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day02part2",
        |b| {
            b.iter(|| day02::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 3).unwrap();

    c.bench_function(
        "day03part1",
        |b| {
            b.iter(|| day03::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day03part2",
        |b| {
            b.iter(|| day03::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 4).unwrap();

    c.bench_function(
        "day04part1",
        |b| {
            b.iter(|| day04::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day04part2",
        |b| {
            b.iter(|| day04::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 5).unwrap();

    c.bench_function(
        "day05part1",
        |b| {
            b.iter(|| day05::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day05part2",
        |b| {
            b.iter(|| day05::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 6).unwrap();

    c.bench_function(
        "day06part1",
        |b| {
            b.iter(|| day06::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day06part2",
        |b| {
            b.iter(|| day06::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 7).unwrap();

    c.bench_function(
        "day07part1",
        |b| {
            b.iter(|| day07::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day07part2",
        |b| {
            b.iter(|| day07::part2(black_box(&input)))
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

