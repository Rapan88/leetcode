struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered: String = s
            .to_lowercase()
            .chars()
            .filter(|e| e.is_alphanumeric()).collect();
        filtered.chars().rev().collect::<String>().eq(&filtered)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false)
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::is_palindrome(" ".to_string()), true)
    }
}