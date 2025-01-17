use std::io::{self, Write};
use rand::Rng;

const BOARD_SIZE: usize = 10;
const LETTERS: [&str; 10] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
    Sunken,
}

struct Ship {
    positions: Vec<(usize, usize)>,
    is_sunken: bool,
}

struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<Ship>,
}

impl Board {
    // Function to initialize a new game board
    fn new() -> Self {
        Board {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }
    // Function to place a ship on the board
    fn place_ship(&mut self, size: usize) {
        let mut rng = rand::thread_rng();

        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            let direction = rng.gen::<bool>();

            if self.can_place_ship(row, col, size, direction) {
                let mut positions = Vec::new();
                for i in 0..size {
                    let (r, c) = if direction { (row, col + i) } else { (row + i, col) };
                    self.grid[r][c] = CellState::Ship;
                    positions.push((r, c));
                }
                self.ships.push(Ship { positions, is_sunken: false });
                break;
            }
        }
    }

    fn can_place_ship(&self, row: usize, col: usize, size: usize, direction: bool) -> bool {
        let is_valid = |r: usize, c: usize| -> bool {
            r < BOARD_SIZE && c < BOARD_SIZE && self.grid[r][c] == CellState::Empty
        };

        if direction {
            if col + size > BOARD_SIZE { return false; }
            for i in 0..size {
                if !is_valid(row, col + i) || !self.is_area_clear(row, col + i) {
                    return false;
                }
            }
        } else {
            if row + size > BOARD_SIZE { return false; }
            for i in 0..size {
                if !is_valid(row + i, col) || !self.is_area_clear(row + i, col) {
                    return false;
                }
            }
        }

        true
    }

    fn is_area_clear(&self, row: usize, col: usize) -> bool {
        let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        for &(dx, dy) in &offsets {
            let (new_row, new_col) = (row as isize + dx, col as isize + dy);
            if new_row >= 0 && new_row < BOARD_SIZE as isize && new_col >= 0 && new_col < BOARD_SIZE as isize {
                if self.grid[new_row as usize][new_col as usize] == CellState::Ship {
                    return false;
                }
            }
        }

        true
    }

    fn fire(&mut self, row: usize, col: usize) -> CellState {
        match self.grid[row][col] {
            CellState::Empty => {
                self.grid[row][col] = CellState::Miss;
                CellState::Miss
            },
            CellState::Ship => {
                self.grid[row][col] = CellState::Hit;
                CellState::Hit
            },
            _ => CellState::Miss,
        }
    }

    fn check_for_sunken_ships(&mut self) {
        for ship in &mut self.ships {
            if ship.is_sunken {
                continue;
            }
            
            let mut all_hit = true;
            for &(r, c) in &ship.positions {
                if self.grid[r][c] != CellState::Hit {
                    all_hit = false;
                    break;
                }
            }

            if all_hit {
                for &(r, c) in &ship.positions {
                    self.grid[r][c] = CellState::Sunken;
                }
                ship.is_sunken = true;
                println!("\x1b[1;31mShip sunk!\x1b[0m");
            }
        }
    }

    fn display(&self, hide_ships: bool) {
        print!("   ");
        for i in 0..BOARD_SIZE { print!(" {} ", i); }
        println!();
        for (i, row) in self.grid.iter().enumerate() {
            print!("{:2} ", LETTERS[i]);
            for cell in row {
                match cell {
                    CellState::Empty => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!(" \u{25A1} ");                      
                        }
                    }
                    CellState::Ship => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!("\x1b[32m \u{25A0} \x1b[0m");                    
                        }
                    }
                    CellState::Hit => print!("\x1b[31m \u{25CF} \x1b[0m"),                   
                    CellState::Miss => print!("\x1b[36m \u{2981} \x1b[0m"),                         
                    CellState::Sunken => print!("\x1b[31m \u{2A02} \x1b[0m"),           
                }
            }
            println!();
        }
    }

    fn is_game_over(&self) -> bool {
        self.ships.iter().all(|ship| ship.is_sunken)
    }
}

fn main() {
    let mut player_board = Board::new();
    let mut opponent_board = Board::new();

    player_board.place_ship(4);                                      
    for _ in 0..2 { player_board.place_ship(3); }                   
    for _ in 0..3 { player_board.place_ship(2); }                   
    for _ in 0..4 { player_board.place_ship(1); }      

    opponent_board.place_ship(4);                     
    for _ in 0..2 { opponent_board.place_ship(3); }
    for _ in 0..3 { opponent_board.place_ship(2); }
    for _ in 0..4 { opponent_board.place_ship(1); }

    let mut player_turn = true;

    loop {
        print!("\x1b[2J\x1b[1;1H");

        println!("\x1b[1;37mYour Board:\x1b[0m");
        player_board.display(false);
        println!("\x1b[1;37mOpponent's Board:\x1b[0m");
        opponent_board.display(true);

        if player_turn {
            let (player_row, player_col) = get_player_input();
            let result = opponent_board.fire(player_row, player_col);
            match result {
                CellState::Miss => {
                    println!("\x1b[36mYou missed!\x1b[0m");
                    player_turn = false;
                },
                CellState::Hit => {
                    println!("\x1b[31mYou hit a ship!\x1b[0m");
                    opponent_board.check_for_sunken_ships();
                },
                _ => (),
            }
            if opponent_board.is_game_over() {
                println!("\x1b[1;32mCongratulations! You sank all of your opponent's ships!\x1b[0m");
                break;
            }
        } else {
            let (opponent_row, opponent_col) = generate_opponent_move();
            let result = player_board.fire(opponent_row, opponent_col);
            match result {
                CellState::Miss => {
                    println!("\x1b[36mOpponent missed!\x1b[0m");
                    player_turn = true;
                },
                CellState::Hit => {
                    println!("\x1b[31mOpponent hit one of your ships!\x1b[0m");
                    player_board.check_for_sunken_ships();
                },
                _ => (),
            }
            if player_board.is_game_over() {
                println!("\x1b[1;31mOh no! All of your ships have been sunk.\x1b[0m");
                break;
            }
        }
        print!("Press Enter to continue...");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
    }
}

fn get_player_input() -> (usize, usize) {
    loop {
        print!("Enter your shot (row and column, e.g., A 1): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some((row, col)) = parse_input(&input) {
            if row < BOARD_SIZE && col < BOARD_SIZE {
                return (row, col);
            }
        }
        println!("Invalid input. Try again.");
    }
}

fn parse_input(input: &str) -> Option<(usize, usize)> {
    let mut parts = input.trim().split_whitespace();
    if let (Some(letter), Some(col)) = (parts.next(), parts.next()) {
        let row = LETTERS.iter().position(|&l| l == letter.to_uppercase()).unwrap_or(BOARD_SIZE);
        let col = col.parse::<usize>().unwrap_or(BOARD_SIZE);
        if row < BOARD_SIZE && col < BOARD_SIZE {
            return Some((row, col));
        }
    }
    None
}

fn generate_opponent_move() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE))
}