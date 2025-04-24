use std::fmt::Debug;
use std::time::{Duration, Instant};

pub fn timeout_iterator<I>(mut iter: I, timeout_s: u64)
where
    I: Iterator,
    <I as Iterator>::Item: Debug,
{
    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_s);
    while start.elapsed() < timeout {
        if let Some(item) = iter.next() {
            println!("{:?}", item);
        } else {
            break;
        }
    }

    println!("Timeout reached");
}
