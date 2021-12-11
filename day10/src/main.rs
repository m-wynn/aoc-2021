aoc::main!(10);

#[derive(Default)]
pub struct Day10 {}

impl aoc::AoCSolution for Day10 {
    type ConvertedType = Vec<Vec<char>>;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input.lines().map(|x| x.chars().collect()).collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        input
            .iter()
            .map(|line| {
                let mut stack = Vec::new();
                for char in line.iter() {
                    match char {
                        '<' | '{' | '[' | '(' => stack.push(char),
                        '>' => {
                            if *stack.pop().unwrap() != '<' {
                                return 25137;
                            }
                        }
                        '}' => {
                            if *stack.pop().unwrap() != '{' {
                                return 1197;
                            }
                        }
                        ']' => {
                            if *stack.pop().unwrap() != '[' {
                                return 57;
                            }
                        }
                        ')' => {
                            if *stack.pop().unwrap() != '(' {
                                return 3;
                            }
                        }
                        _ => panic!("Unexpected char"),
                    }
                }
                0
            })
            .sum()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut scores: Vec<usize> = input
            .iter()
            .map(|line| {
                let mut stack = Vec::new();
                for char in line.iter() {
                    match char {
                        '<' | '{' | '[' | '(' => stack.push(char),
                        '>' => {
                            if *stack.pop().unwrap() != '<' {
                                return 0;
                            }
                        }
                        '}' => {
                            if *stack.pop().unwrap() != '{' {
                                return 0;
                            }
                        }
                        ']' => {
                            if *stack.pop().unwrap() != '[' {
                                return 0;
                            }
                        }
                        ')' => {
                            if *stack.pop().unwrap() != '(' {
                                return 0;
                            }
                        }
                        _ => panic!("Unexpected char"),
                    }
                }
                let mut total_score = 0;
                for char in stack.iter().rev() {
                    total_score *= 5;
                    total_score += match char {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Unexpected char"),
                    }
                }
                total_score
            })
            .filter(|score| *score != 0)
            .collect();

        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}
