use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();

        for num in nums.iter() {
            if (map.get(num).is_some()) {
                return true;
            }

            map.insert(num);
        }

        return false;
    }
}
