
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for char in s.chars() {
            if char == '(' || char == '[' || char == '{' {
                stack.push(char);
            }

            if char == ')' {
                if stack.ends_with(&['(']) {
                    stack.pop();
                } else {
                    return false;
                }
            }

            if char == ']' {
                if stack.ends_with(&['[']) {
                    stack.pop();
                } else {
                    return false;
                }
            }

            if char == '}' {
                if stack.ends_with(&['{']) {
                    stack.pop();
                } else {
                    return false;
                }
            }
        }

        stack.len() == 0
    }
}