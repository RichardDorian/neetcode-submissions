use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        for num in nums.iter() {
            if (map.get(num).is_some()) {
                return true;
            }

            map.insert(num, true);
        }

        return false;
    }
}
