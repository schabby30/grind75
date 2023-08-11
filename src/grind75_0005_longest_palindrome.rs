pub fn longest_palindrome(s: String) -> String {
    let mut window_size = s.len();

    while window_size > 0 {
        match s.as_bytes().windows(window_size).find(|slice| {
            is_palindrome(std::str::from_utf8(*slice).expect("Invalid UTF-8"))
        }) {
            Some(result) => return std::str::from_utf8(result).unwrap_or("").to_string(),
            None => window_size -= 1
        }
    }

    return "".to_string();
}

fn is_palindrome(s: &str) -> bool {
    let length = s.len();
    if length < 2 || (s.as_bytes()[0].eq(&s.as_bytes()[length - 1]) && is_palindrome(&s[1..s.len()-1])) {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(longest_palindrome("".to_string()), "");
    }

    #[test]
    fn test_2() {
        assert_eq!(longest_palindrome("a".to_string()), "a");
    }

    #[test]
    fn test_3() {
        assert_eq!(longest_palindrome("aa".to_string()), "aa");
    }

    #[test]
    fn test_4() {
        assert_eq!(longest_palindrome("baa".to_string()), "aa");
    }

    #[test]
    fn test_5() {
        assert_eq!(longest_palindrome("bbaa".to_string()), "bb");
    }

    #[test]
    fn test_6() {
        assert_eq!(longest_palindrome("aaadsasdd".to_string()), "dsasd");
    }

}
