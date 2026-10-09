use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Get the frequency of each number in the nums vector
        let mut frequency: HashMap<i32, u32> = HashMap::new();

        for num in &nums {
            *frequency.entry(*num).or_insert(0) += 1;
        }

        // We do a bucket sort in the count, if the nums vec has n elements,
        // the frequency of a single element is at most n so we have n buckets
        // In each buckets we store the elements that have bucket's frequency.
        // At the end we traverse (starting at the end) the buckets to get k elements.

        let mut buckets: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

        for (num, frequency) in frequency {
            buckets[frequency as usize].push(num);
        }

        let mut k_frequent: Vec<i32> = vec![];

        for bucket in (0..=nums.len()).rev() {
            for num in buckets[bucket].iter() {
                k_frequent.push(*num);
            }

            if k_frequent.len() >= k as usize {
                break;
            }
        }

        k_frequent
    }
}
