use std::collections::HashMap;

aoc::main!(5);

#[derive(Default)]
pub struct Day5 {}

impl aoc::AoCSolution for Day5 {
    type ConvertedType = Vec<Line>;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| x.split_once(" -> ").expect("No -> found"))
            .map(|(begin, end)| (begin.split_once(',').unwrap(), end.split_once(',').unwrap()))
            .map(|((begin_x, begin_y), (end_x, end_y))| Line {
                begin: Point {
                    x: begin_x.parse().unwrap(),
                    y: begin_y.parse().unwrap(),
                },
                end: Point {
                    x: end_x.parse().unwrap(),
                    y: end_y.parse().unwrap(),
                },
            })
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut occurrences = HashMap::new();
        input
            .iter()
            .map(|line| {
                if line.begin.x == line.end.x {
                    // vertical
                    range(line.begin.y, line.end.y)
                        .map(|y| Point { x: line.begin.x, y })
                        .collect::<Vec<Point>>()
                } else if line.begin.y == line.end.y {
                    // horizontal
                    range(line.begin.x, line.end.x)
                        .map(|x| Point { x, y: line.begin.y })
                        .collect::<Vec<Point>>()
                } else {
                    // diagonal
                    vec![]
                }
            })
            .flatten()
            .for_each(|point| *occurrences.entry(point).or_insert(0) += 1);

        occurrences.iter().filter(|(_point, num)| **num > 1).count()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut occurrences = HashMap::new();

        input
            .iter()
            .map(|line| {
                if line.begin.x == line.end.x {
                    // vertical
                    range(line.begin.y, line.end.y)
                        .map(|y| Point { x: line.begin.x, y })
                        .collect::<Vec<Point>>()
                } else if line.begin.y == line.end.y {
                    // horizontal
                    range(line.begin.x, line.end.x)
                        .map(|x| Point { x, y: line.begin.y })
                        .collect::<Vec<Point>>()
                } else {
                    // diagonal
                    range(line.begin.x, line.end.x)
                        .zip(range(line.begin.y, line.end.y))
                        .map(|(x, y)| Point { x, y })
                        .collect::<Vec<Point>>()
                }
            })
            .flatten()
            .for_each(|point| *occurrences.entry(point).or_insert(0) += 1);

        occurrences.iter().filter(|(_point, num)| **num > 1).count()
    }
}

fn range(x: usize, y: usize) -> Box<dyn Iterator<Item = usize>> {
    if x < y {
        Box::new(x..=y)
    } else {
        Box::new((y..=x).rev())
    }
}

pub struct Line {
    begin: Point,
    end: Point,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}
