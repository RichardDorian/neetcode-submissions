use std::collections::HashMap;

fn letters(s: &String) -> [u8; 26] {
    let mut count = [0u8; 26];

    for char in s.bytes() {
        count[(char - b'a') as usize] += 1;
    }

    return count;
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for str in strs {
            let key = letters(&str);
            map.entry(key).or_default().push(str);
        }

        map.into_values().collect()
    }
}
