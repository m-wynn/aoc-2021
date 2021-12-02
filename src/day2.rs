use std::str::FromStr;
pub struct Day2 { }

pub enum Day2Data {
    Forward(isize),
    Down(isize),
    Up(isize)
}

impl FromStr for Day2Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Day2Data, Self::Err> {
        let (direction, number) = input.split_once(' ').expect("Could not split");
        let n = number.parse().expect("Could not parse number");
        match direction {
            "forward"  => Ok(Day2Data::Forward(n)),
            "down"  => Ok(Day2Data::Down(n)),
            "up"  => Ok(Day2Data::Up(n)),
            _      => Err(()),
        }
    }
}


impl crate::aoc::AoCSolution<Vec<Day2Data>, isize> for Day2 {
    fn day(&self) -> usize {
        2
    }

    fn convert(&self, input: &str) -> Vec<Day2Data> {
        input.lines()
        .map(|x| x.parse::<Day2Data>().expect("Could not parse"))
        .collect()
    }

    fn part1(&self, input: &Vec<Day2Data>) -> isize {
        let mut horizontal = 0;
        let mut depth = 0;
        for dir in input.iter() {
            match dir {
                Day2Data::Forward(n) => horizontal += n,
                Day2Data::Up(n) => depth -= n,
                Day2Data::Down(n) => depth += n,
            }
        }
        horizontal * depth
    }

    fn part2(&self, input: &Vec<Day2Data>) -> isize {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;
        for dir in input.iter() {
            match dir {
                Day2Data::Forward(n) => {
                    horizontal += n;
                    depth += aim * n;
                },
                Day2Data::Up(n) => aim -= n,
                Day2Data::Down(n) => aim += n,
            }
        }
        horizontal * depth
    }
}
