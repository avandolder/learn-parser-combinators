// Following https://bodil.lol/parser-combinators/

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attr: HashMap<String, String>,
    children: Vec<Element>,
}

fn match_literal(expected: &'static str)
        -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], ()))
        },
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn literal_parser() {
        let parse_joe = super::match_literal("Hello Joe!");
        assert_eq!(
            Ok(("", ())),
            parse_joe("Hello Joe!")
        );
        assert_eq!(
            Ok((" Hello Robert!", ())),
            parse_joe("Hello Joe! Hello Robert!")
        );
        assert_eq!(
            Err("Hello Mike!"),
            parse_joe("Hello Mike!")
        );
    }
}
