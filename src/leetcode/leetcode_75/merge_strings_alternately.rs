use crate::leetcode::Solution;
// You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
//
// Return the merged string.
//
//
//
// Example 1:
//
// Input: word1 = "abc", word2 = "pqr"
// Output: "apbqcr"
// Explanation: The merged string will be merged as so:
// word1:  a   b   c
// word2:    p   q   r
// merged: a p b q c r
// Example 2:
//
// Input: word1 = "ab", word2 = "pqrs"
// Output: "apbqrs"
// Explanation: Notice that as word2 is longer, "rs" is appended to the end.
// word1:  a   b
// word2:    p   q   r   s
// merged: a p b q   r   s
// Example 3:
//
// Input: word1 = "abcd", word2 = "pq"
// Output: "apbqcd"
// Explanation: Notice that as word1 is longer, "cd" is appended to the end.
// word1:  a   b   c   d
// word2:    p   q
// merged: a p b q c   d
//
//
// Constraints:
//
// 1 <= word1.length, word2.length <= 100
// word1 and word2 consist of lowercase English letters.

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut w1 = word1.chars();
        let mut w2 = word2.chars();
        let max = word1.len().max(word2.len());
        for _i in 0..max {
            if let Some(w1i) = w1.next() {
                result.push(w1i);
            }
            if let Some(w2i) = w2.next() {
                result.push(w2i);
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use crate::leetcode::Solution;

    #[test]
    fn test_merge_alternately() {
        let input = vec![
            ("abc".to_string(), "pqr".to_string()),
            ("ab".to_string(), "pqrs".to_string()),
            ("abcd".to_string(), "pq".to_string()),
        ];
        let want = vec![
            "apbqcr".to_string(),
            "apbqrs".to_string(),
            "apbqcd".to_string(),
        ];
        let mut got = Vec::new();
        for data in input {
            got.push(Solution::merge_alternately(data.0, data.1));
        }
        assert_eq!(want, got);
    }
}
