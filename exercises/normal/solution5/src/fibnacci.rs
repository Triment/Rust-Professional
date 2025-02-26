pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut i = 0_u32;
    let mut j = 1_u32;
    let mut sum_of_odd = 0_u32;
    if threshold == 0 {
        return 0;
    }
    if threshold == 1 {
        return 1;
    }

    while j < threshold {
        if j % 2 == 1 {
            sum_of_odd += j;
        }
        let tmp = j;
        j = i + j;
        i = tmp;
    }
    sum_of_odd
}
