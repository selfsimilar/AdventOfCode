// Advent of Code 2022 Day 02b:
// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
// input is a file of numbers, one per line, with a blank line to demarcate groupings (Elves).
//
// Sum the groupings to find the grouping (Elf) with the highest total. Print the largest sum.

#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

const WINPOINTS: u32 = 6;
const DRAWPOINTS: u32 = 3;
const LOSEPOINTS: u32 = 0;
const ROCKPOINTS: u32 = 1;
const PAPERPOINTS: u32 = 2;
const SCISSORPOINTS: u32 = 3;

#[derive(PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn move_points(player: Move) -> u32 {
    match player {
        Move::Rock     => return ROCKPOINTS,
        Move::Paper    => return PAPERPOINTS,
        Move::Scissors => return SCISSORPOINTS,
    }
}

fn outcome_points(player: Outcome) -> u32 {
    match player {
        Outcome::Win  => return WINPOINTS,
        Outcome::Draw => return DRAWPOINTS,
        Outcome::Lose => return LOSEPOINTS,
    }
}

fn play(opponent: Move, player: Move) -> Outcome {
    if (Move::Rock == opponent && Move::Paper == player)
        || (Move::Paper == opponent && Move::Scissors == player)
        || (Move::Scissors == opponent && Move::Rock == player) {
        return Outcome::Win;
    }
    else if (Move::Paper == opponent && Move::Paper == player)
        || (Move::Scissors == opponent && Move::Scissors == player)
        || (Move::Rock == opponent && Move::Rock == player) {
        return Outcome::Draw;
    }
    return Outcome::Lose;
}

fn inferred_play(opponent: &Move, result: &Outcome) -> Move {
    if (&Move::Rock == opponent && &Outcome::Draw == result)
        || (&Move::Paper == opponent && &Outcome::Lose == result)
        || (&Move::Scissors == opponent && &Outcome::Win == result) {
        return Move::Rock;
    }
    else if (&Move::Paper == opponent && &Outcome::Draw == result)
        || (&Move::Scissors == opponent && &Outcome::Lose == result)
        || (&Move::Rock == opponent && &Outcome::Win == result) {
        return Move::Paper;
    }
    return Move::Scissors;
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let mut score = 0;
    let mut opponent_move;
    let mut your_move;
    let mut outcome;

    for line in content.lines() {
        let mut movepoints = 0;
        let moves: Vec<&str> = line.split(' ').collect();
        // let mut opponent_move = moves[0];
        // let mut outcome = moves[1];
        match moves[0] {
            "A" => opponent_move = Move::Rock,
            "B" => opponent_move = Move::Paper,
            "C" => opponent_move = Move::Scissors,
            _ => panic!("bad input: opponent move {}.", moves[0]),
        }

        match moves[1] {
            "X" => outcome = Outcome::Lose,
            "Y" => outcome = Outcome::Draw,
            "Z" => outcome = Outcome::Win,
            _ => panic!("bad input: outcome {}.", moves[1]),
        }

        your_move = inferred_play(&opponent_move, &outcome);
        score += move_points(your_move) + outcome_points(outcome);
    }
    println!("{}", score);
}

/*
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
*/
