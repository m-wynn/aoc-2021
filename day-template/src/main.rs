aoc::main!(${NUMBER});

#[derive(Default)]
pub struct Day${NUMBER} {}

impl aoc::AoCSolution for Day${NUMBER} {
    type ConvertedType = Vec<String>;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| x.to_string())
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        unimplemented!()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        unimplemented!()
    }
}
