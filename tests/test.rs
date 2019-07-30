extern crate sbench;

#[test]
fn bench_test() {
    let t = sbench::bench(|| {
        let a = 1;
        let b = 2;
        let _ = a + b;
    });

    assert!(t.as_nanos() > 0);
}

#[test]
fn bench_n_test() {
    let t = sbench::bench_n(10, || {
        let a = 1;
        let b = 2;
        let _ = a + b;
    });

    assert!(t.as_nanos() > 0);
}