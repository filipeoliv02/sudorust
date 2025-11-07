mod board;

fn main() {
        let mut board1 = board::board::Board::import_from_file("data/single9x9.txt");
        board1.display();
        _ = board1.solve_brute_force();
        println!("\n\nSolved Board:\n");
        board1.display();
}
