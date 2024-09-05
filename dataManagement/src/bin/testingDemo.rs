fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {

}

#[cfg(test)] // cfg == configuration
mod test {
    use crate::*;
    #[test]
    fn check_all_caps() {
        let result = all_caps("za world");
        let expected = String::from("ZA WoRLD");
        assert_eq!(result, expected, "string should be all uppercase");
    }
}