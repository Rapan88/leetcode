struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut res: i32 = -1;
        if let Some(ind) = haystack.find(&needle) {
            res = ind as i32;
        } else {
            return -1
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn ex1() {
        assert_eq!(Solution::str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }

    fn ex2() {
        assert_eq!(Solution::str_str("leetcode".to_string(), "leetee".to_string()), -1);
    }
}