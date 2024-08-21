pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }
}

fn main() {
    Solution::contains_duplicate(vec![1, 2, 3, 1]);
}

