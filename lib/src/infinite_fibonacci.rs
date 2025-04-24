pub fn fibonacci_sequence(last: u32, current: u32) {
    fibonacci_sequence(current, last + current);
}
