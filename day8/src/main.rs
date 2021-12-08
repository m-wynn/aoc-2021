use std::collections::HashSet;

aoc::main!(8);

#[derive(Default)]
pub struct Day8 {}

impl aoc::AoCSolution for Day8 {
    type ConvertedType = Vec<Line>;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| x.split_once(" | ").unwrap())
            .map(|(input, output)| Line {
                input: input.split(' ').map(|x| x.to_string()).collect(),
                output: output.split(' ').map(|x| x.to_string()).collect(),
            })
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        input
            .iter()
            .map(|line| {
                line.output
                    .iter()
                    .filter(|word| {
                        word.len() == 2 || word.len() == 3 || word.len() == 4 || word.len() == 7
                    })
                    .count()
            })
            .sum()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut numbers: Vec<HashSet<char>> = Vec::new();
        (0..=9).for_each(|_i| numbers.push(HashSet::new()));
        input
            .iter()
            .map(|line| {
                let line_chain = line
                    .input
                    .iter()
                    .chain(line.output.iter())
                    .map(|word| word.chars().collect::<HashSet<char>>());
                numbers[1] = line_chain
                    .clone()
                    .find(|word| word.len() == 2)
                    .unwrap();
                numbers[7] = line_chain
                    .clone()
                    .find(|word| word.len() == 3)
                    .unwrap();
                numbers[4] = line_chain
                    .clone()
                    .find(|word| word.len() == 4)
                    .unwrap();
                numbers[3] = line_chain
                    .clone()
                    .filter(|word| word.len() == 5)
                    .find(|chars| chars.is_superset(&numbers[1]))
                    .unwrap();
                numbers[9] = line_chain
                    .clone()
                    .filter(|word| word.len() == 6)
                    .find(|chars| chars.is_superset(&numbers[3]))
                    .unwrap();
                numbers[8] = line_chain
                    .clone()
                    .find(|word| word.len() == 7)
                    .unwrap();
                let bottom_left = *(&numbers[8] - &numbers[9]).iter().next().unwrap();
                numbers[2] = line_chain
                    .clone()
                    .filter(|word| word.len() == 5)
                    .find(|chars| chars.contains(&bottom_left))
                    .unwrap();
                numbers[5] = line_chain
                    .clone()
                    .filter(|word| word.len() == 5)
                    .find(|chars| chars != &numbers[2] && chars != &numbers[3])
                    .unwrap();
                numbers[6] = line_chain
                    .clone()
                    .filter(|word| word.len() == 6)
                    .find(|chars| chars != &numbers[9] && chars.is_superset(&numbers[5]))
                    .unwrap();
                numbers[0] = line_chain
                    .clone()
                    .filter(|word| word.len() == 6)
                    .find(|chars| chars != &numbers[9] && chars != &numbers[6])
                    .unwrap();

                line
                    .output
                    .iter()
                    .map(|word| word.chars().collect::<HashSet<char>>())
                    .map(|chars| {
                        numbers
                            .iter()
                            .enumerate()
                            .find(|(_i, digits)| chars == **digits)
                            .map(|(i, _digits)| i)
                            .expect("Could not find a matching digit")
                    })
                    .rev()
                    .enumerate()
                    .map(|(e, digit)| ((digit as usize) * 10_usize.pow(e.try_into().unwrap())))
                    .sum::<usize>()
            })
            .sum()
    }
}

pub struct Line {
    input: Vec<String>,
    output: Vec<String>,
}
