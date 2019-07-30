# sbench
Simple benchmark for Rust

## Examples
```rust
let t = sbench::bench(|| {
    let a = 1;
    let b = 2;
    let _ = a + b;
});

assert!(t.as_nanos() > 0);
```
```rust
let t = sbench::bench_n(10, || {
    let a = 1;
    let b = 2;
    let _ = a + b;
});

assert!(t.as_nanos() > 0);
```