pub struct Board {
    size: usize,
    cells: Vec<usize>,
    // fixed cells (index, value)
    fixed_cells: Vec<(usize, usize)>,
}

impl Board {
    pub fn display(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let idx = self.index(x, y);
                print!("{} ", self.cells[idx]);
            }
            println!();
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    fn check_if_valid(&self, x: usize, y: usize, value: usize) -> bool {
        let idx = self.index(x, y);
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

    fn set_value(&mut self, x: usize, y: usize, value: usize) {
        let idx = self.index(x, y);
        self.cells[idx] = value;
    }

    fn get_value(&self, x: usize, y: usize) -> usize {
        let idx = self.index(x, y);
        self.cells[idx]
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
