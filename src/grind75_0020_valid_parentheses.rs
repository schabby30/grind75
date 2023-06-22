pub fn is_valid(s: String) -> bool {
    let mut p = "".to_string();

    for ch in s.chars() {
        match ch {
            '(' => p.push(')'),
            '[' => p.push(']'),
            '{' => p.push('}'),
            ')' | ']' | '}' => {
                if let Some(c) = p.pop() {
                    if c != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    p.len() == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(is_valid("(][)".to_string()), false);
    }
}
