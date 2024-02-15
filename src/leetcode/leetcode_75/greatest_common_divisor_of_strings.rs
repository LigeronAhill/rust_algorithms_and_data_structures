use crate::leetcode::Solution;
// For two strings s and t, we say "t divides s" if and only if s = t + ... + t (i.e., t is concatenated with itself one or more times).
//
// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.
//
//
//
// Example 1:
//
// Input: str1 = "ABCABC", str2 = "ABC"
// Output: "ABC"
// Example 2:
//
// Input: str1 = "ABABAB", str2 = "ABAB"
// Output: "AB"
// Example 3:
//
// Input: str1 = "LEET", str2 = "CODE"
// Output: ""
//
//
// Constraints:
//
// 1 <= str1.length, str2.length <= 1000
// str1 and str2 consist of English uppercase letters.
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 == str2.clone() + &str1 {
            let gcd_len = gcd(str1.len(), str2.len());
            str1[..gcd_len].to_string()
        } else {
            "".to_string()
        }
    }
}
fn gcd(mut a: usize, mut b: usize) -> usize {
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
    use crate::leetcode::Solution;

    #[test]
    fn test_gcd_of_strings() {
        let input = vec![
            (String::from("ABCABC"), String::from("ABC")),
            (String::from("ABABAB"), String::from("ABAB")),
            (String::from("LEET"), String::from("CODE")),
            (String::from("ABCDEF"), String::from("ABC")),
            (String::from("ABABCCABAB"), String::from("ABAB")),
        ];
        let want = vec![
            String::from("ABC"),
            String::from("AB"),
            String::from(""),
            String::from(""),
            String::from(""),
        ];
        let mut got = Vec::new();
        for data in input {
            got.push(Solution::gcd_of_strings(data.0, data.1));
        }
        assert_eq!(want, got);
    }
}
