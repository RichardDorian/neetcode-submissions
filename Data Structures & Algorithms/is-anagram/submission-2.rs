use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut characters = HashMap::new();

        for char in s.chars() {
            if let Some(count) = characters.insert(char, 1) {
                characters.insert(char, count + 1);
            }
        }

        let mut characters2 = HashMap::new();

        for char in t.chars() {
            if let Some(count) = characters2.insert(char, 1) {
                characters2.insert(char, count + 1);
            }
        }

        return characters == characters2;
    }
}
