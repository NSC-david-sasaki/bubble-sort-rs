# Implementation
During this assignment 3 implementations of bubble sort were compared.

## Basic Bubble Sort
```rust
pub fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for _ in 0..len {
        for j in 0..len - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
```

## First Optimization Attempt
This version tracks both swapped flag and last swapped position:
```rust
pub fn optimized_bubble_sort_v1(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        let mut last_swapped_index = 0;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
                last_swapped_index = j + 1;
            }
        }
        if !swapped {
            break; // Array is already sorted
        }
        // Reduce the range of the next pass
        if last_swapped_index < n - 1 - i {
            break;
        }
    }
}
```
## Final Optimized Version
This version simplifies the logic while maintaining performance benefits:
```rust
pub fn optimized_bubble_sort(arr: &mut Vec<i32>) {
    let mut n = arr.len();
    let mut new_n;
    
    loop {
        new_n = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_n = i;
            }
        }
        if new_n == 0 {
            break;
        }
        n = new_n;
    }
}
```

## Implementation Comparison

| Feature | Basic | First Optimization | Final Optimization |
|---------|-------|-------------------|-------------------|
| Early Exit | No | Yes | Yes |
| Swap Flag | No | Yes | No |
| Position Tracking | No | Yes | Yes |
| Loop Structure | Nested For | Nested For | Loop with For |
| Extra Variables | 0 | 2 | 1 |


## Performance Impact
The benchmarks show that while all implementations correctly sort the array, they have different performance characteristics:

1. Basic version: Consistent but slower performance
2. First optimization: Additional overhead from multiple flags
3. Final optimization: Best overall performance, especially for partially sorted arrays

To run the benchmarks yourself:
`cargo bench`
The results will be available in `target/criterion/index.html`.