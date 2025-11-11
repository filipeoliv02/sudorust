mod board;

fn main() {
    let board1 = board::board::Board::generate_new_sudoku(9, 25);
    println!("{}", board1);
}
