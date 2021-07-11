Objective:
Create a matrix with all 9 numbers in each row and column and and 3x3
iterative; place a random number in a random spot, then check if it's a valid solution

* Fast check (narrows down): check if each row and column sums to (1..=9).sum();
* pattern check: boolean array that must be all;

```rust
type Block = [u8; 9];
fn check_block(block: Block) -> bool {
    let mut bits = [false; 10]; // Could also do -1 but I figure this may be faster... 
    for elem in block.iter().zip(&mut bits) {
        let bit = &mut bits[elem as usize];
        if bit {
            return false;
        } else {
            bit = true;
        }
    }
}

#[cfg(test)]
mod tests {
    fn test_check_block() {
        assert!(check_block([1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }
}
```
