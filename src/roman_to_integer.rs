struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut previous = 0;

        let roman_chars: Vec<char> = s.chars().collect();

        for &ch in roman_chars.iter().rev() {
            let value = match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0
            };
            if value < previous {
                result -= value
            } else {
                result += value
            }

            previous = value;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58)
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994)
    }
}
