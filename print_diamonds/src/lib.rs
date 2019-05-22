pub fn print_diamonds(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }

    let mut str_list = vec![];
    let mut d: i32 = 1;
    let half_run = n / 2 as i32;

    for i in 0..n {
        let line = " ".repeat(((n - d) / 2) as usize) + &"*".repeat(d as usize) + &"\n".to_string();

        if i < half_run {
            d += 2;
        } else {
            d -= 2;
        }
        str_list.push(line);
    }

    Some(str_list.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_one() {
        assert_eq!(print_diamonds(1), Some("*\n".to_string()));
    }

    #[test]
    fn test_with_two() {
        assert_eq!(print_diamonds(2), None);
    }

    #[test]
    fn test_with_three() {
        assert_eq!(print_diamonds(3), Some(" *\n***\n *\n".to_string()));
    }
}
