use array2d::Array2D;
use dialoguer::{theme::ColorfulTheme, Select};
use inline_colorization::*;

mod input_handlers;
use input_handlers::{input, int_input};

// Connect 4 Rules - edited from https://rulesofplaying.com/connect-4-rules/
//  - tic-tac-toe game played by two players.
//  - Players take turns placing pieces on a vertical board.
//  - The board is 7 columns long and 6 rows high.
//  - Each player uses pieces of a specific color, usually black and red or sometimes yellow and red.
// The goal is to be the first to get four pieces in a horizontal, vertical, or diagonal line.
// Since the board is vertical, parts inserted in a certain column always fall in the lowest unoccupied row in that column.
// As soon as a column contains six parts, it is full, and no further parts can be placed on the column.
// Both the players begin with 21 similar pieces, and the first player to reach a series of four connected pieces wins the game.
// If all the men have played and neither player has four parts in a row, the game is a tie.

#[derive(Debug, Clone)]
struct Game {
    name: String,
    board: Board,
    empty_character: String,
    player_turn: PlayerTurn,
}
impl Game {
    fn next_player(&mut self) {
        match self.player_turn {
            PlayerTurn::Player1 => self.player_turn = PlayerTurn::Player2,
            PlayerTurn::Player2 => self.player_turn = PlayerTurn::Player1,
        }
    }
    fn print_board(&self) {
        self.board.print(self.empty_character.clone());
    }
}
#[derive(Debug, Clone)]
struct Board(Array2D<BoardState>);
impl Board {
    fn print(&self, empty_char: String) {
        // Simply prints the board, with formatting, and colours.

        // A reference to the 2d array
        let board = &self.0;

        // Print the top table lablelling
        // ->      0  1  2  3  4  5  6 [x]
        print!("    ");
        for col_index in 0..board.num_columns() {
            print!(" {col_index} ")
        }
        println!("[x] {style_reset}");

        // Print the top table formattings (+ - - - +)
        // ->    +---------------------+
        print!("{color_green}   +");
        for _ in 0..board.num_columns() {
            print!("---")
        }
        println!("+{style_reset}");

        // Print each row, labelled
        // ->  5 | -  -  X  -  -  O  - |
        for row_index in 0..board.num_rows() {
            // The row lablelling
            print!(" {row_index} {color_green}|{style_reset}");

            for col_index in 0..board.num_columns() {
                // Print each section of the board, with the users colour, or none if its empty
                let state = board.get(row_index, col_index);
                match state {
                    None => panic!("Tried to print a space that doesnt exist?!"),
                    Some(state) => match state {
                        BoardState::Empty => {
                            print!("{} {} {}", color_white, empty_char, style_reset)
                        }
                        BoardState::Taken(player) => {
                            print!("{} {} {}", player.colour, player.character, style_reset)
                        }
                    },
                }
            }
            println!("{color_green}|{style_reset}")
        }

        // Print the bottom table formattings
        // -> [y]+---------------------+
        print!("{style_reset}[y]{color_green}+");
        for _ in 0..board.num_columns() {
            print!("---")
        }
        println!("+{style_reset}");
    }
    fn is_full(&self) -> bool {
        // The board is full when there are no more empty spaces
        !self
            .0
            .elements_row_major_iter()
            .any(|f| f == &BoardState::Empty)
    }
    fn is_at_bottom(&self, row: usize, column: usize) -> bool {
        let board = &self.0;
        // Check if anything exists below, and if it does, make sure it is not empty
        // If it is empty, then we are not at the bottom
        match board.get(row + 1, column) {
            Some(&ref state) => {
                match state {
                    BoardState::Taken(_) => true, // Cant go any lower
                    BoardState::Empty => false,   // Could have gone lower
                }
            }
            None => true, // Nothing exists below it
        }
    }
}

fn check_horizontal_wins(board: Array2D<BoardState>) -> Option<Player> {
    // Check for 4 in a row, on all rows

    // As close to the right as we can check for 4 in a row
    let max_col_index = board.num_columns() - 3;

    // For each row
    for row_index in 0..board.num_rows() {
        // For each column *we need to check*
        // (impossible to win when there are only 3 or less existing spaces to the right)
        for col_index in 0..max_col_index {
            // Its fine to unwrap here, since if the item doesnt exist, something is wrong with max_col_index
            let item1 = board.get(row_index, col_index).unwrap().clone();
            let item2 = board.get(row_index, col_index + 1).unwrap().clone();
            let item3 = board.get(row_index, col_index + 2).unwrap().clone();
            let item4 = board.get(row_index, col_index + 3).unwrap().clone();

            if let BoardState::Taken(player) = item1.clone() {
                if item1 == item2 && item1 == item3 && item1 == item4 {
                    // We found 4 in a row!
                    return Some(player.clone());
                }
            } else {
                // Empty space, continue searching for a winner
                continue;
            }
        }
    }

    None // No wins were found
}

fn check_vertical_wins(board: Array2D<BoardState>) -> Option<Player> {
    // How low down we can go, where the is still 4 items to check
    let max_row_index = board.num_columns() - 4;

    // For each column
    for col_index in 0..board.num_columns() {
        for row_index in 0..max_row_index {
            let item1 = board.get(row_index, col_index).unwrap().clone();
            let item2 = board.get(row_index + 1, col_index).unwrap().clone();
            let item3 = board.get(row_index + 2, col_index).unwrap().clone();
            let item4 = board.get(row_index + 3, col_index).unwrap().clone();

            if let BoardState::Taken(player) = item1.clone() {
                if item1 == item2 && item1 == item3 && item1 == item4 {
                    return Some(player.clone());
                }
            } else {
                // Empty space
                continue;
            }
        }
    }

    None
}

