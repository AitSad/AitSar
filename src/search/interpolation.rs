fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = arr.len() - 1;

    while lo <= hi && target >= arr[lo] && target <= arr[hi] {
        let pos = lo + ((target - arr[lo]) * (hi - lo) as i32 / (arr[hi] - arr[lo])) as usize;

        if arr[pos] == target {
            return Some(pos);
        }

        if arr[pos] < target {
            lo = pos + 1;
        } else {
            hi = pos - 1;
        }
    }

    None
}
