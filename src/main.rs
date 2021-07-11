fn main() {
}

type Block = [u8; 9];

fn check_block(block: Block) -> bool {
    let mut bits = [false; 10]; // Could also do -1 but I figure this may be faster... 
    for &elem in &block {
        //let bit = &mut bits[elem as usize];
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
        assert!(check_block([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert!(!check_block([1, 2, 3, 4, 5, 6, 7, 8, 10]));
        assert!(!check_block([2, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert!(check_block([2, 1, 3, 4, 5, 6, 7, 8, 9]));
        assert!(check_block([4, 1, 3, 2, 5, 6, 7, 8, 9]));
        assert!(check_block([4, 1, 3, 2, 5, 6, 9, 8, 7]));
        assert!(check_block([4, 1, 3, 2, 6, 5, 8, 9, 7]));
        assert!(check_block([4, 8, 3, 2, 6, 5, 1, 9, 7]));
    }
}
