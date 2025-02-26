pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let denominations = [100, 50, 30, 20, 10, 5, 2, 1];  // 币种面额
    let mut dp = vec![u32::MAX; (amount + 1) as usize]; // 初始化 dp 数组，设置最大值
    dp[0] = 0;  // 金额为0时需要0张纸币

    for i in 1..=amount {
        for &denom in &denominations {
            if i >= denom {
                dp[i as usize] = dp[i as usize].min(dp[(i - denom) as usize] + 1);
            }
        }
    }

    dp[amount as usize]
}
