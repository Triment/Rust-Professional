pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let mut p = 1_f64;
    for i in 0..n {
        p *= (365_f64 - (i as f64)) / 365.0;
    }
    1_f64 - p
}
