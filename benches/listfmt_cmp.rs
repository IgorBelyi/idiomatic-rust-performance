use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn plain_string(vec: &[usize]) -> String {
    let mut res = String::new();
    let mut first = true;
    for v in vec {
        if !first {
            res.push(',');
        }
        first = false;
        res.push_str(&v.to_string());
    }
    res
}

fn prealloc_string(vec: &[usize]) -> String {
    let mut res = String::with_capacity(4000);
    let mut first = true;
    for v in vec {
        if !first {
            res.push(',');
        }
        first = false;
        res.push_str(&v.to_string());
    }
    res
}

fn join_string(vec: &[usize]) -> String {
    vec.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn format_string(vec: &[usize]) -> String {
    vec.iter()
        .format_with(",", |arg, f| f(&format_args!("{:?}", arg)))
        .to_string()
}

fn criterion_benchmark(c: &mut Criterion) {
    let vec: Vec<usize> = (0..1000).collect();
    c.bench_function("Plain push", |b| {
        b.iter(|| {
            black_box(plain_string(&vec));
        });
    });

    c.bench_function("Preallocated push", |b| {
        b.iter(|| {
            black_box(prealloc_string(&vec));
        });
    });

    c.bench_function("Join string vector", |b| {
        b.iter(|| {
            black_box(join_string(&vec));
        });
    });

    c.bench_function("Format iterator", |b| {
        b.iter(|| {
            black_box(format_string(&vec));
        });
    });
}
