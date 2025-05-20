# Test Cases

This chapter details the test suite used to verify our bubble sort implementations. All tests can be run using:

```bash
cargo test
```

## Correctness Tests

### 1. Random Array Test
Tests sorting of unordered elements:

```rust
#[test]
fn test_random_array() {
    let mut arr = vec![5, 3, 8, 1, 2];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 5, 8]);
}
```

### 2. Already Sorted Array
Verifies behavior with pre-sorted data:

```rust
#[test]
fn test_already_sorted_array() {
    let mut arr = vec![1, 2, 3, 4, 5];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}
```

### 3. Descending Order Array
Tests worst-case scenario:

```rust
#[test]
fn test_descending_order_array() {
    let mut arr = vec![5, 4, 3, 2, 1];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}
```

### 4. Identical Elements Array
Verifies stability with duplicate values:

```rust
#[test]
fn test_identical_elements_array() {
    let mut arr = vec![2, 2, 2, 2, 2];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![2, 2, 2, 2, 2]);
}
```

### 5. Empty Array
Tests boundary conditions:

```rust
#[test]
fn test_empty_array() {
    let mut arr: Vec<i32> = vec![];
    bubble_sort(&mut arr);
    assert_eq!(arr, Vec::<i32>::new());
}
```

### 6. Single Element
Tests minimal input case:

```rust
#[test]
fn test_single_element_array() {
    let mut arr = vec![42];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![42]);
}
```