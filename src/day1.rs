use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn elf_parser(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(str::parse::<u32>)
                .map(|r| r.expect("Elf couldn't fit in u32!"))
                .sum::<u32>()
        })
        .collect()
}


#[aoc(day1, part1)]
pub fn max_elves(elves: &[u32]) -> u32 {
    elves.iter().copied().max().expect("At least one elf!")
}

#[aoc(day1, part2)]
pub fn sum_top_elves(elves: &[u32]) -> u32 {
    let mut elves = elves.to_owned();
    elves.sort_unstable();

    let elf_count = elves.len();
    let top_3_elves = &elves[elf_count - 3..];

    top_3_elves.iter().sum()
}
