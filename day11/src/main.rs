aoc::main!(11);

#[derive(Default)]
pub struct Day11 {}

impl aoc::AoCSolution for Day11 {
    type ConvertedType = Vec<Vec<Octopus>>;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| Octopus {
                        has_flashed: false,
                        energy: x.to_digit(10).expect("Could not parse octopus energy!"),
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut input = input.clone();
        (0..100).map(|_| step_flashes(&mut input)).sum()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut input = input.clone();
        let mut step: usize = 0;
        let target_flashes = input.len() * input[0].len();
        loop {
            step += 1;

            // all octopodes flashed
            if step_flashes(&mut input) == target_flashes {
                return step;
            }
        }
    }
}

fn step_flashes(input: &mut Vec<Vec<Octopus>>) -> usize {
    let mut step_flashes = 0;
    let mut flashed_this_scan;
    input
        .iter_mut()
        .for_each(|line| line.iter_mut().for_each(|octopus| octopus.new_step()));

    loop {
        flashed_this_scan = false;
        for lindex in 0..input.len() {
            for index in 0..input[0].len() {
                if input[lindex][index].check_flash() {
                    step_flashes += 1;
                    flashed_this_scan = true;

                    charge_adjacent(input, lindex, index);
                }
            }
        }
        if !flashed_this_scan {
            break;
        }
    }
    step_flashes
}

fn charge_adjacent(input: &mut Vec<Vec<Octopus>>, lindex: usize, index: usize) {
    //increment adjacent
    if lindex > 0 {
        input[lindex - 1][index].charge(); // above
    }
    if lindex < input.len() - 1 {
        input[lindex + 1][index].charge(); // below
    }
    if index > 0 {
        input[lindex][index - 1].charge(); //left
    }
    if index < input[0].len() - 1 {
        input[lindex][index + 1].charge(); // right
    }

    if lindex > 0 && index > 0 {
        input[lindex - 1][index - 1].charge(); // above left
    }
    if lindex < input.len() - 1 && index > 0 {
        input[lindex + 1][index - 1].charge(); // below left
    }
    if lindex > 0 && index < input[0].len() - 1 {
        input[lindex - 1][index + 1].charge(); // above right
    }
    if lindex < input.len() - 1 && index < input[0].len() - 1 {
        input[lindex + 1][index + 1].charge(); // below right
    }
}

#[derive(Clone)]
pub struct Octopus {
    has_flashed: bool,
    energy: u32,
}

impl Octopus {
    fn charge(&mut self) {
        if !self.has_flashed {
            self.energy += 1;
        }
    }
    fn new_step(&mut self) {
        self.has_flashed = false;
        self.energy += 1;
    }

    fn check_flash(&mut self) -> bool {
        if !self.has_flashed && self.energy > 9 {
            self.energy = 0;
            self.has_flashed = true;
            true
        } else {
            false
        }
    }
}
