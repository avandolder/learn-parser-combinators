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

fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    chars.take_while(|c| c.is_alphanumeric() || *c == '-')
         .for_each(|c| matched.push(c));

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
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

    #[test]
    fn identifer_parser() {
        use crate::identifier;
        assert_eq!(
            Ok(("", "i-am-an-identifier".to_string())),
            identifier("i-am-an-identifier")
        );
        assert_eq!(
            Ok((" entirely an identifier", "not".to_string())),
            identifier("not entirely an identifier")
        );
        assert_eq!(
            Err("!not at all an identifier"),
            identifier("!not at all an identifier")
        );
    }
}
