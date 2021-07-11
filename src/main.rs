fn main() {
}

const EMPTY: u8 = 0; // TODO: Use NonZeroU8?

struct Puzzle {
    data: [u8; 9*9],
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
            for dx in x..x+3 {
                for dy in y..y+3 {
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

    fn test_check_puzzle() {
        let data = [
            5, 3, 4, 6, 7, 8, 9, 1, 2,
            6, 7, 2, 1, 9, 5, 3, 4, 8,
            1, 9, 8, 3, 4, 2, 5, 6, 7,
            8, 5, 9, 7, 6, 1, 4, 2, 3,
            4, 2, 6, 8, 5, 3, 7, 9, 1,
            7, 1, 3, 9, 2, 4, 8, 5, 6,
            9, 6, 1, 5, 3, 7, 2, 8, 4,
            2, 8, 7, 4, 1, 9, 6, 3, 5,
            3, 4, 5, 2, 8, 6, 1, 7, 9,
        ];
        let puzzle = Puzzle { data };
    }
}
