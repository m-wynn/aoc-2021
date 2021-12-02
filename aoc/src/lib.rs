use std::fs;
use std::path::Path;

// thanks dinoocch
pub trait AoCSolution<ConvertedType, ReturnType>
where
    ReturnType: std::fmt::Display,
{
    fn day(&self) -> usize;

    fn convert(&self, input: &str) -> ConvertedType;

    fn part1(&self, input: &ConvertedType) -> ReturnType;

    fn part2(&self, input: &ConvertedType) -> ReturnType;

    fn input_path(&self) -> String {
        format!("input/day{}.txt", self.day())
    }

    fn input(&self) -> String {
        let input_path_str = self.input_path();
        let input_path = Path::new(&input_path_str);
        if !input_path.exists() {
            panic!("No input named {}", input_path_str);
        }
        return fs::read_to_string(&self.input_path())
            .expect("Something went wrong reading input path for this day!");
    }

    fn run(&self) {
        let converted = self.convert(&self.input());
        println!("Part 1: {}", self.part1(&converted));
        println!("Part 2: {}", self.part2(&converted));
    }
}
