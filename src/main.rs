use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;

fn main() {
    let mut puzzle = Puzzle { data: KNOWN_SOLVED };
    println!("{}", puzzle);
    println!();

    puzzle.data[2] = 0;
    puzzle.data[8] = 0;

    println!("{}", puzzle);
    println!();
    let mask = make_mask(&puzzle);
    let p = random_fill(puzzle, mask.clone());
    println!("{}", p);

    //let soln = evaluative_solver();
    //println!("{}", soln);
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

    const ONE_PART_EMPTY: BoardData = [
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

    #[test]
    fn test_check_puzzle() {
        // True puzzle
        let puzzle = Puzzle { data: KNOWN_SOLVED };
        assert!(check_puzzle(&puzzle, true));
        assert!(check_puzzle(&puzzle, false));

        // One part empty
        let puzzle = Puzzle {
            data: ONE_PART_EMPTY,
        };
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

    #[test]
    fn test_eval_block() {
        assert_eq!(eval_block([3, 4, 5, 2, 8, 6, 1, 7, 9]), 0);
        assert_eq!(eval_block([3, 4, 5, 2, 8, 6, 1, 7, 7]), 2);
        assert_eq!(eval_block([3, 4, 5, 2, 8, 6, 7, 7, 7]), 4);
        assert_eq!(eval_block([3, 4, 5, 2, 8, 7, 7, 7, 7]), 6);
        assert_eq!(eval_block([3, 4, 5, 2, 8, 7, 7, 7, 0]), 6);
        assert_eq!(eval_block([0; 9]), 9 * 3 - 1);
    }

    #[test]
    fn test_eval_puzzle() {
        assert_eq!(eval_puzzle(&Puzzle { data: KNOWN_SOLVED }), 0);
        assert_eq!(
            eval_puzzle(&Puzzle {
                data: ONE_PART_EMPTY
            }),
            2 * 3
        );
        assert_eq!(eval_puzzle(&Puzzle::empty()), (9 * 3 - 1) * 3 * (3 * 3));
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

fn eval_block(block: Block) -> usize {
    let mut cost = 0;
    let mut seen = [false; 10]; // Could also do -1 but I figure this may be faster...
    for &elem in &block {
        // Penalty for empty spaces
        if elem == 0 {
            cost += 1;
        }

        // Penalty for double seen numbers
        if seen[elem as usize] {
            cost += 1;
        } else {
            seen[elem as usize] = true;
        }
    }

    // Penalty for unseen numbers
    cost += seen[1..].iter().filter(|b| !*b).count();

    cost
}

fn random_puzzle() -> Puzzle {
    let mut rng = rand::thread_rng();
    let mut data = [0u8; 9 * 9];
    data.chunks_exact_mut(9)
        .enumerate()
        .for_each(|(n, chunk)| chunk.fill(n as u8 + 1));
    //println!("{}", Puzzle { data });
    data.shuffle(&mut rng);
    Puzzle { data }
}

// TODO: Not very DRY of you...
fn eval_puzzle(puzzle: &Puzzle) -> usize {
    let mut cost = 0;

    // Check rows:
    for row in puzzle.data.chunks_exact(9) {
        let mut block = [0u8; 9];
        block.copy_from_slice(row);
        cost += eval_block(block);
    }

    // Check columns
    for i in 0..9 {
        let mut block = [0u8; 9];
        for (&puzz, block) in puzzle.data.iter().skip(i).step_by(9).zip(&mut block) {
            *block = puzz;
        }
        cost += eval_block(block);
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
            cost += eval_block(block);
        }
    }

    cost
}

fn evaluative_solver() -> Puzzle {
    let mut puzzle = random_puzzle();
    let mut n = 0;
    let mut rng = rand::thread_rng();
    let mut best_cost = eval_puzzle(&puzzle);

    const REPEATS: u32 = 10_000;
    let mut repeat_countdown = REPEATS;

    let mut lowest_ever = usize::MAX;

    // TODO: Fruitless restarts?
    while !check_puzzle(&puzzle, true) {
        let mut proposal = puzzle;
        let first_idx = rng.gen_range(0..9 * 9);
        let second_idx = rng.gen_range(0..9 * 9);
        proposal.data.swap(first_idx, second_idx);
        let proposal_cost = eval_puzzle(&proposal);
        if proposal_cost < best_cost {
            best_cost = proposal_cost;
            puzzle = proposal;
            repeat_countdown = REPEATS;
            lowest_ever = lowest_ever.min(best_cost);
        } else {
            repeat_countdown -= 1;
        }

        n += 1;
        if n % 100_000 == 0 {
            dbg!(n, best_cost, repeat_countdown, lowest_ever);
            //std::thread::sleep_ms(1);
        }

        if repeat_countdown == 0 {
            puzzle = random_puzzle();
            best_cost = eval_puzzle(&puzzle);
        }
        //if n % 1000 == 0 {
        //dbg!(n, best_cost);
        //}
    }

    puzzle
}

/// Sparse mask
type Mask = Vec<usize>;

fn make_mask(puzzle: &Puzzle) -> Mask {
    puzzle
        .data
        .iter()
        .enumerate()
        .filter_map(|(i, &e)| (e == EMPTY).then(|| i))
        .collect()
}

/// Fill zeroes with the correct amount of random numbers 1-9
fn random_fill(mut puzzle: Puzzle, mut mask: Mask) -> Puzzle {
    let mut buckets = vec![(0u8, 0u8); 9];
    buckets.iter_mut().enumerate().for_each(|(i, b)| *b = ((i+1) as u8, 9));
    for &elem in &puzzle.data {
        if elem != EMPTY {
            buckets[(elem - 1) as usize].1 -= 1;
        }
    }

    let mut rng = rand::thread_rng();
    mask.shuffle(&mut rng);

    for i in mask {
        buckets.retain(|b| b.1 != 0);
        let bucket = buckets.choose_mut(&mut rng).expect("Should be unreachable for well-formed puzzles...");
        puzzle.data[i] = bucket.0;
        bucket.1 -= 1;
    }

    puzzle
}
