fn exponential_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut bound = 1;
    while bound < arr.len() && arr[bound] < target {
        bound *= 2;
    }
    let left = bound / 2;
    let right = std::cmp::min(bound, arr.len() - 1);

    arr[left..=right].binary_search(&target).ok().map(|i| left + i)
}
