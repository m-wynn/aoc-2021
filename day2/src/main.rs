use aoc::AoCSolution;
use std::str::FromStr;

pub struct Day2 {}

pub enum Day2Data {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Day2Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Day2Data, Self::Err> {
        let (direction, number) = input.split_once(' ').expect("Could not split");
        let n = number.parse().expect("Could not parse number");
        match direction {
            "forward" => Ok(Day2Data::Forward(n)),
            "down" => Ok(Day2Data::Down(n)),
            "up" => Ok(Day2Data::Up(n)),
            _ => Err(()),
        }
    }
}

impl aoc::AoCSolution<Vec<Day2Data>, isize> for Day2 {
    fn day(&self) -> usize {
        2
    }

    fn convert(&self, input: &str) -> Vec<Day2Data> {
        input
            .lines()
            .map(|x| x.parse::<Day2Data>().expect("Could not parse"))
            .collect()
    }

    fn part1(&self, input: &Vec<Day2Data>) -> isize {
        let (horizontal, depth) =
            input
                .iter()
                .fold((0, 0), |(horizontal, depth), line| match line {
                    Day2Data::Forward(n) => (horizontal + n, depth),
                    Day2Data::Up(n) => (horizontal, depth - n),
                    Day2Data::Down(n) => (horizontal, depth + n),
                });
        horizontal * depth
    }

    fn part2(&self, input: &Vec<Day2Data>) -> isize {
        let (horizontal, depth, _) =
            input
                .iter()
                .fold((0, 0, 0), |(horizontal, depth, aim), line| match line {
                    Day2Data::Forward(n) => (horizontal + n, depth + aim * n, aim),
                    Day2Data::Up(n) => (horizontal, depth, aim - n),
                    Day2Data::Down(n) => (horizontal, depth, aim + n),
                });
        horizontal * depth
    }
}

fn main() {
    Day2 {}.run();
}
