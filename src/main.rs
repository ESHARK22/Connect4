use array2d::Array2D;

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

fn main() {
    println!("Hello from connect 4!");

    let empty_character = "-";

    // Create a new 2d array
    let game_board = Array2D::filled_with(empty_character, 6, 7);
    println!("{:?}", game_board)
}
