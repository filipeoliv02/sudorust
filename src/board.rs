use rand::Rng;
use std::fmt;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<usize>,
    pub fixed_cells: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardError {
    Io(String),
    InvalidSize(usize),
    InvalidCellCount {
        expected: usize,
        actual: usize,
    },
    InvalidValue {
        index: usize,
        value: usize,
        size: usize,
    },
    InvalidBoard,
    GenerationFailed,
    MalformedInput(String),
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(message) => write!(f, "I/O error: {message}"),
            Self::InvalidSize(size) => write!(f, "invalid Sudoku size: {size}"),
            Self::InvalidCellCount { expected, actual } => {
                write!(f, "invalid cell count: expected {expected}, got {actual}")
            }
            Self::InvalidValue { index, value, size } => {
                write!(f, "invalid value {value} at index {index}; size is {size}")
            }
            Self::InvalidBoard => write!(
                f,
                "board contains duplicate values in a row, column, or box"
            ),
            Self::GenerationFailed => write!(
                f,
                "could not generate a valid Sudoku board within the attempt limit"
            ),
            Self::MalformedInput(message) => write!(f, "malformed Sudoku input: {message}"),
        }
    }
}

impl std::error::Error for BoardError {}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let box_size = square_root(self.size).unwrap_or(0);
        let cell_width = self.size.to_string().len().max(2);
        let horizontal_line = "-".repeat((cell_width + 1) * self.size + box_size.saturating_add(1));

        for y in 0..self.size {
            if y != 0 && y % box_size == 0 {
                writeln!(f, "{horizontal_line}")?;
            }
            for x in 0..self.size {
                if x != 0 && x % box_size == 0 {
                    write!(f, "| ")?;
                }
                let value = self.cells[self.index(x, y)];
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

fn square_root(n: usize) -> Option<usize> {
    if n == 0 {
        return None;
    }
    let root = (n as f64).sqrt() as usize;
    if root * root == n {
        Some(root)
    } else {
        None
    }
}

impl Board {
    pub fn new(size: usize, cells: Vec<usize>) -> Result<Self, BoardError> {
        let box_size = square_root(size).ok_or(BoardError::InvalidSize(size))?;
        if box_size == 0 {
            return Err(BoardError::InvalidSize(size));
        }
        let expected = size
            .checked_mul(size)
            .ok_or(BoardError::InvalidSize(size))?;
        if cells.len() != expected {
            return Err(BoardError::InvalidCellCount {
                expected,
                actual: cells.len(),
            });
        }
        for (index, &value) in cells.iter().enumerate() {
            if value > size {
                return Err(BoardError::InvalidValue { index, value, size });
            }
        }
        let fixed_cells = cells
            .iter()
            .enumerate()
            .filter_map(|(index, &value)| (value != 0).then_some((index, value)))
            .collect();
        let board = Self {
            size,
            cells,
            fixed_cells,
        };
        if !board.is_valid() {
            return Err(BoardError::InvalidBoard);
        }
        Ok(board)
    }

    pub fn get_all_solutions(&mut self) -> Vec<Vec<usize>> {
        if !self.is_valid() {
            return Vec::new();
        }
        let mut solutions = Vec::new();
        self.solve_all_recursive(&mut solutions);
        solutions
    }

    pub fn get_all_solutions_parallel(&self) -> Vec<Vec<usize>> {
        if !self.is_valid() {
            return Vec::new();
        }
        let mut candidates = vec![self.clone()];
        if let Some(idx) = self.mrv_empty_cell() {
            let mut branches = Vec::new();
            for value in self.candidate_values(idx) {
                let mut board = self.clone();
                board.cells[idx] = value;
                branches.push(board);
            }
            candidates = branches;
        }

        let num_threads = num_cpus::get().max(1);
        let tasks = Arc::new(Mutex::new(candidates));
        let solutions = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            let tasks = Arc::clone(&tasks);
            let solutions = Arc::clone(&solutions);
            handles.push(thread::spawn(move || {
                while let Some(mut board) = tasks
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .pop()
                {
                    let found = board.get_all_solutions();
                    solutions
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .extend(found);
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap_or(());
        }
        match Arc::try_unwrap(solutions) {
            Ok(mutex) => mutex
                .into_inner()
                .unwrap_or_else(|poisoned| poisoned.into_inner()),
            Err(arc) => arc
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .clone(),
        }
    }

    pub fn generate_new_sudoku(size: usize, number_of_clues: usize) -> Result<Board, BoardError> {
        let box_size = square_root(size).ok_or(BoardError::InvalidSize(size))?;
        if box_size == 0 {
            return Err(BoardError::InvalidSize(size));
        }
        let total = size
            .checked_mul(size)
            .ok_or(BoardError::InvalidSize(size))?;
        if number_of_clues > total {
            return Err(BoardError::InvalidCellCount {
                expected: total,
                actual: number_of_clues,
            });
        }

        const MAX_ATTEMPTS: usize = 100;
        for _ in 0..MAX_ATTEMPTS {
            let mut board = Board {
                size,
                cells: vec![0; total],
                fixed_cells: Vec::new(),
            };
            let mut rng = rand::rng();
            let mut clues_added = 0;
            let mut attempts = 0;
            const CLUE_ATTEMPTS_PER_CLUE: usize = 1000;

            while clues_added < number_of_clues
                && attempts
                    < number_of_clues
                        .saturating_mul(CLUE_ATTEMPTS_PER_CLUE)
                        .max(1000)
            {
                attempts += 1;
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
            if clues_added == number_of_clues && board.check_if_possible() {
                return Ok(board);
            }
        }
        Err(BoardError::GenerationFailed)
    }

    pub fn import_from_file(file_path: &str) -> Result<Board, BoardError> {
        let contents = fs::read_to_string(file_path)
            .map_err(|error| BoardError::Io(format!("{file_path}: {error}")))?;
        let lines: Vec<&str> = contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        if lines.is_empty() {
            return Err(BoardError::MalformedInput(
                "missing Sudoku data".to_string(),
            ));
        }

        let first_tokens: Vec<&str> = lines[0].split_whitespace().collect();
        let header = if first_tokens.len() == 1 {
            first_tokens[0].parse::<usize>().ok()
        } else {
            None
        };

        let (size, value_lines) = if let Some(size) = header {
            square_root(size).ok_or(BoardError::InvalidSize(size))?;
            (size, &lines[1..])
        } else {
            let width = first_tokens.len();
            square_root(width).ok_or(BoardError::InvalidSize(width))?;
            (width, lines.as_slice())
        };

        if value_lines.len() != size {
            return Err(BoardError::InvalidCellCount {
                expected: size * size,
                actual: value_lines.len().saturating_mul(size),
            });
        }

        let mut cells = Vec::with_capacity(size * size);
        for (row, line) in value_lines.iter().enumerate() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() != size {
                return Err(BoardError::InvalidCellCount {
                    expected: size * size,
                    actual: cells.len() + tokens.len(),
                });
            }
            for (col, token) in tokens.iter().enumerate() {
                let value: usize = token.parse().map_err(|_| {
                    BoardError::MalformedInput(format!(
                        "value at row {row}, column {col} is not an integer"
                    ))
                })?;
                if value > size {
                    return Err(BoardError::InvalidValue {
                        index: cells.len(),
                        value,
                        size,
                    });
                }
                cells.push(value);
            }
        }
        Board::new(size, cells)
    }

    pub fn is_valid(&self) -> bool {
        if square_root(self.size).is_none() {
            return false;
        }
        if self.cells.len() != self.size * self.size {
            return false;
        }
        if self.cells.iter().any(|&value| value > self.size) {
            return false;
        }

        for row in 0..self.size {
            let mut seen = vec![false; self.size + 1];
            for col in 0..self.size {
                let value = self.cells[self.index(col, row)];
                if value != 0 {
                    if seen[value] {
                        return false;
                    }
                    seen[value] = true;
                }
            }
        }
        for col in 0..self.size {
            let mut seen = vec![false; self.size + 1];
            for row in 0..self.size {
                let value = self.cells[self.index(col, row)];
                if value != 0 {
                    if seen[value] {
                        return false;
                    }
                    seen[value] = true;
                }
            }
        }
        let box_size = square_root(self.size).unwrap();
        for box_row in 0..box_size {
            for box_col in 0..box_size {
                let mut seen = vec![false; self.size + 1];
                for y in 0..box_size {
                    for x in 0..box_size {
                        let value =
                            self.cells[self.index(box_col * box_size + x, box_row * box_size + y)];
                        if value != 0 {
                            if seen[value] {
                                return false;
                            }
                            seen[value] = true;
                        }
                    }
                }
            }
        }
        true
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    fn check_if_possible(&self) -> bool {
        let mut board = self.clone();
        board.solve_brute_force()
    }

    fn solve_brute_force(&mut self) -> bool {
        if let Some(idx) = self.mrv_empty_cell() {
            let candidates = self.candidate_values(idx);
            for value in candidates {
                self.cells[idx] = value;
                if self.solve_brute_force() {
                    return true;
                }
                self.cells[idx] = 0;
            }
            false
        } else {
            self.is_valid()
        }
    }

    fn solve_all_recursive(&mut self, solutions: &mut Vec<Vec<usize>>) {
        if let Some(idx) = self.mrv_empty_cell() {
            for value in self.candidate_values(idx) {
                self.cells[idx] = value;
                self.solve_all_recursive(solutions);
                self.cells[idx] = 0;
            }
        } else if self.is_valid() {
            solutions.push(self.cells.clone());
        }
    }

    fn mrv_empty_cell(&self) -> Option<usize> {
        let mut best: Option<(usize, usize)> = None;
        for (idx, &value) in self.cells.iter().enumerate() {
            if value == 0 {
                let count = self.candidate_values(idx).len();
                if count == 0 {
                    return Some(idx);
                }
                if best.is_none_or(|(_, best_count)| count < best_count) {
                    best = Some((idx, count));
                }
            }
        }
        best.map(|(idx, _)| idx)
    }

    fn candidate_values(&self, idx: usize) -> Vec<usize> {
        let row = idx / self.size;
        let col = idx % self.size;
        let box_size = square_root(self.size).unwrap_or(0);
        let box_row = row / box_size;
        let box_col = col / box_size;
        let mut used = vec![false; self.size + 1];
        for x in 0..self.size {
            used[self.cells[self.index(x, row)]] = true;
        }
        for y in 0..self.size {
            used[self.cells[self.index(col, y)]] = true;
        }
        for y in 0..box_size {
            for x in 0..box_size {
                used[self.cells[self.index(box_col * box_size + x, box_row * box_size + y)]] = true;
            }
        }
        (1..=self.size).filter(|&value| !used[value]).collect()
    }

    fn check_if_valid(&self, idx: usize, value: usize) -> bool {
        idx < self.cells.len() && value <= self.size && self.candidate_values(idx).contains(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solved_4x4() -> Vec<usize> {
        vec![1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 1]
    }

    fn valid_solution(board: &Board) -> bool {
        board.is_valid() && !board.cells.contains(&0)
    }

    #[test]
    fn rejects_non_square_and_wrong_cell_count() {
        assert!(Board::new(6, vec![0; 36]).is_err());
        assert!(matches!(
            Board::new(4, vec![0; 15]),
            Err(BoardError::InvalidCellCount { .. })
        ));
    }

    #[test]
    fn accepts_valid_board_and_rejects_duplicates() {
        let solved = Board::new(4, solved_4x4()).unwrap();
        assert!(valid_solution(&solved));
        let mut cells = solved_4x4();
        cells[1] = 1;
        assert!(matches!(
            Board::new(4, cells),
            Err(BoardError::InvalidBoard)
        ));
        assert!(!Board::new(4, vec![0; 16]).unwrap().cells.contains(&5));
    }

    #[test]
    fn solves_4x4_and_finds_all_solutions() {
        let mut board =
            Board::new(4, vec![1, 0, 0, 4, 0, 4, 1, 0, 0, 1, 0, 0, 4, 0, 2, 0]).unwrap();
        let solutions = board.get_all_solutions();
        assert!(!solutions.is_empty());
        assert!(solutions.iter().all(|cells| {
            let candidate = Board {
                size: 4,
                cells: cells.clone(),
                fixed_cells: vec![],
            };
            valid_solution(&candidate)
        }));

        let mut solved = board.clone();
        assert!(solved.solve_brute_force());
        assert!(valid_solution(&solved));
    }

    #[test]
    fn rejects_unsolvable_and_duplicate_initial_value() {
        let mut cells = solved_4x4();
        cells[0] = 0;
        cells[5] = 1;
        let board = Board::new(4, cells).unwrap_or_else(|_| {
            let mut fallback = vec![0; 16];
            fallback[0] = 1;
            fallback[5] = 1;
            Board {
                size: 4,
                cells: fallback,
                fixed_cells: vec![],
            }
        });
        assert!(!board.is_valid() || !board.check_if_possible());
    }

    #[test]
    fn parsing_requires_header_exact_row_count_and_range() {
        use std::fs;
        let dir = std::env::temp_dir();
        let path = dir.join(format!("sudorust-test-{}-valid.txt", std::process::id()));
        fs::write(&path, "4\n1 0 0 4\n0 4 1 0\n0 1 0 0\n4 0 2 0\n").unwrap();
        let board = Board::import_from_file(path.to_str().unwrap()).unwrap();
        assert_eq!(board.size, 4);
        assert_eq!(board.cells.len(), 16);
        fs::remove_file(path).unwrap();

        let malformed = dir.join(format!(
            "sudorust-test-{}-malformed.txt",
            std::process::id()
        ));
        fs::write(&malformed, "4\n1 0 0\n").unwrap();
        assert!(Board::import_from_file(malformed.to_str().unwrap()).is_err());

        fs::write(&malformed, "4\n1 0 0 4\n0 4 1 0\n0 1 0 0\n4 0 2 5\n").unwrap();
        assert!(matches!(
            Board::import_from_file(malformed.to_str().unwrap()),
            Err(BoardError::InvalidValue { .. })
        ));
        fs::remove_file(&malformed).unwrap();

        fs::write(&malformed, "6\n".repeat(1)).unwrap();
        assert!(matches!(
            Board::import_from_file(malformed.to_str().unwrap()),
            Err(BoardError::InvalidSize(6))
        ));
        fs::remove_file(malformed).unwrap();
    }

    #[test]
    fn parser_handles_bundled_multi_digit_puzzles() {
        let board = Board::import_from_file("data/single4x4.txt").unwrap();
        assert_eq!(board.size, 4);
        assert_eq!(board.cells.len(), 16);
        let board16 = Board::import_from_file("data/single16x16.txt").unwrap_or_else(|_| {
            let contents = std::fs::read_to_string("data/unsolved_16x16.txt").unwrap();
            let lines: Vec<&str> = contents.lines().take(17).collect();
            let temp = std::env::temp_dir()
                .join(format!("sudorust-test-{}-single16.txt", std::process::id()));
            std::fs::write(&temp, lines.join("\n")).unwrap();
            let board = Board::import_from_file(temp.to_str().unwrap()).unwrap();
            std::fs::remove_file(temp).unwrap();
            board
        });
        assert_eq!(board16.size, 16);
        assert_eq!(board16.cells.len(), 256);
    }

    #[test]
    fn parallel_solver_matches_serial_solver() {
        let board = Board::new(4, vec![1, 0, 0, 4, 0, 4, 1, 0, 0, 1, 0, 0, 4, 0, 2, 0]).unwrap();
        let mut serial_board = board.clone();
        let serial = serial_board.get_all_solutions();
        let parallel = board.get_all_solutions_parallel();
        assert_eq!(serial.len(), parallel.len());
    }

    #[test]
    fn bounded_generation_returns_valid_board_for_small_size() {
        let board = Board::generate_new_sudoku(4, 8).unwrap();
        assert_eq!(board.fixed_cells.len(), 8);
        assert!(board.is_valid());
        assert!(board.check_if_possible());
    }

    #[test]
    fn generation_rejects_invalid_arguments() {
        assert!(matches!(
            Board::generate_new_sudoku(6, 1),
            Err(BoardError::InvalidSize(6))
        ));
        assert!(matches!(
            Board::generate_new_sudoku(4, 17),
            Err(BoardError::InvalidCellCount { .. })
        ));
    }
}
