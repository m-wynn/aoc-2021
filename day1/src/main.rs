use itertools::Itertools;

aoc::main!(1);

#[derive(Default)]
pub struct Day1 {}

impl aoc::AoCSolution for Day1 {
    type ConvertedType = Vec<i32>;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| x.parse::<i32>().expect("Could not parse"))
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        input
            .iter()
            .tuple_windows()
            .filter(|(prev, cur)| cur > prev)
            .count()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        input
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(prev, cur)| cur > prev)
            .count()
    }
}
