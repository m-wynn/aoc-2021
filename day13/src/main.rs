use std::collections::HashSet;

aoc::main!(13);

#[derive(Default)]
pub struct Day13 {}

impl aoc::AoCSolution for Day13 {
    type ConvertedType = Day13Input;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        let (dots, folds) = input.split_once("\n\n").unwrap();
        let dots = dots
            .lines()
            .map(|line| line.split_once(',').expect("No comma found"))
            .map(|(x, y)| Dot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
            .collect();

        let folds = folds
            .lines()
            .map(|line| {
                line.strip_prefix("fold along ")
                    .expect("Did not find 'fold along'")
                    .split_once('=')
                    .expect("Could not split fold")
            })
            .map(|(axis, line)| match axis {
                "x" => Fold {
                    axis: Axis::X,
                    line: line.parse().unwrap(),
                },
                "y" => Fold {
                    axis: Axis::Y,
                    line: line.parse().unwrap(),
                },
                _ => panic!("Axis was not x or y"),
            })
            .collect();

        Day13Input { dots, folds }
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let fold = &input.folds[0];
        input
            .dots
            .iter()
            .map(|dot| dot.fold(fold))
            .collect::<HashSet<Dot>>()
            .len()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let final_dots = input.folds.iter().fold(input.dots.clone(), |acc, fold| {
            acc.iter().map(|dot| dot.fold(fold)).collect()
        });

        for y in 0..7 {
            for x in 0..40 {
                print!(
                    "{}",
                    if final_dots.contains(&Dot { x, y }) {
                        "█"
                    } else {
                        " "
                    }
                );
            }
            println!();
        }
        0
    }
}

pub struct Day13Input {
    dots: HashSet<Dot>,
    folds: Vec<Fold>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Dot {
    x: u32,
    y: u32,
}

impl Dot {
    fn fold(&self, fold: &Fold) -> Dot {
        match fold.axis {
            Axis::X => {
                if self.x < fold.line {
                    self.clone()
                } else {
                    Dot {
                        x: 2 * fold.line - self.x,
                        y: self.y,
                    }
                }
            }
            Axis::Y => {
                if self.y < fold.line {
                    self.clone()
                } else {
                    Dot {
                        x: self.x,
                        y: 2 * fold.line - self.y,
                    }
                }
            }
        }
    }
}

pub struct Fold {
    axis: Axis,
    line: u32,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
}
