aoc::main!(9);

#[derive(Default)]
pub struct Day9 {}

impl aoc::AoCSolution for Day9 {
    type ConvertedType = Vec<Vec<u32>>;
    type ReturnType = u32;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        input
            .iter()
            .enumerate()
            .map(|(lindex, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(index, digit)| {
                        if lindex > 0 && *digit >= input[lindex - 1][index] // above
                            || lindex < input.len() - 1 && *digit >= input[lindex + 1][index] // below
                            || index > 0 && *digit >= line[index - 1] // left
                            || index < line.len() - 1 && *digit >= line[index + 1] // right
                        {
                            None
                        } else {
                            Some(digit + 1)
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let input: Vec<Vec<&u32>> = input
            .iter()
            .map(|line| line.iter().collect()) // Convert all u32s to references
            .collect();
        let mut basins: Vec<Vec<&u32>> = Vec::new(); // References to the u32s in input
        for lindex in 0..input.len() {
            for index in 0..input[0].len() {
                let current = input[lindex][index];
                // skip if border or exists in any existing basin
                if *current != 9
                    && !basins
                        .iter()
                        .any(|basin| basin.iter().any(|x| std::ptr::eq(*x, current)))
                {
                    let mut basin: Vec<&u32> = Vec::new();
                    populate_basin(&input, lindex, index, &mut basin);
                    basins.push(basin)
                }
            }
        }

        let mut sorted_basins: Vec<usize> = basins.iter().map(|basin| basin.len()).collect();
        sorted_basins.sort_unstable();
        sorted_basins.reverse();
        sorted_basins.iter().take(3).product::<usize>() as u32
    }
}

fn populate_basin<'a>(input: &[Vec<&'a u32>], lindex: usize, index: usize, basin: &mut Vec<&'a u32>) {
    // Recurse like a depth-first search
    let current = input[lindex][index];
    if *current != 9 && !basin.iter().any(|x| std::ptr::eq(*x, current)) { // border or already seen
        basin.push(current);
        if lindex > 0 {
            //above
            populate_basin(input, lindex - 1, index, basin)
        }
        if lindex < input.len() - 1 {
            //below
            populate_basin(input, lindex + 1, index, basin)
        }
        if index > 0 {
            //left
            populate_basin(input, lindex, index - 1, basin)
        }
        if index < input[lindex].len() - 1 {
            //right
            populate_basin(input, lindex, index + 1, basin)
        }
    }
}
