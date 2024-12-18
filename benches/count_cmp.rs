use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn plain_count(vec: &[usize]) -> usize {
    let mut count = 0;
    for v in vec {
        if v % 2 == 1 {
            count += 1;
        }
    }
    count
}

fn filter_count(vec: &[usize]) -> usize {
    vec.iter().filter(|&v| v % 2 == 1).count()
}

fn criterion_benchmark(c: &mut Criterion) {
    let vec: Vec<usize> = (0..1000).collect();
    c.bench_function("Using for loop", |b| {
        b.iter(|| {
            black_box(plain_count(&vec));
        });
    });

    c.bench_function("Using iter filter", |b| {
        b.iter(|| {
            black_box(filter_count(&vec));
        });
    });
}
