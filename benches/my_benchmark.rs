use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn hashmap_benchmark(c: &mut Criterion) {
    let mut map: hashmap::HashMap<String, String> = hashmap::HashMap::new();
    c.bench_function("hashmap insert 1000", |b| b.iter(|| 
        for i in 0..black_box(1000) {
            map.insert(i.to_string(), i.to_string());
        }
    ));
}

fn rust_hashmap_benchmark(c: &mut Criterion) {
    let mut map: HashMap<String, String> = HashMap::new();

    c.bench_function("rust hashmap insert 1000", |b| b.iter(|| 
        for i in 0..black_box(1000) {
            map.insert(i.to_string(), i.to_string());
        }
    ));
}

criterion_group!(benches, hashmap_benchmark, rust_hashmap_benchmark);
criterion_main!(benches);