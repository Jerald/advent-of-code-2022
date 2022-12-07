#![feature(array_chunks)]

// I use "Louise" and "Richard" for the left and right player respectively.
// This is an homage to combinatorial game theory where you do the same, inspired by some old game theorist named Richard who used his wife Louise's name as a player stand-in.

use std::ops::Add;
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    pub louise: LouisePlays,
    pub richard: RichardPlays,
}

impl Game {
    const LINE_BYTE_LENGTH: usize = 3;
}

type GameLine = [u8; Game::LINE_BYTE_LENGTH];

#[derive(Debug)]
enum GameError {
    LouisePlaysWrong,
    RichardPlaysWrong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

trait GamePlayer {
    /// Rock, Paper, Scissors...
    fn shoot(&self) -> GameMove;

    fn play_against(&self, other: &impl GamePlayer) -> GameOutcome {
        let your_move = self.shoot();
        let their_move = other.shoot();

        if your_move == their_move {
            GameOutcome::Draw
        } else if your_move == your_move + their_move {
            GameOutcome::Won
        } else {
            /* their_move == your_move + their_move */
            GameOutcome::Lost
        }
    }

    fn how_to(&self, outcome: GameOutcome) -> GameMove {
        use GameOutcome::*;
        match outcome {
            Won => self.how_to_win(),
            Lost => self.how_to_lose(),
            Draw => self.shoot(),
        }
    }

    fn how_to_lose(&self) -> GameMove {
        use GameMove::*;
        match self.shoot() {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn how_to_win(&self) -> GameMove {
        use GameMove::*;
        match self.shoot() {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LouisePlays(GameMove);

impl GamePlayer for LouisePlays {
    fn shoot(&self) -> GameMove {
        self.0
    }
}

impl TryFrom<&GameLine> for LouisePlays {
    type Error = GameError;

    fn try_from(value: &GameLine) -> Result<LouisePlays, Self::Error> {
        use GameMove::*;

        match value[0] {
            b'A' => Ok(LouisePlays(Rock)),
            b'B' => Ok(LouisePlays(Paper)),
            b'C' => Ok(LouisePlays(Scissors)),

            _ => Err(GameError::LouisePlaysWrong),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RichardPlays(GameMove);

impl GamePlayer for RichardPlays {
    fn shoot(&self) -> GameMove {
        self.0
    }
}

impl TryFrom<&GameLine> for RichardPlays {
    type Error = GameError;

    fn try_from(value: &GameLine) -> Result<Self, Self::Error> {
        use GameMove::*;

        match value[2] {
            b'X' => Ok(RichardPlays(Rock)),
            b'Y' => Ok(RichardPlays(Paper)),
            b'Z' => Ok(RichardPlays(Scissors)),

            _ => Err(GameError::RichardPlaysWrong),
        }
    }
}

impl TryFrom<&GameLine> for Game {
    type Error = GameError;

    fn try_from(line: &GameLine) -> Result<Self, Self::Error> {
        Ok(Game {
            louise: line.try_into()?,
            richard: line.try_into()?,
        })
    }
}

// For moves, adding is defined as returning the winning move
impl Add for GameMove {
    type Output = Self;

    /// Do NOT call this when lhs == rhs, that is considered an error
    fn add(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(Self::Rock as u8, 1);
        debug_assert_eq!(Self::Paper as u8, 2);
        debug_assert_eq!(Self::Scissors as u8, 3);
        // Can't call this with lhs == rhs! I just told you!
        debug_assert_ne!(self, rhs);

        // R + P == 1 + 2 == 3 => Paper wins (2)
        // R + S == 1 + 3 == 4 => Rock wins (1)
        // P + S == 2 + 3 == 5 => Scissors wins (3)
        match self as u8 + rhs as u8 {
            3 => Self::Paper,
            4 => Self::Rock,
            5 => Self::Scissors,

            _ => panic!("GameMove has been messed up! Don't do that!"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum GameOutcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl FromStr for Game {
    type Err = GameError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Want to make sure it's of the form we expect
        debug_assert!(line.is_ascii());
        debug_assert!(line.bytes().len() == Game::LINE_BYTE_LENGTH);
        debug_assert!(line.as_bytes()[1] == b' ');

        let game_line = line
            .as_bytes()
            .array_chunks()
            .next()
            .expect("We already asserted this!");

        game_line.try_into()
    }
}

#[derive(Debug, Clone, Copy)]
struct PlayerScore(GameMove, GameOutcome);

impl PlayerScore {
    fn to_value(self) -> u32 {
        self.0 as u32 + self.1 as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_score = input
        .lines()
        .map(Game::from_str)
        .map(|result| result.expect("Oh no! There was a "))
        .map(|Game { louise, richard }| {
            // We play as Richard against Louise
            PlayerScore(richard.shoot(), richard.play_against(&louise))
        })
        .map(PlayerScore::to_value)
        .sum();

    Some(total_score)
}

struct FindMoveForRichard {
    louise: LouisePlays,
    outcome: GameOutcome,
}

impl FromStr for FindMoveForRichard {
    type Err = GameError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Game { louise, richard } = Game::from_str(line)?;

        use GameMove::*;
        use GameOutcome::*;

        // Now do black magic because I'm lazy and it's getting late
        let outcome = match richard.shoot() {
            Rock => Lost,
            Paper => Draw,
            Scissors => Won,
        };

        Ok(FindMoveForRichard { louise, outcome })
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_score = input
        .lines()
        .map(FindMoveForRichard::from_str)
        .map(|result| result.expect("Oh no! There was a "))
        .map(|FindMoveForRichard { louise, outcome }| {
            // We want to have `outcome` against Louise as Richard
            PlayerScore(louise.how_to(outcome), outcome)
        })
        .map(PlayerScore::to_value)
        .sum();

    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
