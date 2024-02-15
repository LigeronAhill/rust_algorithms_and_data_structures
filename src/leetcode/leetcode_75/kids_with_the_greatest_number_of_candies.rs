use crate::leetcode::Solution;

// 1431. Kids With the Greatest Number of Candies
// Easy
// Topics
// Companies
// Hint
// There are n kids with candies. You are given an integer array candies, where each candies[i] represents the number of candies the ith kid has, and an integer extraCandies, denoting the number of extra candies that you have.
//
// Return a boolean array result of length n, where result[i] is true if, after giving the ith kid all the extraCandies, they will have the greatest number of candies among all the kids, or false otherwise.
//
// Note that multiple kids can have the greatest number of candies.
//
//
//
// Example 1:
//
// Input: candies = [2,3,5,1,3], extraCandies = 3
// Output: [true,true,true,false,true]
// Explanation: If you give all extraCandies to:
// - Kid 1, they will have 2 + 3 = 5 candies, which is the greatest among the kids.
// - Kid 2, they will have 3 + 3 = 6 candies, which is the greatest among the kids.
// - Kid 3, they will have 5 + 3 = 8 candies, which is the greatest among the kids.
// - Kid 4, they will have 1 + 3 = 4 candies, which is not the greatest among the kids.
// - Kid 5, they will have 3 + 3 = 6 candies, which is the greatest among the kids.
// Example 2:
//
// Input: candies = [4,2,1,1,2], extraCandies = 1
// Output: [true,false,false,false,false]
// Explanation: There is only 1 extra candy.
// Kid 1 will always have the greatest number of candies, even if a different kid is given the extra candy.
// Example 3:
//
// Input: candies = [12,1,12], extraCandies = 10
// Output: [true,false,true]
//
//
// Constraints:
//
// n == candies.length
// 2 <= n <= 100
// 1 <= candies[i] <= 100
// 1 <= extraCandies <= 50
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result = Vec::new();
        let max = *candies.iter().max().unwrap_or(&0);
        for candy in candies {
            result.push(candy + extra_candies >= max);
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use crate::leetcode::Solution;
    #[test]
    fn test_kids_with_candies() {
        let input = vec![
            (vec![2, 3, 5, 1, 3], 3),
            (vec![4, 2, 1, 1, 2], 1),
            (vec![12, 1, 12], 10),
        ];
        let want = vec![
            vec![true, true, true, false, true],
            vec![true, false, false, false, false],
            vec![true, false, true],
        ];
        let mut got = Vec::new();
        for item in input {
            got.push(Solution::kids_with_candies(item.0, item.1));
        }
        assert_eq!(want, got);
    }
}
