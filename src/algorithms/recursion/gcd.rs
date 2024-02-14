pub fn greatestcd(mut a: usize, mut b: usize) -> usize {
    assert!(a != 0 && b != 0);
    while b != 0 {
        if b < a {
            (a, b) = (b, a);
        }
        b %= a;
    }
    a
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greatestcd() {
        assert_eq!(greatestcd(14, 15), 1);
        assert_eq!(greatestcd(64, 1296), 16);
    }
}
