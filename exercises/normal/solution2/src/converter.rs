pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    // a1(12) => 121(10)
    let mut result = String::new();
    let num = num_str.to_lowercase();
    let res : Vec<&str>= num.split('(').collect();
    let origin_num = res[0];
    let origin_base = res[1].trim_end_matches(')').parse::<u32>().unwrap();
    let decimal = to_decimal(origin_num, origin_base);
    
    if decimal == 0 {
        return "0".to_string();
    }
    
    let digits = "0123456789abcdef";
    let mut value = decimal;
    
    while value > 0 {
        result.push(digits.chars().nth((value % to_base) as usize).unwrap());
        value /= to_base;
    }
    
    result = result.chars().rev().collect::<String>();
    result
    
}
fn to_decimal(num_str: &str, base: u32) -> u32 {
    let mut decimal_value = 0;
    for (i, digit) in num_str.chars().rev().enumerate() {
        let value = match digit {
            '0'..='9' => digit.to_digit(10).unwrap(),
            'a'..='f' => digit.to_digit(16).unwrap(),
            'A'..='F' => digit.to_digit(16).unwrap(),
            _ => panic!("Invalid character {} in input", digit),
        };
        decimal_value += value * base.pow(i as u32);
    }
    decimal_value
}