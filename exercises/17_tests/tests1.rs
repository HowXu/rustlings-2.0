// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::is_even; // super指向上层mod 最底层mod是crate 这层之外就是crate

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        // assert直接接收bool值
        assert!(is_even(2));
        assert!(is_even(4));
        
    }
}
