use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};

fn pair_to_range(input: &str) -> RangeInclusive<u8> {
    let (left, right) = input.split_once('-').unwrap();

    let left_int: u8 = left.parse().unwrap();
    let right_int: u8 = right.parse().unwrap();

    left_int..=right_int
}

#[aoc_generator(day4)]
fn input_to_range(input: &str) -> Vec<(RangeInclusive<u8>, RangeInclusive<u8>)> {
    input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(left, right)| (pair_to_range(left), pair_to_range(right)))
        .collect()
}

#[aoc(day4, part1)]
fn count_containing_pairs(input: &[(RangeInclusive<u8>, RangeInclusive<u8>)]) -> u16 {
    input
        .iter()
        .map(|(left, right)| {
            let left_includes = (left.start() >= right.start()) && (left.end() <= right.end());
            let right_includes = (right.start() >= left.start()) && (right.end() <= left.end());

            (left_includes || right_includes) as u16
        })
        .sum()
}

#[aoc(day4, part2)]
fn count_overlapping_pairs(input: &[(RangeInclusive<u8>, RangeInclusive<u8>)]) -> u16 {
    input
        .iter()
        .map(|(left, right)| {
            println!("{left:?} | {right:?}");

            (dbg!(left.end() >= right.start()) && dbg!(right.start() >= left.end())) as u16
        })
        .sum()
}
