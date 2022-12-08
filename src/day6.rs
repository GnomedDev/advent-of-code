use aoc_runner_derive::{aoc};

pub fn find_header<const LEN: usize>(input: &[u8]) -> u16 {
    let mut window: [u8; LEN];
    let mut position: usize = 0;

    loop {
        window = input[position..position+LEN].try_into().unwrap();

        let contains_duplicates = window
            .iter()
            .enumerate()
            .any(|(i, c)| window[i + 1..].contains(c));

        if !contains_duplicates {
            break (position + LEN) as u16
        }

        position += 1;
    }
}

#[aoc(day6, part1)]
pub fn find_4_header(input: &[u8]) -> u16 {
    find_header::<4>(input)
}

#[aoc(day6, part2)]
pub fn find_14_header(input: &[u8]) -> u16 {
    find_header::<14>(input)
}
