use aoc_runner_derive::{aoc, aoc_generator};

type Ship = Vec<Vec<char>>;

#[derive(Debug)]
pub struct Instruction {
    num: u8,
    src: u8,
    dst: u8,
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> (Ship, Vec<Instruction>) {
    let mut crates: Vec<_> = input.lines().take_while(|l| !l.is_empty()).collect();

    let instruction_offset = crates.len() + 1;
    let last_line = crates.remove(crates.len() - 1);
    let num_crates = last_line.chars().filter(|c| *c != ' ').count();

    crates.reverse();

    let mut ship: Ship = (0..num_crates).map(|_| Vec::new()).collect();
    for line in crates {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| ship[i].push(c));
    }

    let instructions = input
        .lines()
        .skip(instruction_offset)
        .map(|line| {
            let mut split_line = line.split(' ');
            split_line.next(); // move
            let num = split_line.next().unwrap().parse().unwrap();
            split_line.next(); // from
            let src: u8 = split_line.next().unwrap().parse().unwrap();
            split_line.next(); // to
            let dst: u8 = split_line.next().unwrap().parse().unwrap();

            Instruction {
                num,
                src: src - 1,
                dst: dst - 1,
            }
        })
        .collect();

    (ship, instructions)
}

fn follow_instructions(ship: &mut Ship, instructions: &[Instruction], reverse_buffer: bool) {
    let mut move_buffer = Vec::new();

    for instruction in instructions {
        let heap = &mut ship[instruction.src as usize];

        for _ in 0..instruction.num {
            move_buffer.push(heap.remove(heap.len() - 1))
        }

        if reverse_buffer {
            move_buffer.reverse()
        };

        ship[instruction.dst as usize].append(&mut move_buffer);
    }
}

#[aoc(day5, part1)]
pub fn get_top_crates((ship, instructions): &(Ship, Vec<Instruction>)) -> String {
    let mut ship = ship.clone();
    follow_instructions(&mut ship, instructions, false);
    ship.into_iter()
        .map(|heap| *heap.last().unwrap_or(&' '))
        .collect()
}

#[aoc(day5, part2)]
pub fn get_top_crates_9001((ship, instructions): &(Ship, Vec<Instruction>)) -> String {
    let mut ship = ship.clone();
    follow_instructions(&mut ship, instructions, true);
    ship.into_iter()
        .map(|heap| *heap.last().unwrap_or(&' '))
        .collect()
}
