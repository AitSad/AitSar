fn fibonacci_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut fib_m2 = 0;
    let mut fib_m1 = 1;
    let mut fib = fib_m2 + fib_m1;

    while fib < arr.len() {
        fib_m2 = fib_m1;
        fib_m1 = fib;
        fib = fib_m2 + fib_m1;
    }

    let mut offset = -1;
    while fib > 1 {
        let i = std::cmp::min(offset + fib_m2 as i32, (arr.len() - 1) as i32) as usize;
        if arr[i] < target {
            fib = fib_m1;
            fib_m1 = fib_m2;
            fib_m2 = fib - fib_m1;
            offset = i as i32;
        } else if arr[i] > target {
            fib = fib_m2;
            fib_m1 = fib_m1 - fib_m2;
            fib_m2 = fib - fib_m1;
        } else {
            return Some(i);
        }
    }

    if fib_m1 == 1 && arr[(offset + 1) as usize] == target {
        Some(offset as usize + 1)
    } else {
        None
    }
}
