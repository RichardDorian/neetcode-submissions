impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = nums.into_iter().collect();

        let mut max = 0;

        for n in &nums {
            // If the previous number exists, then it is not the begining of a sequence
            if nums.contains(&(n - 1)) {
                continue;
            }

            let mut i = 0;

            loop {
                if nums.contains(&(n + i)) {
                    i += 1;
                } else {
                    break;
                }
            }

            if i > max {
                max = i
            }
        }

        max
    }
}