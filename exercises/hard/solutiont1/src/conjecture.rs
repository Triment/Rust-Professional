pub fn goldbach_conjecture() -> String {
    let mut res: Vec<u32> = Vec::new();
    
    'outer :for i in 16.. {
            if res.len() == 2  {
                break;
            }
            if i % 2 == 0 || i.is_prime() {
                continue;
            }
            for j in 1..i {
                let double = 2 * j * j;
                if double >= i {
                    break;
                }
                let prime = i - double;
                if !prime.is_prime() {
                    continue;
                } else {
                    continue 'outer;
                }
            }
            res.push(i);
    }
    format!("{},{}", res[0], res[1])
}

trait Prime {
    fn is_prime(&self) -> bool;
}

fn pow_mod(a: u32, b: u32, m: u32) -> u32 {//防止溢出
    let mut result = 1;
    let mut base = a % m;
    let mut exponent = b;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % m;
        }
        base = (base * base) % m;
        exponent >>= 1;
    }
    result
}

impl Prime for u32 {
    fn is_prime(&self) -> bool {
        let n = *self;
        if n <= 1 {
            return false;
        } else if n <= 3 {
            return true;
        } else if n % 2 == 0 {
            return false;
        }

        // 将 n-1 分解为 d * 2^s
        let mut d = (n - 1) / 2;
        let mut s = 1;
        while d % 2 == 0 {
            d /= 2;
            s += 1;
        }

        // 测试确定性基底（覆盖 u32 范围）
        let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

        for a in bases {
            if a >= n {
                continue; // 当 a >= n 时跳过（如 n=2,3）
            }
            let mut x = pow_mod(a, d, n);
            if x == 1 || x == n - 1 {
                continue;
            }
            for _ in 0..s-1 {
                x = pow_mod(x, 2, n);
                if x == n - 1 {
                    break;
                }
            }
            if x != n - 1 {
                return false;
            }
        }
        true
    }
}