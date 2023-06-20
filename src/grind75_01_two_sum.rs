pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![]
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn test_1() {
                assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
        }
}
