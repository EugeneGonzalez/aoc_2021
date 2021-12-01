use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<i32>;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Input, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(nums: &Input) -> usize {
    nums.windows(2).filter(|&w| w[0] < w[1]).count()
}

#[aoc(day1, part2)]
fn part2(nums: &Input) -> usize {
    nums.windows(3)
        .map(|w| w.iter().sum())
        .fold((0usize, i32::MAX), |(c, prev), next| {
            (c + (prev < next) as usize, next)
        })
        .0
}
