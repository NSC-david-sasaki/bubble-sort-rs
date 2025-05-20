use criterion::{criterion_group, criterion_main, Criterion};
use bubble_sort_rs::sorter::{bubble_sort, optimized_bubble_sort};
use rand::seq::SliceRandom;
use rand::rng;
use std::hint::black_box;
use std::time::Duration;

fn benchmark_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");
    group
        .sample_size(10000) 
        .measurement_time(Duration::from_secs(6))
        .warm_up_time(Duration::from_secs(2));
    
    // Generate test data
    let mut rng = rng();
    let size = 1000;
    let mut data: Vec<i32> = (0..size).collect();
    data.shuffle(&mut rng);

    // Test both implementations with same data
    group.bench_function("Basic Bubble Sort", |b| {
        b.iter_with_setup(
            || data.clone(),
            |mut arr| {
                bubble_sort(black_box(&mut arr));
            }
        )
    });

    group.bench_function("Optimized Bubble Sort", |b| {
        b.iter_with_setup(
            || data.clone(),
            |mut arr| {
                optimized_bubble_sort(black_box(&mut arr));
            }
        )
    });

    group.finish();
}

criterion_group!(benches, benchmark_sorting);
criterion_main!(benches);