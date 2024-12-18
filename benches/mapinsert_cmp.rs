use criterion::{black_box, criterion_group, criterion_main, Criterion};
use indexmap::IndexMap;
use std::collections::{BTreeMap, HashMap};

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn hashmap_insert(entries: &[(String, usize)], alloc: bool) -> HashMap<String, usize> {
    let mut result = if alloc {
        HashMap::with_capacity(entries.len())
    } else {
        HashMap::new()
    };
    for (k, v) in entries {
        result.insert(k.clone(), *v);
    }
    result
}

fn hashmap_from(entries: &[(String, usize)]) -> HashMap<String, usize> {
    HashMap::from_iter(entries.iter().cloned())
}

fn indexmap_insert(entries: &[(String, usize)], alloc: bool) -> IndexMap<String, usize> {
    let mut result = if alloc {
        IndexMap::with_capacity(entries.len())
    } else {
        IndexMap::new()
    };
    for (k, v) in entries {
        result.insert(k.clone(), *v);
    }
    result
}

fn indexmap_from(entries: &[(String, usize)]) -> IndexMap<String, usize> {
    IndexMap::from_iter(entries.iter().cloned())
}

fn btreemap_insert(entries: &[(String, usize)]) -> BTreeMap<String, usize> {
    let mut result = BTreeMap::new();
    for (k, v) in entries {
        result.insert(k.clone(), *v);
    }
    result
}

fn btreemap_from(entries: &[(String, usize)]) -> BTreeMap<String, usize> {
    BTreeMap::from_iter(entries.iter().cloned())
}

fn criterion_benchmark(c: &mut Criterion) {
    let vec: Vec<(String, usize)> = (0..1000).map(|v| (format!("key_{v}"), v)).collect();
    for size in [1, 5, 25, 125, vec.len()] {
        for alloc in ["", " with prealloc"] {
            c.bench_function(&format!("HashMap insert {size}{alloc}"), |b| {
                b.iter(|| {
                    black_box(hashmap_insert(&vec[0..size], !alloc.is_empty()));
                });
            });

            c.bench_function(&format!("IndexMap insert {size}{alloc}"), |b| {
                b.iter(|| {
                    black_box(indexmap_insert(&vec[0..size], !alloc.is_empty()));
                });
            });
        }

        c.bench_function(&format!("BTreeMap insert {size}"), |b| {
            b.iter(|| {
                black_box(btreemap_insert(&vec[0..size]));
            });
        });

        c.bench_function(&format!("HashMap from {size}"), |b| {
            b.iter(|| {
                black_box(hashmap_from(&vec[0..size]));
            });
        });

        c.bench_function(&format!("IndexMap from {size}"), |b| {
            b.iter(|| {
                black_box(indexmap_from(&vec[0..size]));
            });
        });
        c.bench_function(&format!("BTreeMap from {size}"), |b| {
            b.iter(|| {
                black_box(btreemap_from(&vec[0..size]));
            });
        });
    }
}
