pub fn is_palindrom(s: String) -> bool {
    let lower_cased = s.to_lowercase();

    if lower_cased.chars().rev().collect::<String>() == lower_cased {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn true_when_palindrom() {
        assert_eq!(is_palindrom(String::from("ana")), true);
    }

    #[test]
    fn false_when_not_palindrom() {
        assert_eq!(is_palindrom(String::from("anan")), false);
    }

    #[test]
    fn true_when_palindrom_with_even_characters() {
        assert_eq!(is_palindrom(String::from("anna")), true);
    }

    #[test]
    fn true_when_empty_string() {
        assert_eq!(is_palindrom(String::from("")), true);
    }

    #[test]
    fn true_when_upper_and_lower_case_string() {
        assert_eq!(is_palindrom(String::from("iTopiNonAvevanoNipoti")), true);
    }

    #[test]
    fn false_when_upper_and_lower_case_string() {
        assert_eq!(is_palindrom(String::from("NunayIVergrubench!acho")), false);
    }
}
