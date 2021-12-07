aoc::main!(6);

#[derive(Default)]
pub struct Day6 {}

impl aoc::AoCSolution for Day6 {
    type ConvertedType = Vec<usize>;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .split(',')
            .map(|x| x.trim().parse().expect(&format!("Could not parse {}", x)))
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut input = input.clone();
        for _day in 0..80 {
            let mut new_fish: Vec<usize> = vec![];
            input = input
                .iter_mut()
                .map(|x| {
                    if *x == 0 {
                        new_fish.push(8);
                        6
                    } else {
                        *x - 1
                    }
                })
                .collect();

            input.append(&mut new_fish)
        }
        input.len()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut fish_days = [0usize;9];
        let input = input.clone();
        input.iter().for_each(|d| { fish_days[d % 6] += 1 });

        for _day in 0..=255 {
            let birthday_fish = fish_days[0];
            for i in 1..fish_days.len() {
                fish_days[i - 1] = fish_days[i];
            }
            fish_days[6] += birthday_fish;
            fish_days[8] = birthday_fish;
        }
        fish_days.iter().sum()

    }
}
