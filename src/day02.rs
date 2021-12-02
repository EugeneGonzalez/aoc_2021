use aoc_runner_derive::{aoc, aoc_generator};

// Make it easier to synchronize the input type for the generator and solvers.
type Input = Vec<(i32, i32)>;

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Input {
    input.lines().filter_map(|l| l.split_once(' ')).map(|(w, num)| match w {
        "forward" => (num.parse::<i32>().unwrap(), 0),
        "up" => (0, -num.parse::<i32>().unwrap()),
        "down" => (0, num.parse::<i32>().unwrap()),
        _ => panic!("Unexpected word."),
    }).collect()
}

#[aoc(day2, part1)]
fn part1(moves: &Input) -> i32 {
    let (mut x, mut depth) = (0, 0);
    for (dx, dy) in moves {
        x += dx;
        depth += dy;
    }

    x * depth
}

#[aoc(day2, part2)]
fn part2(moves: &Input) -> i32 {
    let (mut x, mut depth, mut aim) = (0, 0, 0);
    for (dx, da) in moves {
        if *dx == 0 {
            aim += da;
        } else {
            x += dx;
            depth += aim * dx;
        }
    }

    x * depth
}
