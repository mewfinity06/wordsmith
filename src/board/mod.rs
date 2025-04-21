#![allow(dead_code)]

use anyhow::anyhow;

pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct Board {
    elements: [char; Self::BOARD_SIZE],
}

impl Board {
    // Size constants
    const BOARD_SIZE: usize = Self::ROW_LENGTH * Self::COL_LENGTH;
    const ROW_LENGTH: usize = 15;
    const COL_LENGTH: usize = 15;

    fn index_to_coords(i: usize) -> anyhow::Result<(usize, usize)> {
        if i < Self::ROW_LENGTH * Self::COL_LENGTH {
            let y = i / Self::COL_LENGTH;
            let x = i % Self::COL_LENGTH;
            Ok((x, y))
        } else {
            Err(anyhow!("Index out of bounds"))
        }
    }

    fn coords_to_index(x: usize, y: usize) -> anyhow::Result<usize> {
        if x < Self::COL_LENGTH && y < Self::ROW_LENGTH {
            Ok(y * Self::COL_LENGTH + x)
        } else {
            Err(anyhow!("Coords out of index"))
        }
    }

    // Print constants
    // (See https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797)
    const TERMINAL_COLOR_DEFAULT: &str = "\x1b[39m\x1b[49m";
    const CENTER_SQUARE_COLOR: &str = "\x1b[48;5;226m\x1b[38;5;16m";
    const TEXT_COLOR_DEF: &str = "\x1b[39m\x1b[49m";
    const TW_SCORE_COLOR: &str = "\x1b[48;5;161m\x1b[38;5;16m";
    const DW_SCORE_COLOR: &str = "\x1b[48;5;217m\x1b[38;5;16m";
    const TL_SCORE_COLOR: &str = "\x1b[48;5;32m\x1b[38;5;16m";
    const DL_SCORE_COLOR: &str = "\x1b[45;5;51m\x1b[38;5;16m";

    // Positions (y, x)
    const TW_SCORE_POS: &[(usize, usize)] = &[
        (0x0, 0x0),
        (0x0, 0x7),
        (0x0, 0xE),
        (0x7, 0x0),
        (0x7, 0xE),
        (0xE, 0x0),
        (0xE, 0x7),
        (0xE, 0xE),
    ];
    const DW_SCORE_POS: &[(usize, usize)] = &[
        (0x1, 0x1),
        (0x1, 0xD),
        (0x2, 0x2),
        (0x2, 0xC),
        (0x3, 0x3),
        (0x3, 0xB),
        (0x4, 0x4),
        (0x4, 0xA),
        (0xA, 0x4),
        (0xA, 0xA),
        (0xB, 0x3),
        (0xB, 0xB),
        (0xC, 0x2),
        (0xC, 0xC),
        (0xD, 0x1),
        (0xD, 0xD),
    ];
    const TL_SCORE_POS: &[(usize, usize)] = &[
        (0x1, 0x5),
        (0x1, 0x9),
        (0x5, 0x1),
        (0x5, 0x5),
        (0x5, 0x9),
        (0x5, 0xD),
        (0x9, 0x1),
        (0x9, 0x5),
        (0x9, 0x9),
        (0x9, 0xD),
        (0xE, 0x5),
        (0xE, 0x9),
    ];
    const DL_SCORE_POS: &[(usize, usize)] = &[
        (0x0, 0x3),
        (0x0, 0xB),
        (0x2, 0x6),
        (0x2, 0x8),
        (0x3, 0x0),
        (0x3, 0x7),
        (0x3, 0xE),
        (0x6, 0x2),
        (0x6, 0x6),
        (0x6, 0x8),
        (0x6, 0xC),
        (0x7, 0x3),
        (0x7, 0xB),
        (0x8, 0x2),
        (0x8, 0x6),
        (0x8, 0x8),
        (0x8, 0xC),
        (0xB, 0x0),
        (0xB, 0x7),
        (0xB, 0xE),
        (0xC, 0x6),
        (0xC, 0x8),
        (0xE, 0x3),
        (0xE, 0xB),
    ];
    const CENTER_SQUARE: (usize, usize) = (0x7, 0x7);
    const STAR_CHAR: char = '*';

