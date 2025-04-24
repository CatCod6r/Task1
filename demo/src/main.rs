use lib::{infinite_fibonacci::fibonacci_sequence, timout_iter::timeout_iterator};
fn main() {
    //timeout_iterator() function executes next step of iteration after set period of time
    let iterarray = vec![1, 2, 3];
    let iter = iterarray.iter();
    timeout_iterator(iter, 15);

    //fibonacci_sequence() function generates fibonacci number indefinetly(rust doesnt approwe
    //of this )
    fibonacci_sequence(0, 1);
}
