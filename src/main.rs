mod board;
use board::Board as boardImpl;

fn main() {
    let board1 = boardImpl::generate_new_sudoku(9, 25);
    println!("{}", board1);
}
