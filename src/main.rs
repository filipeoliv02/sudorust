use sudorust::board::Board;

fn main() {
    match Board::generate_new_sudoku(9, 25) {
        Ok(board) => println!("{board}"),
        Err(error) => {
            eprintln!("Failed to generate Sudoku: {error}");
            std::process::exit(1);
        }
    }
}
