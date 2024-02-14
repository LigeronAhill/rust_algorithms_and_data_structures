pub fn karatsuba(x: u64, y: u64) -> u64 {
    if x < 10 || y < 10 {
        x * y
    } else {
        let n = (x.to_string().len() as f64 / 2.0).ceil() as usize;
        let (a, b) = (x / 10_u64.pow(n as u32), x % 10_u64.pow(n as u32));
        let (c, d) = (y / 10_u64.pow(n as u32), y % 10_u64.pow(n as u32));
        let ac = karatsuba(a, c);
        let bd = karatsuba(b, d);
        let pq = karatsuba(a + b, c + d);
        let adbc = pq - ac - bd;
        10_u64.pow(2 * n as u32) * ac + 10_u64.pow(n as u32) * adbc + bd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_recint() {
        let x = 5678;
        let y = 1234;
        let got = karatsuba(x, y);
        let want = 7006652;
        assert_eq!(got, want);
    }
}
