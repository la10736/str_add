fn main() {
    println!("Hello, string_adder!");
}

pub fn add(data: &str) -> i32 {
    if data.is_empty() {
        return 0
    }
    data.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_should_return_0() {
        assert_eq!(0, add(""));
    }

    #[test]
    fn should_return_parsed_integers() {
        assert_eq!(3, add("3"));
        assert_eq!(12, add("12"));
    }

}