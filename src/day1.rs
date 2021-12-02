pub struct Day1 { }


impl crate::aoc::AoCSolution<Vec<i32>, usize> for Day1 {
    fn day(&self) -> usize {
        1
    }

    fn convert(&self, input: &str) -> Vec<i32> {
        input.lines()
        .map(|x| x.parse::<i32>().expect("Could not parse"))
        .collect()
    }

    fn part1(&self, input: &Vec<i32>) -> usize {
        let mut prev: Option<i32> = None;
        let mut increases = 0;
        for cur in input.into_iter() {
            if let Some(p) = prev {
                if cur > &p {
                    increases += 1;
                }
            }
            prev = Some(*cur);
        }
        increases
    }

    fn part2(&self, input: &Vec<i32>) -> usize {
        let mut prev: Option<i32> = None;
        let mut prev_2: Option<i32> = None;
        let mut prev_3: Option<i32> = None;
        let mut increases = 0;
        for cur in input.into_iter() {
            if let Some(p) = prev {
                if let Some(p1) = prev_2 {
                    if let Some(p2) = prev_3 {
                        if (cur + p + p1) > (p + p1 + p2) {
                            increases += 1;
                        }
                    }
                }
            }
            prev_3 = prev_2;
            prev_2 = prev;
            prev = Some(*cur);
        }
        increases
    }
}
