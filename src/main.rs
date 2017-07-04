fn main() {
    println!("Hello, string_adder!");
}

pub fn add(_data: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_should_return_0() {
        assert_eq!(0, add(""));
    }

}