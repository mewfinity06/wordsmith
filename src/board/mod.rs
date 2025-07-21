#![allow(dead_code)]

use anyhow::{Context, anyhow};

pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct Board {
    elements: [[char; Self::ROW_LENGTH]; Self::COL_LENGTH],
}

impl Board {
    // Size constants
    const ROW_LENGTH: usize = 15;
    const COL_LENGTH: usize = 15;

    fn index_to_coords(i: usize) -> anyhow::Result<(usize, usize)> {
        Self::bounds_check_index(i)?;

        let y = i / Self::ROW_LENGTH;
        let x = i % Self::COL_LENGTH;

        Ok((y, x))
    }

    fn coords_to_index(y: usize, x: usize) -> anyhow::Result<usize> {
        Self::bounds_check_coords(y, x)?;
        Ok(y * Self::ROW_LENGTH + x)
    }

    fn bounds_check_coords(y: usize, x: usize) -> anyhow::Result<()> {
        if x > Self::COL_LENGTH || y > Self::ROW_LENGTH {
            Err(anyhow!(
                "(y: {:X}, x: {:X}) is out of bounds. Expected (y: {:X}, x: {:X})",
                y,
                x,
                Self::ROW_LENGTH,
                Self::COL_LENGTH
            ))
        } else {
            Ok(())
        }
    }

    fn bounds_check_index(i: usize) -> anyhow::Result<()> {
        if i >= Self::ROW_LENGTH * Self::COL_LENGTH {
            Err(anyhow!("Index out of bounds"))
        } else {
            Ok(())
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
        let mut elements = [['_'; Self::ROW_LENGTH]; Self::COL_LENGTH];

        elements[Self::CENTER_SQUARE.1][Self::CENTER_SQUARE.0] = Self::STAR_CHAR;

        Self { elements }
    }

    /// Displays the board in a 15x15 grid
    /// ```
    ///     0x0 0x1 0x2 ...
    /// 0x0 '_' '_' '_'
    /// 0x1 '_' '_' '_'
    /// 0x2 '_' '_' '_'
    /// ...
    /// ```
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

        for (index_r, row) in self.elements.iter().enumerate() {
            print!("| {:X} ", index_r);

            for (index_c, c) in row.iter().enumerate() {
                let coords = (index_r, index_c);

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
                print!("{}{} ", c, Self::TEXT_COLOR_DEF)
            }

            println!("|");
        }

        println!("-----------------------------------");
        println!("{}", Self::TERMINAL_COLOR_DEFAULT);
    }

    pub fn place_word(
        &mut self,
        word: &str,
        y: usize,
        x: usize,
        dir: Direction,
    ) -> anyhow::Result<()> {
        match dir {
            Direction::Horizontal => self.place_word_horizontal(word, y, x),
            Direction::Vertical => self.place_word_vertical(word, y, x),
        }
    }

    fn place_word_horizontal(&mut self, word: &str, y: usize, x: usize) -> anyhow::Result<()> {
        // Check if start coords are in bounds
        Self::bounds_check_coords(y, x)?;

        // Check if end coords are in bounds
        Self::bounds_check_coords(y, x + word.len()).context(format!(
            "Word '{}' len does not pass bounds check | Start: (y: {:X}, x: {:X}), End: (y: {:X}, x: {:X})",
            word,
            y,
            x,
            y,
            x + word.len()
        ))?;

        // If both succeed, then we must be able to place the word
        let row = &mut self.elements[y];
        let mut word_chars = word.chars();
        for i in 0..word.len() {
            if let Some(c) = word_chars.next() {
                let cell = &mut row[i + x];

                // Check if letters match
                if *cell != '_' && *cell != '*' && *cell != c {
                    return Err(anyhow!("Conflicting letter found {} in '{}'", c, word));
                }

                *cell = c;
            }
        }

        Ok(())
    }

    fn place_word_vertical(&mut self, word: &str, y: usize, x: usize) -> anyhow::Result<()> {
        // Check if start coords are in bounds
        Self::bounds_check_coords(y, x)?;

        // Check if end coords are in bounds
        Self::bounds_check_coords(y + word.len(), x).context(format!(
            "Word '{}' len does not pass bounds check\n| Start: (y: {:X}, x: {:X}), End: (y: {:X}, x: {:X})",
            word,
            y,
            x,
            y + word.len(),
            x,
        ))?;

        // If both succeed, then we must be able to place the word
        for i in 0..word.len() {
            if let Some(row) = self.elements.get_mut(y + i) {
                if let Some(cell) = row.get_mut(x) {
                    let c = word.chars().nth(i).unwrap();

                    if *cell != '_' && *cell != '*' && *cell != c {
                        return Err(anyhow!("Conflicting letter found {} in '{}'", c, word));
                    }

                    *cell = c;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test;
