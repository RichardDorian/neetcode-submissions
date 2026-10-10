impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = nums.iter().cloned().collect();

        println!("{:?}", nums);

        // List of possible numbers that starts a sequence
        let mut candidates = vec![];

        for n in &nums {
            // If the previous number isn't in the list then it's the begining of a sequence
            if !nums.contains(&(n - 1)) {
                candidates.push(n);
            }
        }

        let mut longest = 0;

        for candidate in candidates {
            let mut i = 0;

            loop {
                if nums.contains(&(candidate + i)) {
                    i += 1;
                } else {
                    break;
                }
            }

            if i > longest {
                longest = i
            }
        }

        longest
    }
}