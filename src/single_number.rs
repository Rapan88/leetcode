struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result ^= num;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4)
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::single_number(vec![1]), 1)
    }
}