    pub fn new() -> Self {
        let mut elements = ['_'; Self::BOARD_SIZE];
        if let Ok(center_index) =
            Self::coords_to_index(Self::CENTER_SQUARE.0, Self::CENTER_SQUARE.1)
        {
            elements[center_index] = Self::STAR_CHAR;
        }
        Self { elements }
    }

    // TODO: Center the board display in the middle of the terminal...
    // Maybe use Crossterm? Or maybe just use Rust std::???
    pub fn display(&self) {
        fn print_n_char(len: usize, c: char) {
            for _ in 0..len {
                print!("{c}");
            }
        }

        print!("{}", Self::TEXT_COLOR_DEF);
        println!();
        println!("-----------------------------------");
        println!("|            WORDSMITH            |");
        println!("-----------------------------------");
        print!("|   ");
        for i in 0..Self::COL_LENGTH {
            print!("{:X} ", i);
        }
        println!("|");

        let mut row: usize = 0;
        for (i, e) in self.elements.iter().enumerate() {
            if i % Self::COL_LENGTH == 0 {
                print!("| {:X} ", row);
                row += 1;
            }

            let coords =
                Self::index_to_coords(i).expect("Value 'i' should always be a valid index.");

            if Self::TW_SCORE_POS.contains(&coords) {
                print!("{}", Self::TW_SCORE_COLOR);
            } else if Self::DW_SCORE_POS.contains(&coords) {
                print!("{}", Self::DW_SCORE_COLOR);
            } else if Self::TL_SCORE_POS.contains(&coords) {
                print!("{}", Self::TL_SCORE_COLOR);
            } else if Self::DL_SCORE_POS.contains(&coords) {
                print!("{}", Self::DL_SCORE_COLOR);
            } else if Self::CENTER_SQUARE == coords {
                print!("{}", Self::CENTER_SQUARE_COLOR);
            }
            print!("{}{} ", e, Self::TEXT_COLOR_DEF);
            if (i + 1) % Self::COL_LENGTH == 0 {
                println!("|")
            }
        }
        println!("-----------------------------------");
        println!("{}", Self::TERMINAL_COLOR_DEFAULT);
    }

    pub fn place_word(
        &mut self,
        word: &str,
        direction: Direction,
        start_x: usize,
        start_y: usize,
    ) -> anyhow::Result<()> {
        if start_x >= Self::COL_LENGTH || start_y >= Self::ROW_LENGTH {
            return Err(anyhow!(
                "({}, {}) is out of bounds ({}, {})",
                start_x,
                start_y,
                Self::COL_LENGTH,
                Self::ROW_LENGTH,
            ));
        }

        match direction {
            Direction::Vertical => {
                if start_y + word.len() >= Self::COL_LENGTH {
                    return Err(anyhow!(
                        "({}, {}) is out of bounds ({}, {})",
                        start_y,
                        start_x,
                        Self::COL_LENGTH,
                        Self::ROW_LENGTH,
                    ));
                }

                for (i, c) in word.as_bytes().iter().enumerate() {
                    self.set_char(start_x + i, start_y, *c as char)?;
                }
            }
            Direction::Horizontal => {
                if start_x + word.len() >= Self::ROW_LENGTH {
                    return Err(anyhow!(
                        "({}, {}) is out of bounds ({}, {})",
                        start_y,
                        start_x,
                        Self::COL_LENGTH,
                        Self::ROW_LENGTH,
                    ));
                }

                for (i, c) in word.as_bytes().iter().enumerate() {
                    self.set_char(start_x, start_y + i, *c as char)?;
                }
            }
        }

        Ok(())
    }

    fn get_char_i(&self, i: usize) -> char {
        self.elements[i]
    }

    fn get_char(&self, y: usize, x: usize) -> char {
        self.elements[y * Self::ROW_LENGTH + x]
    }

    fn set_char(&mut self, y: usize, x: usize, c: char) -> anyhow::Result<()> {
        if x < Self::COL_LENGTH && y < Self::ROW_LENGTH {
            let index = Self::coords_to_index(x, y)?;
            self.elements[index] = c;
            Ok(())
        } else {
            Err(anyhow!("Coordinates out of bounds"))
        }
    }

    fn set_char_i(&mut self, i: usize, c: char) -> anyhow::Result<()> {
        if i < Self::BOARD_SIZE {
            self.elements[i] = c;
            Ok(())
        } else {
            Err(anyhow!("Index out of bounds"))
        }
    }
}
