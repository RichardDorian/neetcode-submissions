impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefixes = vec![1; nums.len()];

        for i in 1..nums.len() {
            prefixes[i] = prefixes[i - 1] * nums[i - 1];
        }

        let mut suffixes = vec![1; nums.len()];

        for i in (0..(nums.len() - 1)).rev() {
            suffixes[i] = suffixes[i + 1] * nums[i + 1];
        }

        let mut result = vec![0; nums.len()];

        for i in 0..nums.len() {
            result[i] = prefixes[i] * suffixes[i];
        }

        result
    }
}