fn check_diagnal_wins(board: Array2D<BoardState>) -> Option<Player> {
    // First focus on ones going from top left to bottom right
    let max_row_index = board.num_columns() - 4;
    let max_col_index = board.num_columns() - 3;

    // For each row
    for row_index in 0..max_row_index {
        // For each column
        for col_index in 0..max_col_index {
            let item1 = board.get(row_index, col_index).unwrap().clone();
            let item2 = board.get(row_index + 1, col_index + 1).unwrap().clone();
            let item3 = board.get(row_index + 2, col_index + 2).unwrap().clone();
            let item4 = board.get(row_index + 3, col_index + 3).unwrap().clone();

            if let BoardState::Taken(player) = item1.clone() {
                if item1 == item2 && item1 == item3 && item1 == item4 {
                    return Some(player.clone());
                }
            } else {
                // Empty space
                continue;
            }
        }
    }

    let min_col_index = board.num_columns() - max_col_index;

    for row_index in 0..max_row_index {
        for col_index in (min_col_index..board.num_columns()).rev() {
            let item1 = board.get(row_index, col_index).unwrap().clone();
            let item2 = board.get(row_index + 1, col_index - 1).unwrap().clone();
            let item3 = board.get(row_index + 2, col_index - 2).unwrap().clone();
            let item4 = board.get(row_index + 3, col_index - 3).unwrap().clone();

            if let BoardState::Taken(player) = item1.clone() {
                if item1 == item2 && item1 == item3 && item1 == item4 {
                    return Some(player.clone());
                }
            } else {
                // Empty space
                continue;
            }
        }
    }

    None
}

fn check_wins(board: Array2D<BoardState>) -> Option<Player> {
    match check_horizontal_wins(board.clone()) {
        Some(player) => return Some(player),
        None => {}
    }
    match check_vertical_wins(board.clone()) {
        Some(player) => return Some(player),
        None => {}
    }
    match check_diagnal_wins(board.clone()) {
        Some(player) => return Some(player),
        None => {}
    }

    None
}

fn check_tie(board: Board) -> bool {
    board.is_full()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoardState {
    Taken(Player),
    Empty,
}

#[derive(Debug, Clone)]
enum PlayerTurn {
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Player {
    name: String,
    character: String,
    colour: String,
}
impl Player {
    fn play_turn(self, game: &mut Game) {
        let board = game.board.clone();
        let board_arr = &mut game.board.0;
        let name = self.name.clone();

        loop {
            let mut col_index: usize;
            loop {
                println!("It is {}'s turn!", name);
                col_index = int_input("Enter the column you would like to go in: ");
                let current_state = board_arr.get_mut(0, col_index); // Start at the top
                match current_state {
                    Some(_) => break, // It exists
                    None => {
                        println!("Stop trying to play outside the board -_-");
                        println!("Try again...");
                    }
                }
            }
            // Find the lowest cords you can go to
            let mut row_index = 0;
            while !board.is_at_bottom(row_index, col_index) {
                row_index += 1
            }

            let current_state = board_arr.get_mut(row_index, col_index);
            match current_state {
                Some(current_state) => match current_state {
                    BoardState::Empty => {
                        board_arr[(row_index, col_index)] = BoardState::Taken(self.clone());
                        return;
                    }
                    BoardState::Taken(_) => {
                        println!("That column is already full!");
                        println!("Try a different one...");
                    }
                },
                None => {
                    // Does not exist on the board
                    println!("Oi, stop trying to play outside the box!");
                    println!("Try again...");
                }
            }
        }
    }
}

fn new_game() {
    let game_name = input("What would you like to call this game save?");

    let player1 = Player {
        name: "Player 1".into(),
        character: "O".into(),
        colour: format!("{color_bright_yellow}").into(),
    };

    let player2 = Player {
        name: "Player 2".into(),
        character: "X".into(),
        colour: format!("{color_bright_red}").into(),
    };

    let mut game = Game {
        name: game_name,
        board: Board(Array2D::filled_with(BoardState::Empty, 6, 7)),
        empty_character: "-".into(),
        player_turn: PlayerTurn::Player1,
    };

    loop {
        let player: Player;
        match game.player_turn {
            PlayerTurn::Player1 => player = player1.clone(),
            PlayerTurn::Player2 => player = player2.clone(),
        }
        game.print_board();
        player.clone().play_turn(&mut game);
        println!("");

        match check_wins(game.board.0.clone()) {
            None => {}
            Some(player) => {
                game.print_board();
                let player_name = player.name;
                println!("__________________________");
                println!("{player_name} won the game!!!");
                return;
            }
        }

        match check_tie(game.board.clone()) {
            false => {}
            true => {
                println!("__________________________");
                println!("The game is a tie!!!");
                return;
            }
        }
        game.next_player();
    }
}

fn load_game() {}

fn main() {
    let options = &["Start a new game", "Load a game save"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => new_game(),
        1 => load_game(),
        _ => panic!("What menu item did you click?"),
    }
}
