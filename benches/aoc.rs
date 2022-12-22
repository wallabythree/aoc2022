use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use rudolf_rs;
use solutions::{
    day00,
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
};

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

    input = client.get(2022, 8).unwrap();

    c.bench_function(
        "day08part1",
        |b| {
            b.iter(|| day08::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day08part2",
        |b| {
            b.iter(|| day08::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 9).unwrap();

    c.bench_function(
        "day09part1",
        |b| {
            b.iter(|| day09::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day09part2",
        |b| {
            b.iter(|| day09::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 10).unwrap();

    c.bench_function(
        "day10part1",
        |b| {
            b.iter(|| day10::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day10part2",
        |b| {
            b.iter(|| day10::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 11).unwrap();

    c.bench_function(
        "day11part1",
        |b| {
            b.iter(|| day11::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day11part2",
        |b| {
            b.iter(|| day11::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 12).unwrap();

    c.bench_function(
        "day12part1",
        |b| {
            b.iter(|| day12::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day12part2",
        |b| {
            b.iter(|| day12::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 13).unwrap();

    c.bench_function(
        "day13part1",
        |b| {
            b.iter(|| day13::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day13part2",
        |b| {
            b.iter(|| day13::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 14).unwrap();

    c.bench_function(
        "day14part1",
        |b| {
            b.iter(|| day14::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day14part2",
        |b| {
            b.iter(|| day14::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 15).unwrap();

    c.bench_function(
        "day15part1",
        |b| {
            b.iter(|| day15::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day15part2",
        |b| {
            b.iter(|| day15::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 16).unwrap();

    c.bench_function(
        "day16part1",
        |b| {
            b.iter(|| day16::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day16part2",
        |b| {
            b.iter(|| day16::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 17).unwrap();

    c.bench_function(
        "day17part1",
        |b| {
            b.iter(|| day17::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day17part2",
        |b| {
            b.iter(|| day17::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 18).unwrap();

    c.bench_function(
        "day18part1",
        |b| {
            b.iter(|| day18::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day18part2",
        |b| {
            b.iter(|| day18::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 19).unwrap();

    c.bench_function(
        "day19part1",
        |b| {
            b.iter(|| day19::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day19part2",
        |b| {
            b.iter(|| day19::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 20).unwrap();

    c.bench_function(
        "day20part1",
        |b| {
            b.iter(|| day20::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day20part2",
        |b| {
            b.iter(|| day20::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 21).unwrap();

    c.bench_function(
        "day21part1",
        |b| {
            b.iter(|| day21::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day21part2",
        |b| {
            b.iter(|| day21::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 22).unwrap();

    c.bench_function(
        "day22part1",
        |b| {
            b.iter(|| day22::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day22part2",
        |b| {
            b.iter(|| day22::part2(black_box(&input)))
        }
    );

    input = client.get(2022, 23).unwrap();

    c.bench_function(
        "day23part1",
        |b| {
            b.iter(|| day23::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day23part2",
        |b| {
            b.iter(|| day23::part2(black_box(&input)))
        }
    );
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);

