use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{fmt, thread};

#[derive(Clone, Serialize)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<usize>,
    pub fixed_cells: Vec<(usize, usize)>, // fixed cells (index, value)
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let box_size = (self.size as f64).sqrt() as usize;
        let cell_width = self.size.to_string().len().max(2); // adjusts spacing for large sizes

        let horizontal_line = if box_size > 1 {
            "-".repeat((cell_width + 1) * self.size + box_size + 1)
        } else {
            "-".repeat((cell_width + 1) * self.size)
        };

        for y in 0..self.size {
            if y % box_size == 0 && y != 0 {
                writeln!(f, "{}", horizontal_line)?;
            }

            for x in 0..self.size {
                if x % box_size == 0 && x != 0 {
                    write!(f, "| ")?;
                }

                let idx = self.index(x, y);
                let value = self.cells[idx];
                if value == 0 {
                    write!(f, "{:>width$} ", "□", width = cell_width)?;
                } else {
                    write!(f, "{:>width$} ", value, width = cell_width)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Board {
    #![allow(dead_code)]
    /// Returns all possible solutions for the current Sudoku board.
    pub fn get_all_solutions(&mut self) -> Vec<Vec<usize>> {
        let mut solutions = Vec::new();
        self.solve_all_recursive(&mut solutions);
        solutions
    }

    /// Returns all possible solutions for the current Sudoku board using parallel processing.
    pub fn get_all_solutions_parallel(&self) -> Vec<Vec<usize>> {
        let empty_idx = self.cells.iter().position(|&x| x == 0);

        if empty_idx.is_none() {
            return vec![self.cells.clone()];
        }

        let idx = empty_idx.unwrap();
        let num_threads = num_cpus::get();

        let tasks: Vec<Board> = (1..=self.size)
            .filter(|&v| self.check_if_valid(idx, v))
            .map(|v| {
                let mut new_board = self.clone();
                new_board.cells[idx] = v;
                new_board
            })
            .collect();

        let tasks = Arc::new(Mutex::new(tasks));
        let solutions = Arc::new(Mutex::new(Vec::new()));

        let mut handles = Vec::new();

        for _ in 0..num_threads {
            let tasks_clone = Arc::clone(&tasks);
            let solutions_clone = Arc::clone(&solutions);

            let handle = thread::spawn(move || {
                while let Some(mut board) = {
                    let mut locked = tasks_clone.lock().unwrap();
                    locked.pop()
                } {
                    let mut result = board.get_all_solutions();
                    let mut sol_locked = solutions_clone.lock().unwrap();
                    sol_locked.append(&mut result);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_solutions = Arc::try_unwrap(solutions).unwrap().into_inner().unwrap();
        final_solutions
    }

    /// Generates a new Sudoku board with the specified size and number of clues.
    pub fn generate_new_sudoku(size: usize, number_of_clues: usize) -> Board {
        use rand::Rng;
        let mut board = Board {
            size,
            cells: vec![0; size * size],
            fixed_cells: vec![],
        };
        let mut clues_added = 0;
        let mut rng = rand::rng();

        while clues_added < number_of_clues {
            let row = rng.random_range(0..size);
            let col = rng.random_range(0..size);
            let num = rng.random_range(1..=size);
            let idx = board.index(col, row);

            if board.cells[idx] == 0 && board.check_if_valid(idx, num) {
                board.cells[idx] = num;
                board.fixed_cells.push((idx, num));
                clues_added += 1;
            }
        }

        if board.check_if_possible() {
            board
        } else {
            Board::generate_new_sudoku(size, number_of_clues)
        }
    }

    /// Checks if the current Sudoku board is solvable.
    fn check_if_possible(&self) -> bool {
        let mut board_to_check = self.clone();
        if board_to_check.solve_brute_force() {
            true
        } else {
            false
        }
    }

    /// Imports a Sudoku board from a file.
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

    /// Helper function to convert (x, y) coordinates to a linear index.
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    /// Solves the Sudoku board using a brute-force backtracking algorithm.
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

    /// Recursive helper function to find all solutions for the Sudoku board.
    fn solve_all_recursive(&mut self, solutions: &mut Vec<Vec<usize>>) {
        for idx in 0..self.cells.len() {
            if self.cells[idx] == 0 {
                for value in 1..=self.size {
                    if self.check_if_valid(idx, value) {
                        self.cells[idx] = value;
                        self.solve_all_recursive(solutions);
                        self.cells[idx] = 0;
                    }
                }
                return;
            }
        }
        solutions.push(self.cells.clone());
    }

    /// Checks if placing a value at a given index is valid according to Sudoku rules.
    fn check_if_valid(&self, idx: usize, value: usize) -> bool {
        !self.exists_in_row(idx, value)
            && !self.exists_in_column(idx, value)
            && !self.exists_in_box(idx, value)
    }

    /// Checks if a value exists in the same row as the given index.
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

    /// Checks if a value exists in the same column as the given index.
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

    /// Checks if a value exists in the same box as the given index.
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
}
