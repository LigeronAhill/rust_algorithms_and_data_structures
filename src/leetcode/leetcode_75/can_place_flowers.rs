use crate::leetcode::Solution;

// 605. Can Place Flowers
// Easy
// Topics
// Companies
// You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
//
// Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.
//
//
//
// Example 1:
//
// Input: flowerbed = [1,0,0,0,1], n = 1
// Output: true
// Example 2:
//
// Input: flowerbed = [1,0,0,0,1], n = 2
// Output: false
//
//
// Constraints:
//
// 1 <= flowerbed.length <= 2 * 104
// flowerbed[i] is 0 or 1.
// There are no two adjacent flowers in flowerbed.
// 0 <= n <= flowerbed.length
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let (mut capability, mut zero_count) = (0, 1);
        for f in flowerbed {
            if f == 1 {
                capability += (zero_count - 1) / 2;
                zero_count = 0;
            } else {
                zero_count += 1;
            }
        }
        capability += zero_count / 2;
        n <= capability
    }
}
#[cfg(test)]
mod tests {
    use crate::leetcode::Solution;

    #[test]
    fn test_can_place_flowers() {
        let input = vec![
            (vec![1, 0, 0, 0, 1], 1),
            (vec![1, 0, 0, 0, 1], 2),
            (vec![0, 1, 0], 1),
            (vec![0, 0, 1, 0, 1], 1),
            (vec![1, 0, 0, 0, 0, 1], 2),
            (vec![0, 1, 0, 1, 0, 1, 0, 0], 1),
            (vec![1, 0, 1, 0, 0, 1, 0], 1),
        ];
        let want = vec![true, false, false, true, false, true, false];
        let mut got = Vec::new();
        for item in input {
            got.push(Solution::can_place_flowers(item.0, item.1));
        }
        assert_eq!(got, want);
    }
}
