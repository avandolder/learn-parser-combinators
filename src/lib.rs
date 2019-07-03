// Following https://bodil.lol/parser-combinators/

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attr: HashMap<String, String>,
    children: Vec<Element>,
}

fn match_literal(expected: &'static str)
    -> impl Fn(&str) -> Result<(&str, ()), &str>
{
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

fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2)
    -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
    where
        P1: Fn(&str) -> Result<(&str, R1), &str>,
        P2: Fn(&str) -> Result<(&str, R2), &str>,
{
    move |input| match parser1(input) {
        Ok((next_input, result1)) => match parser2(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}


fn map<P, F, A, B>(parser: P, map_fn: F)
    -> impl Fn(&str) -> Result<(&str, B), &str>
    where
        P: Fn(&str) -> Result<(&str, A), &str>,
        F: Fn(A) -> B,
{
    move |input| parser(input).map(|(next, result)| (next, map_fn(result)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn literal_parser() {
        use crate::match_literal;
        let parse_joe = match_literal("Hello Joe!");
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

    #[test]
    fn pair_combinator() {
        use crate::{identifier, match_literal, pair};
        let tag_opener = pair(match_literal("<"), identifier);
        assert_eq!(
            Ok(("/>", ((), "my-first-element".to_string()))),
            tag_opener("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener("oops"));
        assert_eq!(Err("!oops"), tag_opener("<!oops"));
    }

    #[test]
    fn map_combinator() {
        use crate::{identifier, map, match_literal};
        let to_upper = map(identifier, |s| s.to_uppercase());
        assert_eq!(
            Ok((" bob", "ALICE".to_string())),
            to_upper("alice bob")
        );
        assert_eq!(Err("!alice"), to_upper("!alice"));
    }
}
