fn main() {
    println!("Hello, string_adder!");
}

pub type Values = i32;

pub fn add(data: &str) -> Values {
    if data.is_empty() {
        return 0
    }
    if data.starts_with("//["){
        let separator = data.chars().nth(3).unwrap();
        let itemizer = Itemize::new(separator);

        return itemizer.itemize(data.splitn(2,'\n').nth(1).unwrap()).sum()
    }
    Itemize::default().itemize(data).sum()
}

struct Itemize {
    separator: char
}

impl Default for Itemize{
    fn default() -> Self {
        Self::new(',')
    }
}

impl Itemize {
    pub fn new(separator: char) -> Self {
        Itemize {
            separator: separator
        }
    }

    pub fn itemize<'a>(self, data: &'a str) -> Box<Iterator<Item=Values> + 'a> {
        Box::new(data.split(move |c| self.is_separator(c)).map(Self::parse_token))
    }

    fn is_separator(&self, c: char) -> bool {
        c==self.separator || c=='\n'
    }

    fn parse_token(token: &str) -> Values {
        token.parse().unwrap()
    }
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

    #[test]
    fn also_carriage_return_is_a_separator() {
        assert_eq!(3, add("1\n2"));
        assert_eq!(25, add("1,2\n3,4,12\n1\n2"));
    }

    #[test]
    fn should_parse_header_to_seect_correct_separator() {
        assert_eq!(3, add("//[;]\n1;2"));
        assert_eq!(17, add("//[;]\n1;2\n12;2"));
    }
}