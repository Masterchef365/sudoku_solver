use rand::seq::SliceRandom;
use std::fmt;

fn main() {
    let zero = Puzzle::empty();
    let soln = dumb_solver(zero);
    if let Some(soln) = soln {
        println!("{}", soln);
    } else {
        unreachable!("Inconceivable!!")
    }
}

const EMPTY: u8 = 0; // TODO: Use NonZeroU8?

type BoardData = [u8; 9 * 9];

#[derive(Copy, Clone, Debug)]
struct Puzzle {
    data: BoardData,
}

impl Puzzle {
    fn empty() -> Self {
        Self { data: [0u8; 9 * 9] }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.chunks_exact(9) {
            for elem in row {
                write!(f, "{} ", elem)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn check_puzzle(puzzle: &Puzzle, test_zero: bool) -> bool {
    // Check rows:
    for row in puzzle.data.chunks_exact(9) {
        let mut block = [0u8; 9];
        block.copy_from_slice(row);
        if !check_block(block, test_zero) {
            return false;
        }
    }

    // Check columns
    for i in 0..9 {
        let mut block = [0u8; 9];
        for (&puzz, block) in puzzle.data.iter().skip(i).step_by(9).zip(&mut block) {
            *block = puzz;
        }
        if !check_block(block, test_zero) {
            return false;
        }
    }

    // Check 3x3s
    for x in (0..9).step_by(3) {
        for y in (0..9).step_by(3) {
            let mut block = [0u8; 9];
            let mut i = 0;
            for dx in x..x + 3 {
                for dy in y..y + 3 {
                    block[i] = puzzle.data[(dx + dy * 9) as usize];
                    i += 1;
                }
            }
            if !check_block(block, test_zero) {
                return false;
            }
        }
    }

    true
}

type Block = [u8; 9];

fn check_block(block: Block, test_zero: bool) -> bool {
    let mut bits = [false; 10]; // Could also do -1 but I figure this may be faster...
    for &elem in &block {
        if elem == 0 {
            if test_zero {
                return false;
            } else {
                continue;
            }
        }

        let bit = match bits.get_mut(elem as usize) {
            Some(b) => b,
            None => return false,
        };

        if *bit {
            return false;
        } else {
            *bit = true;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_block() {
        assert!(check_block([1, 2, 3, 4, 5, 6, 7, 8, 9], true));
        assert!(!check_block([1, 2, 3, 4, 5, 6, 7, 8, 10], true));
        assert!(!check_block([2, 2, 3, 4, 5, 6, 7, 8, 9], true));
        assert!(check_block([2, 1, 3, 4, 5, 6, 7, 8, 9], true));
        assert!(check_block([4, 1, 3, 2, 5, 6, 7, 8, 9], true));
        assert!(check_block([4, 1, 3, 2, 5, 6, 9, 8, 7], true));
        assert!(check_block([4, 1, 3, 2, 6, 5, 8, 9, 7], true));
        assert!(check_block([4, 8, 3, 2, 6, 5, 1, 9, 7], true));

        assert!(!check_block([1, 2, 3, 4, 5, 6, 7, 8, 0], true));
        assert!(check_block([1, 2, 3, 4, 5, 6, 7, 8, 0], false));
    }

    #[test]
    fn test_check_puzzle() {
        // True puzzle
        let puzzle = Puzzle { data: KNOWN_SOLVED };
        assert!(check_puzzle(&puzzle, true));
        assert!(check_puzzle(&puzzle, false));

        // One part empty
        let data = [
            5, 3, 4, 6, 7, 8, 9, 1, 2, //
            6, 7, 2, 1, 9, 5, 3, 4, 8, //
            1, 9, 8, 3, 4, 2, 5, 6, 7, //
            8, 5, 9, 7, 6, 1, 4, 2, 3, //
            4, 2, 6, 8, 5, 0, 7, 9, 1, //
            7, 1, 3, 9, 2, 4, 8, 5, 6, //
            9, 6, 1, 5, 3, 7, 2, 8, 4, //
            2, 8, 7, 4, 1, 9, 6, 3, 5, //
            3, 4, 5, 2, 8, 6, 1, 7, 9, //
        ];
        let puzzle = Puzzle { data };
        assert!(!check_puzzle(&puzzle, true));
        assert!(check_puzzle(&puzzle, false));

        // Transposed two elements
        let data = [
            5, 7, 4, 6, 7, 8, 9, 1, 2, //
            6, 3, 2, 1, 9, 5, 3, 4, 8, //
            1, 9, 8, 3, 4, 2, 5, 6, 7, //
            8, 5, 9, 7, 6, 1, 4, 2, 3, //
            4, 2, 6, 8, 5, 3, 7, 9, 1, //
            7, 1, 3, 9, 2, 4, 8, 5, 6, //
            9, 6, 1, 5, 3, 7, 2, 8, 4, //
            2, 8, 7, 4, 1, 9, 6, 3, 5, //
            3, 4, 5, 2, 8, 6, 1, 7, 9, //
        ];
        let puzzle = Puzzle { data };
        assert!(!check_puzzle(&puzzle, true));
        assert!(!check_puzzle(&puzzle, false));
    }
}

const KNOWN_SOLVED: BoardData = [
    5, 3, 4, 6, 7, 8, 9, 1, 2, //
    6, 7, 2, 1, 9, 5, 3, 4, 8, //
    1, 9, 8, 3, 4, 2, 5, 6, 7, //
    8, 5, 9, 7, 6, 1, 4, 2, 3, //
    4, 2, 6, 8, 5, 3, 7, 9, 1, //
    7, 1, 3, 9, 2, 4, 8, 5, 6, //
    9, 6, 1, 5, 3, 7, 2, 8, 4, //
    2, 8, 7, 4, 1, 9, 6, 3, 5, //
    3, 4, 5, 2, 8, 6, 1, 7, 9, //
];

/*
struct AvailableOptions {
}

fn check_available_options(puzzle: Puzzle)
*/

fn dumb_solver(mut work: Puzzle) -> Option<Puzzle> {
    let mut rng = rand::thread_rng();
    let mut open_spaces = work
        .data
        .iter()
        .enumerate()
        .filter_map(|(idx, elem)| (*elem == EMPTY).then(|| idx))
        .collect::<Vec<usize>>();
    open_spaces.shuffle(&mut rng);

    if open_spaces.is_empty() {
        if check_puzzle(&work, true) {
            return Some(work);
        } else {
            return None;
        }
    }

    let mut combs = [0u8; 9];
    combs.iter_mut().zip(1..=9).for_each(|(c, n)| *c = n);

    for space in open_spaces {
        combs.shuffle(&mut rng);
        for num in combs {
            work.data[space] = num;
            if !check_puzzle(&work, false) {
                continue;
            }
            if let Some(soln) = dumb_solver(work) {
                return Some(soln);
            }
        }
    }

    None
}
