use std::time::{Duration, Instant};

pub fn bench<T>(f: T) -> Duration
    where T: Fn()
{
    bench_n(1000, f)
}

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
