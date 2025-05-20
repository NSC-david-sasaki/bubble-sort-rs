// This file provides utility functions that may assist in generating test cases, such as creating random arrays or arrays with specific properties for testing.

use rand::Rng;

/// Generates a random array of integers with the specified length and range.
pub fn generate_random_array(length: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range(min..=max)).collect()
}

/// Creates an array that is already sorted in ascending order.
pub fn create_sorted_array(length: usize) -> Vec<i32> {
    (0..length as i32).collect()
}

/// Creates an array that is sorted in descending order.
pub fn create_reverse_sorted_array(length: usize) -> Vec<i32> {
    (0..length as i32).rev().collect()
}

/// Creates an array where all elements are identical.
pub fn create_identical_array(length: usize, value: i32) -> Vec<i32> {
    vec![value; length]
}

/// Creates an empty array.
pub fn create_empty_array() -> Vec<i32> {
    Vec::new()
}

/// Creates an array with a single element.
pub fn create_single_element_array(value: i32) -> Vec<i32> {
    vec![value]
}