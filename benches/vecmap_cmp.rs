use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn plain_push(vec: &[usize]) -> Vec<usize> {
    let mut d = Vec::new();
    for &n in vec {
        d.push(n * n);
    }
    d
}

fn capacity_push(vec: &[usize]) -> Vec<usize> {
    let mut d = Vec::with_capacity(vec.len());
    for &n in vec {
        d.push(n * n);
    }
    d
}

fn chain(vec: &[usize]) -> Vec<usize> {
    vec.iter().map(|&n| n * n).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let vec: Vec<usize> = (0..1000).collect();
    c.bench_function("Using vec push", |b| {
        b.iter(|| {
            black_box(plain_push(&vec));
        });
    });

    c.bench_function("Using prealloc vec push", |b| {
        b.iter(|| {
            black_box(capacity_push(&vec));
        });
    });

    c.bench_function("Using vec iter chain", |b| {
        b.iter(|| {
            black_box(chain(&vec));
        });
    });
}
