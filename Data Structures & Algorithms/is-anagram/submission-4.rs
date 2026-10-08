impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        // We are sure the letters are lowercase so only 26 possibilities
        let mut count = [0u8; 26];

        for char in s.bytes() {
            count[(char - 97) as usize] += 1;
        }

        for char in t.bytes() {
            count[(char - 97) as usize] -= 1;
        }

        return count.iter().all(|v| *v == 0);
    }
}
