// Since the length of an element is never longer than 200 we can prefix with a 3 characters long
// number (padded with zeroes) all elements. When decoding the decoder will know how long the
// element is.

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        strs
            .iter()
            .map(|s| format!("{:0>3}{s}", s.len().to_string()))
            .collect()
    }

    pub fn decode(mut s: String) -> Vec<String> {
        let mut strs = vec![];

        loop {
            if s.len() <= 0 {
                break;
            }

            let length = s[..3].parse::<usize>().expect("Malformed payload");
            strs.push(s[3..(length + 3)].to_string());
            s = s[length + 3..s.len()].to_string();
        }

        strs
    }
}
