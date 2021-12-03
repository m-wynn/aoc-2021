aoc::main!(3);
use bitvec::prelude::*;
use std::ops::BitXor;

#[derive(Default)]
pub struct Day3 {}

impl aoc::AoCSolution for Day3 {
    type ConvertedType = Vec<BitVec>;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| x.chars().map(|char| char == '1').collect::<BitVec>())
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut gamma: BitVec = BitVec::with_capacity(12);
        for i in 0..input[0].len() {
            let trues = input.iter().filter(|v| v[i]).count();
            let falses = input.len() - trues;

            gamma.push(trues > falses);
        }

        gamma.reverse();
        let epsilon = gamma.clone().bitxor(bitarr![1; 12]);

        let gamma_value: usize = gamma.load();
        let epsilon_value: usize = epsilon.load();

        (gamma_value * epsilon_value) as usize
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut oxygen = input.clone();
        let mut co2 = input.clone();
        for i in 0..input[0].len() {
            if oxygen.len() == 1 {
                break;
            }
            let trues = oxygen.iter().filter(|v| v[i]).count();
            let falses = oxygen.len() - trues;
            let i_value = trues >= falses;

            oxygen.retain(|line| line[i] == i_value);
        }

        for i in 0..input[0].len() {
            if co2.len() == 1 {
                break;
            }
            let trues = co2.iter().filter(|v| v[i]).count();
            let falses = co2.len() - trues;

            let i_value = trues < falses;

            co2.retain(|line| line[i] == i_value);
        }

        assert_eq!(oxygen.len(), 1);
        assert_eq!(co2.len(), 1);

        oxygen[0].reverse();
        co2[0].reverse();
        let oxygen_value: usize = oxygen[0].load();
        let co2_value: usize = co2[0].load();

        (oxygen_value * co2_value) as usize
    }
}
