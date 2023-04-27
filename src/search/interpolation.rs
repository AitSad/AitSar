fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    
    while left <= right && target >= arr[left] && target <= arr[right] {
        let pos = left + ((target - arr[left]) * (right - left) as i32) / (arr[right] - arr[left]);
        
        if arr[pos] == target {
            return Some(pos);
        } else if arr[pos] < target {
            left = pos + 1;
        } else {
            right = pos - 1;
        }
    }
    
    None
}
