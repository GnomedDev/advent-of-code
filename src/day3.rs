use std::collections::HashSet;

use aoc_runner_derive::aoc;
use fxhash::FxBuildHasher;
use itertools::Itertools;

fn get_priority(c: u8) -> u8 {
    if (b'a'..=b'z').contains(&c) {
        (c - b'a') + 1
    } else {
        (c - b'A') + 27
    }
}

#[aoc(day3, part1)]
fn get_shared_priorities(input: &str) -> u32 {
    input.lines().fold(0, |mut acc, line| {
        let (left, right) = line.split_at(line.len() / 2);

        left.as_bytes().iter().fold(
            HashSet::with_capacity_and_hasher(left.len(), FxBuildHasher::default()),
            |mut shared, c| {
                if right.as_bytes().contains(c) && shared.insert(*c) {
                    acc += get_priority(*c) as u32;
                };

                shared
            },
        );

        acc
    })
}

#[aoc(day3, part2)]
fn get_badge_priorities(input: &str) -> u32 {
    let mut acc = 0;

    for mut group in input.lines().chunks(3).into_iter() {
        let mut shared = HashSet::with_hasher(FxBuildHasher::default());

        let lines = [
            group.next().unwrap().as_bytes(),
            group.next().unwrap().as_bytes(),
            group.next().unwrap().as_bytes(),
        ];

        for c in lines[0] {
            if lines[1].contains(c) && lines[2].contains(c) && shared.insert(*c) {
                acc += get_priority(*c) as u32
            }
        }
    }

    acc
}
