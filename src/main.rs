fn main() {
    println!("Hello, string_adder!");
}

pub type Values = i32;

pub fn add(data: &str) -> Values {
    if data.is_empty() {
        return 0
    }
    itemize(data).sum()
}

fn itemize<'a>(data: &'a str) -> Box<Iterator<Item=Values> + 'a> {
    Box::new(data.split(',').map(parse_token))
}

fn parse_token(token: &str) -> Values {
    token.parse().unwrap()
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

    #[test]
    fn should_sum_comma_separated_integers() {
        assert_eq!(3, add("1,2"));
        assert_eq!(6, add("1,2,3"));
    }

}