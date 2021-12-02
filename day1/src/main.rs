use itertools::Itertools;

aoc::main!(Day1);

pub struct Day1 {}

impl aoc::AoCSolution<Vec<i32>, usize> for Day1 {
    fn convert(&self, input: &str) -> Vec<i32> {
        input
            .lines()
            .map(|x| x.parse::<i32>().expect("Could not parse"))
            .collect()
    }

    fn part1(&self, input: &Vec<i32>) -> usize {
        input
            .into_iter()
            .tuple_windows()
            .filter(|(prev, cur)| cur > prev)
            .count()
    }

    fn part2(&self, input: &Vec<i32>) -> usize {
        input
            .into_iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(prev, cur)| cur > prev)
            .count()
    }
}
