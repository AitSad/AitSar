fn ternary_search<F>(mut left: f64, mut right: f64, eps: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    while right - left > eps {
        let m1 = left + (right - left) / 3.0;
        let m2 = right - (right - left) / 3.0;
        if f(m1) < f(m2) {
            left = m1;
        } else {
            right = m2;
        }
    }
    (left + right) / 2.0
}
