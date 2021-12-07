aoc::main!(7);

#[derive(Default)]
pub struct Day7 {}

impl aoc::AoCSolution for Day7 {
    type ConvertedType = Vec<isize>;
    type ReturnType = isize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .split(',')
            .map(|x| x.trim().parse().expect(&format!("Could not parse {}", x)))
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        (0..2000)
            .map(|test| {
                input
                    .iter()
                    .map(|crab| (*crab as isize - test as isize).abs())
                    .sum::<isize>()
            })
            .min()
            .unwrap()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        (0..2000)
            .map(|test| {
                input
                    .iter()
                    .map(|crab| (*crab as isize - test as isize).abs())
                    .map(|fuel| (fuel * (fuel + 1)) / 2)
                    .sum::<isize>()
            })
            .min()
            .unwrap()
    }
}
