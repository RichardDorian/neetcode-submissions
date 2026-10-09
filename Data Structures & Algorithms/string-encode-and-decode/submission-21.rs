impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let res = strs
            .iter()
            .map(|s| format!("{:0>3}{s}", s.len().to_string()))
            .collect();

        // println!("{res}");

        res
    }

    pub fn decode(mut s: String) -> Vec<String> {
        let mut strs = vec![];

        loop {
            // println!("Starting with {s}");
            if s.len() <= 0 {
                break;
            }

            let length = s[..3].parse::<usize>().expect("Malformed payload");
            let word = s[3..(length + 3)].to_string();
            strs.push(word.clone());
            // println!("s: {s} l:{length} w:{word}");

            s = s[length + 3..s.len()].to_string();
        }

        strs
    }
}
