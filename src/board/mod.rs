pub struct Board { elements: [char; Self::BOARD_SIZE], }

impl Board {
    // Size constants
    const BOARD_SIZE: usize = Self::ROW_LENGTH * Self::COL_LENGTH;
    const ROW_LENGTH: usize = 15;
    const COL_LENGTH: usize = 15;

    fn index_to_coords(i: usize) -> Result<(usize, usize), String> {
        if i < Self::ROW_LENGTH * Self::COL_LENGTH {
            let y = i / Self::COL_LENGTH;
            let x = i % Self::COL_LENGTH;
            Ok((x, y))
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    fn coords_to_index(x: usize, y: usize) -> Result<usize, String> {
        if x < Self::COL_LENGTH && y < Self::ROW_LENGTH {
            Ok(y * Self::COL_LENGTH + x)
        } else {
            Err("Coords out of index".to_string())
        }
    }

    // Print constants
    // (See https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797)
    const TERMINAL_COLOR_DEFAULT: &str = "\x1b[39m\x1b[49m";
    const CENTER_SQUARE_COLOR:    &str = "\x1b[48;5;226m\x1b[38;5;16m";
    const TEXT_COLOR_DEF:         &str = "\x1b[39m\x1b[49m";
    const TW_SCORE_COLOR:         &str = "\x1b[48;5;161m\x1b[38;5;16m";
    const DW_SCORE_COLOR:         &str = "\x1b[48;5;217m\x1b[38;5;16m";
    const TL_SCORE_COLOR:         &str = "\x1b[48;5;32m\x1b[38;5;16m";
    const DL_SCORE_COLOR:         &str = "\x1b[45;5;51m\x1b[38;5;16m";

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

        // Get the terminal size
        let size = termsize::get().expect("We should always be able to get the terminal size!");

        let row_padding = (size.rows as usize).saturating_sub(18) / 2;

        print!("{}", Self::TEXT_COLOR_DEF);
        print_n_char(row_padding, ' ');
        println!();
        print_n_char(row_padding, ' ');
        println!("-----------------------------------");
        print_n_char(row_padding, ' ');
        println!("|            WORDSMITH            |");
        print_n_char(row_padding, ' ');
        println!("-----------------------------------");
        print_n_char(row_padding, ' ');
        print!("|   ");
        for i in 0..Self::COL_LENGTH {
            print!("{:X} ", i);
        }
        println!("|");

        let mut row: usize = 0;
        for (i, e) in self.elements.iter().enumerate() {
            if i % Self::COL_LENGTH == 0 {
                print_n_char(row_padding, ' ');
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
        print_n_char(row_padding, ' ');
        println!("-----------------------------------");
        println!("{}", Self::TERMINAL_COLOR_DEFAULT);
    }

    fn get_char_i(&self, i: usize) -> char {
        self.elements[i]
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        self.elements[y * Self::ROW_LENGTH + x]
    }

    fn set_char(&mut self, x: usize, y: usize, c: char) -> Result<(), String> {
        if x < Self::COL_LENGTH && y < Self::ROW_LENGTH {
            let index = Self::coords_to_index(x, y)?;
            self.elements[index] = c;
            Ok(())
        } else {
            Err("Coordinates out of bounds".to_string())
        }
    }

    fn set_char_i(&mut self, i: usize, c: char) -> Result<(), String> {
        if i < Self::BOARD_SIZE {
            self.elements[i] = c;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }
}
