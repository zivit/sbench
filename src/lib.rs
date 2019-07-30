//! sbench is a simple benchmark for Rust. 
//! It allows you to measure the performance of functions.

use std::time::{Duration, Instant};

/// Measures the performance of a function by running it 1000 times.
/// 
/// # Example
/// ```rust
/// let t = sbench::bench(|| {
///     let a = 1;
///     let b = 2;
///     let _ = a + b;
/// });
/// 
/// assert!(t.as_nanos() > 0);
/// ```
pub fn bench<T>(f: T) -> Duration
    where T: Fn()
{
    bench_n(1000, f)
}

/// Measures the performance of the function, running it as many times as indicated in the parameter `times`.
/// 
/// # Example
/// ```rust
/// let t = sbench::bench_n(1_000_000, || {
///     let a = 1;
///     let b = 2;
///     let _ = a + b;
/// });
/// 
/// assert!(t.as_nanos() > 0);
/// ```
pub fn bench_n<T>(times: usize, f: T) -> Duration
    where T: Fn()
{
    let mut measures = Vec::with_capacity(times);
    let mut start;

    for _ in 0..times {
        start = Instant::now();
        f();
        measures.push(start.elapsed());
    }

    measures.iter().min().unwrap().clone()
}
