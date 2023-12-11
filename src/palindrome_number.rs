struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        x.to_string().chars().rev().collect::<String>() == x.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_palindrome(121), true)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_palindrome(122), false)
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::is_palindrome(-121), false)
    }
}