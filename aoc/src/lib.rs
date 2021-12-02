use std::fs;
use std::path::Path;

pub use aoc_macro::main;

pub trait AoCSolution<ConvertedType, ReturnType>
where
    ReturnType: std::fmt::Display,
{
    fn convert(&self, input: &str) -> ConvertedType;

    fn part1(&self, input: &ConvertedType) -> ReturnType;

    fn part2(&self, input: &ConvertedType) -> ReturnType;

    fn input(&self, day: &str) -> String {
        let input_path_str = format!("input/{}.txt", day.to_lowercase());
        let input_path = Path::new(&input_path_str);
        if !input_path.exists() {
            panic!("No input named {}", input_path_str);
        }
        return fs::read_to_string(&input_path_str)
            .expect("Something went wrong reading input path for this day!");
    }

    fn run(&self, day: &str) {
        let converted = self.convert(&self.input(day));
        println!("Part 1: {}", self.part1(&converted));
        println!("Part 2: {}", self.part2(&converted));
    }
}
