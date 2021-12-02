use reqwest::header::COOKIE;
use reqwest::StatusCode;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub use aoc_macro::main;

pub trait AoCSolution {
    type ConvertedType;
    type ReturnType;
    fn convert(&self, input: &str) -> Self::ConvertedType;

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType;

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType;

    // thanks dinoocch
    fn download_input(&self, day: u8) {
        let token_path = Path::new("./.session");
        let input_path_str = format!("input/day{}.txt", day);
        let input_path = Path::new(&input_path_str);
        if !token_path.exists() {
            panic!("Set token from session cookie in .sesssion file or manually download input!");
        }
        let session_token = fs::read_to_string(token_path)
            .expect("Error reading session token from .session file!");

        let resp = reqwest::blocking::Client::new()
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                2021, day
            ))
            .header(COOKIE, format!("session={}", session_token.trim()))
            .send();

        match resp {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    fs::create_dir_all(&(input_path.parent().unwrap())).unwrap();
                    let body = response.text().expect("Error reading input");
                    let mut file = fs::File::create(input_path_str).unwrap();
                    file.write_all(body.as_bytes()).unwrap();
                }
                error_code => {
                    panic!(
                        "error getting aoc input. StatusCode: {}, Body: {:?}",
                        error_code,
                        response.text()
                    );
                }
            },
            Err(e) => {
                panic!("Failed to get a response. Err({})", e);
            }
        };
    }

    fn input(&self, day: u8) -> String {
        let input_path_str = format!("input/day{}.txt", day);
        let input_path = Path::new(&input_path_str);
        if !input_path.exists() {
            self.download_input(day);
        }
        fs::read_to_string(&input_path_str)
            .expect("Something went wrong reading input path for this day!")
    }

    fn run(day: u8)
    where
        <Self as AoCSolution>::ReturnType: std::fmt::Display,
        Self: std::default::Default,
    {
        let s = Self::default();
        let converted = s.convert(&s.input(day));
        println!("Part 1: {}", s.part1(&converted));
        println!("Part 2: {}", s.part2(&converted));
    }
}
