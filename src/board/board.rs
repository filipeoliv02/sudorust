use std::fs::File;
use std::io::Read;

pub struct Board {
    size: usize,
    cells: Vec<usize>,
    // fixed cells (index, value)
    fixed_cells: Vec<(usize, usize)>,
}

impl Board {
    pub fn display(&self) {
        let box_size = (self.size as f64).sqrt() as usize;
        let cell_width = self.size.to_string().len().max(2); // width scales for large sizes

        let horizontal_line = if box_size > 1 {
            "-".repeat((cell_width + 1) * self.size + box_size + 1)
        } else {
            "-".repeat((cell_width + 1) * self.size)
        };

        for y in 0..self.size {
            if y % box_size == 0 && y != 0 {
                println!("{}", horizontal_line);
            }

            for x in 0..self.size {
                if x % box_size == 0 && x != 0 {
                    print!("| ");
                }
                let idx = self.index(x, y);
                let value = self.cells[idx];
                if value == 0 {
                    print!("{:>width$} ", "□", width = cell_width);
                } else {
                    print!("{:>width$} ", value, width = cell_width);
                }
            }
            println!();
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    pub fn solve_brute_force(&mut self) -> bool {
        for idx in 0..self.cells.len() {
            if self.cells[idx] == 0 {
                for value in 1..=self.size {
                    if self.check_if_valid(idx, value) {
                        self.cells[idx] = value;
                        if self.solve_brute_force() {
                            return true;
                        }
                        self.cells[idx] = 0;
                    }
                }
                return false;
            }
        }
        true
    }

    fn check_if_valid(&self, idx: usize, value: usize) -> bool {
        !self.exists_in_row(idx, value)
            && !self.exists_in_column(idx, value)
            && !self.exists_in_box(idx, value)
    }

    fn exists_in_row(&self, idx: usize, value: usize) -> bool {
        let row = idx / self.size;
        for x in 0..self.size {
            let current_idx = self.index(x, row);
            if self.cells[current_idx] == value {
                return true;
            }
        }
        false
    }

    fn exists_in_column(&self, idx: usize, value: usize) -> bool {
        let column = idx % self.size;
        for y in 0..self.size {
            let current_idx = self.index(column, y);
            if self.cells[current_idx] == value {
                return true;
            }
        }
        false
    }

    fn exists_in_box(&self, idx: usize, value: usize) -> bool {
        let box_size = (self.size as f64).sqrt() as usize;
        let box_row = (idx / self.size) / box_size;
        let box_col = (idx % self.size) / box_size;

        for y in 0..box_size {
            for x in 0..box_size {
                let current_x = box_col * box_size + x;
                let current_y = box_row * box_size + y;
                let current_idx = self.index(current_x, current_y);
                if self.cells[current_idx] == value {
                    return true;
                }
            }
        }
        false
    }

    pub fn import_from_file(_file_path: &str) -> Board {
        let mut file = File::open(_file_path).expect("File not found");
        let mut contents = String::new();
        _ = file.read_to_string(&mut contents);
        let mut board = Board {
            size: 0,
            cells: vec![],
            fixed_cells: vec![],
        };
        for line in contents.lines() {
            for ch in line.chars() {
                if ch.is_digit(10) {
                    let digit = ch.to_digit(10).unwrap() as usize;
                    if digit != 0 {
                        board.fixed_cells.push((board.cells.len(), digit));
                    }
                    board.cells.push(digit);
                }
            }
        }
        board.size = (board.cells.len() as f64).sqrt() as usize;
        board
    }
}

#[cfg(test)]
mod tests {
    use crate::board::board::Board;

    #[test]
    fn test_display_board() {
        let board = Board {
            size: 4,
            cells: vec![1, 2, 3, 4, 4, 3, 2, 1, 1, 2, 3, 4, 4, 3, 2, 1],
            fixed_cells: vec![],
        };
        board.display();
    }

    #[test]
    fn test_exists_in_row() {
        let board = Board {
            size: 4,
            cells: vec![1, 2, 3, 4, 4, 3, 2, 1, 1, 2, 3, 4, 4, 3, 2, 1],
            fixed_cells: vec![],
        };
        assert!(board.exists_in_row(1, 2));
        assert!(!board.exists_in_row(0, 5));
    }

    #[test]
    fn test_exists_in_column() {
        let board = Board {
            size: 4,
            cells: vec![1, 2, 3, 4, 4, 3, 2, 1, 1, 2, 3, 4, 5, 5, 2, 1],
            fixed_cells: vec![],
        };
        assert!(board.exists_in_column(0, 5));
        assert!(!board.exists_in_column(0, 3));
    }

    #[test]
    fn test_exists_in_box() {
        let board = Board {
            size: 4,
            cells: vec![1, 2, 3, 4, 4, 3, 2, 1, 1, 2, 3, 4, 5, 5, 2, 1],
            fixed_cells: vec![],
        };
        assert!(board.exists_in_box(0, 3));
        assert!(!board.exists_in_box(15, 5));
    }
}
