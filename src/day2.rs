use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
enum UnknownColumn {
    X,
    Y,
    Z,
}

impl From<char> for UnknownColumn {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::X,
            'Y' => Self::Y,
            'Z' => Self::Z,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn selection_score(self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl From<UnknownColumn> for Choice {
    fn from(column: UnknownColumn) -> Self {
        match column {
            UnknownColumn::X => Choice::Rock,
            UnknownColumn::Y => Choice::Paper,
            UnknownColumn::Z => Choice::Scissors,
        }
    }
}

impl From<char> for Choice {
    fn from(input: char) -> Self {
        match input {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Move {
    unknown: UnknownColumn,
    opponent: Choice,
}

impl Move {
    fn outcome(player: Choice, opponent: Choice) -> u16 {
        const LOSE: u16 = 0;
        const DRAW: u16 = 3;
        const WIN: u16 = 6;

        let selection_score = player.selection_score() as u16;
        let outcome_score = match (player, opponent) {
            (Choice::Paper, Choice::Rock) => WIN,
            (Choice::Rock, Choice::Paper) => LOSE,

            (Choice::Scissors, Choice::Paper) => WIN,
            (Choice::Paper, Choice::Scissors) => LOSE,

            (Choice::Rock, Choice::Scissors) => WIN,
            (Choice::Scissors, Choice::Rock) => LOSE,

            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => DRAW,
        };

        selection_score + outcome_score
    }

    pub fn outcome_if_unknown_player(self) -> u16 {
        let player: Choice = self.unknown.into();
        Self::outcome(player, self.opponent)
    }

    pub fn part2_outcome(self) -> u16 {
        // X = lose
        // Y = draw
        // Z = win

        let to_move = match (self.opponent, self.unknown) {
            (Choice::Paper, UnknownColumn::X) => Choice::Rock,
            (Choice::Rock, UnknownColumn::X) => Choice::Scissors,
            (Choice::Scissors, UnknownColumn::X) => Choice::Paper,

            (Choice::Paper, UnknownColumn::Z) => Choice::Scissors,
            (Choice::Scissors, UnknownColumn::Z) => Choice::Rock,
            (Choice::Rock, UnknownColumn::Z) => Choice::Paper,

            (_, UnknownColumn::Y) => self.opponent,
        };

        Self::outcome(to_move, self.opponent)
    }
}

#[aoc_generator(day2)]
pub fn move_parser(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut input = input.chars();

    loop {
        let opponent = input.next();
        input.next();
        let unknown = input.next();
        input.next();

        if let (Some(unknown), Some(opponent)) = (unknown, opponent) {
            moves.push(Move {
                unknown: UnknownColumn::from(unknown),
                opponent: Choice::from(opponent),
            })
        } else {
            break;
        }
    }

    moves
}

#[aoc(day2, part1)]
pub fn calculate_total_score(input: &[Move]) -> u16 {
    input
        .iter()
        .copied()
        .map(Move::outcome_if_unknown_player)
        .sum()
}

#[aoc(day2, part2)]
pub fn calculate_max_score(input: &[Move]) -> u16 {
    input.iter().copied().map(Move::part2_outcome).sum()
}
