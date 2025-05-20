// sorter.rs

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

/*pub fn optimized_bubble_sort(arr: &mut Vec<i32>) {
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
    */
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