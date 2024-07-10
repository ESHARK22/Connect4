use array2d::Array2D;
use inline_colorization::*;

mod input_handlers;
use input_handlers::int_input;

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

struct Game {
    board: Array2D<BoardState>,
    empty_character: String,
}
impl Game {
    fn print_board(&self) {
        // Simply prints the board.

        let board = &self.board;
        let empty_char = &self.empty_character;

        // Print the top table lablelling (0, 1, 2..)
        print!("    ");
        for col_index in 0..board.num_columns() {
            print!(" {col_index} ")
        }
        println!("[x] {style_reset}");

        // Print the top table formattings (+ - - - +)
        print!("{color_green}   +");
        for _ in 0..board.num_columns() {
            print!("---")
        }
        println!("+{style_reset}");

        // Print each row, labelled
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
                            print!("{} {} {}", color_white, "-", style_reset)
                        }
                        BoardState::Taken(player) => {
                            print!("{} {} {}", player.colour, player.character, style_reset)
                        }
                    },
                }
            }
            println!("{color_green}|{style_reset}")
        }

        // Print the bottom table formattings (+ - - - +)
        print!("{style_reset}[y]{color_green}+");
        for _ in 0..board.num_columns() {
            print!("---")
        }
        println!("+{style_reset}");
    }
}

#[derive(Debug, Clone)]
enum BoardState {
    Taken(Player),
    Empty,
}

#[derive(Debug, Clone)]
struct Player {
    name: String,
    character: String,
    colour: String,
}
impl Player {
    fn play_turn(self, game: &mut Game) {
        let board = &mut game.board;
        let name = self.name.clone();
        loop {
            println!("It is {}'s turn!", name);

            let col_index = int_input("Enter the column you would like to go in: ");
            let row_index = int_input("Enter the row you would like to go in: "); // TODO: Remove, since should only be able to place at the botoom

            let current_state = board.get_mut(row_index, col_index);
            match current_state {
                Some(current_state) => match current_state {
                    BoardState::Empty => {
                        board[(row_index, col_index)] = BoardState::Taken(self.clone());
                        return;
                    }
                    BoardState::Taken(player) => {
                        println!("That space is already taken by {}!", player.name);
                        println!("Try again...");
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

fn main() {
    println!("Hello from connect 4!");

    let mut game = Game {
        board: Array2D::filled_with(BoardState::Empty, 6, 7),
        empty_character: "-".into(),
    };

    let player1 = Player {
        name: "Player 1".into(),
        character: "O".into(),
        colour: format!("{bg_bright_yellow}").into(),
    };

    let player2 = Player {
        name: "Player 2".into(),
        character: "X".into(),
        colour: format!("{bg_bright_red}").into(),
    };
    loop {
        game.print_board();
        player1.clone().play_turn(&mut game);
        println!("");
        game.print_board();
        player2.clone().play_turn(&mut game);
    }
}
