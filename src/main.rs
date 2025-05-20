mod sorter;
use rand::seq::SliceRandom;
use sorter::optimized_bubble_sort;
use std::time::Instant;

fn main() {
    // Generate same data as Python tests
    let mut rng = rand::rng();
    let mut data: Vec<i32> = (0..1000).collect();
    data.shuffle(&mut rng);
    
    // Time the sort
    let start = Instant::now();
    let mut sorted_data = data.clone();
    optimized_bubble_sort(&mut sorted_data);
    let duration = start.elapsed();
    
    println!("Rust bubble sort (n=1000): {:.1?}", duration);
}