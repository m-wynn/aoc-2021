aoc::main!(4);
use prettytable::{color, format, Attr, Cell, Row, Table};
use std::{cmp::Ordering, thread, time};

#[derive(Default)]
pub struct Day4 {}

impl aoc::AoCSolution for Day4 {
    type ConvertedType = Day4Input;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        let mut sections = input.split("\n\n");
        let numbers_index = 0;
        let numbers: Vec<u8> = sections
            .next()
            .expect("No lines found in input")
            .split(',')
            .map(|n| n.parse::<u8>().expect("Could not parse an integer"))
            .collect();

        let boards: Vec<Board> = sections
            .map(|board| {
                let digits: Vec<Vec<u8>> = board
                    .lines()
                    .map(|line| {
                        line.split(' ')
                            .filter(|num| !num.is_empty())
                            .map(|num| num.parse::<u8>().expect("could not parse int"))
                            .collect::<Vec<u8>>()
                    })
                    .collect();
                let state = [[false; 5]; 5];
                Board { digits, state }
            })
            .collect();

        Day4Input {
            numbers,
            numbers_index,
            boards,
        }
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut input = input.clone();
        for (i, number) in input.numbers.iter().enumerate() {
            input.numbers_index = i;
            print!("\x1B[1;1H");
            input.print();
            print!("\x10J");
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
            for board in input.boards.iter_mut() {
                board.mark(*number);
                if board.is_winner() {
                    board.as_table(*number).printstd();
                    return board.calculate_winnings() * *number as usize;
                }
            }
        }
        0
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut input = input.clone();
        for (i, number) in input.numbers.iter().enumerate() {
            input.numbers_index = i;
            print!("\x1B[2J\x1B[1;1H"); // epilepsy warning.  Unsure how to go back up and get the last column of previous rows without a lot of work
            input.print();
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
            for board in input.boards.iter_mut() {
                board.mark(*number);
            }
            if input.boards.len() == 1 && input.boards[0].is_winner() {
                return input.boards[0].calculate_winnings() * *number as usize;
            }
            input.boards.retain(|b| !b.is_winner());
        }
        0
    }
}

#[derive(Clone)]
pub struct Day4Input {
    numbers: Vec<u8>,
    numbers_index: usize,
    boards: Vec<Board>,
}

impl Day4Input {
    fn print(&self) {
        let mut current_number = 255; // lol
        let mut numtable = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new(' ', ' ', ' ', ' '),
            )
            .padding(0, 0)
            .build();
        numtable.set_format(format);
        numtable.add_row(Row::new(
            self.numbers
                .iter()
                .enumerate()
                .map(|(index, number)| match index.cmp(&self.numbers_index) {
                    Ordering::Greater => Cell::new(&format!("{}", number)),
                    Ordering::Less => Cell::new(&format!("{}", number)).with_style(Attr::Bold),
                    Ordering::Equal => {
                        current_number = *number;
                        Cell::new(&format!("{}", number))
                            .with_style(Attr::Bold)
                            .with_style(Attr::ForegroundColor(color::RED))
                    }
                })
                .collect(),
        ));

        numtable.printstd();

        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new(' ', ' ', ' ', ' '),
            )
            .padding(0, 0)
            .build();
        table.set_format(format);
        self.boards.chunks(15).for_each(|board_row| {
            table.add_row(Row::new(
                board_row
                    .iter()
                    .map(|board| Cell::from(&board.as_table(current_number).to_string()))
                    .collect(),
            ));
        });

        table.printstd();
    }
}

#[derive(Clone)]
pub struct Board {
    digits: Vec<Vec<u8>>,
    state: [[bool; 5]; 5],
}

impl Board {
    fn mark(&mut self, target: u8) {
        self.digits
            .iter()
            .zip(self.state.iter_mut())
            .for_each(|(row, staterow)| {
                row.iter().zip(staterow).for_each(|(number, state)| {
                    if *number == target {
                        *state = true
                    }
                })
            });
    }

    fn is_winner(&self) -> bool {
        let row_winner = self
            .state
            .iter()
            .filter(|staterow| staterow.iter().filter(|marked| !**marked).count() == 0)
            .count()
            > 0;
        if row_winner {
            return row_winner;
        }

        (0..self.state.len())
            .filter(|column| self.state.iter().filter(|row| !row[*column]).count() == 0)
            .count()
            > 0
    }
    fn as_table(&self, current_number: u8) -> Table {
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new(' ', ' ', ' ', ' '),
            )
            .padding(0, 0)
            .build();
        table.set_format(format);
        self.digits
            .iter()
            .zip(self.state)
            .for_each(|(row, staterow)| {
                table.add_row(Row::new(
                    row.iter()
                        .zip(staterow)
                        .map(|(number, selected)| {
                            if *number == current_number {
                                Cell::new(&format!("\x1b[0;1;31m{}\x1b[0m", number))
                            } else if selected {
                                Cell::new(&format!("\x1b[0;1m{}\x1b[0m", number))
                            } else {
                                Cell::new(&format!("\x1b[0;2m{}\x1b[0m", number))
                            }
                        })
                        .collect(),
                ));
            });
        table
    }

    fn calculate_winnings(&self) -> usize {
        self.digits
            .iter()
            .zip(self.state)
            .map(|(row, staterow)| {
                row.iter()
                    .zip(staterow)
                    .filter(|(_number, selected)| !selected)
                    .map(|(number, _selected)| *number as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}
