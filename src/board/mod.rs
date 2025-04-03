pub struct Board {
    elements: [char; Self::BOARD_SIZE],
}

impl Board {
    // Size constants
    const BOARD_SIZE: usize = Self::ROW_LENGTH * Self::COL_LENGTH;
    const ROW_LENGTH: usize = 15;
    const COL_LENGTH: usize = 15;

    // Print constants (i.e. colors)

    // Positions

    pub fn new() -> Self {
        Self {
            elements: ['_'; Self::BOARD_SIZE]
        }
    }

    // TODO: Center the board display in the middle of the terminal...
    // Maybe use Crossterm? Or maybe just use Rust std::???
    pub fn display(&self) {
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
                row +=1;
            }
            print!("{} ", e);
            if (i+1) % Self::COL_LENGTH == 0 {
                println!("|")
            }
        }
        println!("-----------------------------------");
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        return self.elements[y * Self::ROW_LENGTH + x]
    }
}