// Following https://bodil.lol/parser-combinators/

use std::collections::HashMap;

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attr: HashMap<String, String>,
    children: Vec<Element>,
}

fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> { 
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

fn identifier(input: &str) -> ParseResult<String> { 
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

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>, 
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2.parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B,
{
    move |input| parser.parse(input)
                       .map(|(next, result)| (next, map_fn(result)))
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

#[cfg(test)]
mod tests {
    #[test]
    fn literal_parser() {
        use crate::{Parser, match_literal};
        let parse_joe = match_literal("Hello Joe!");
        assert_eq!(
            Ok(("", ())),
            parse_joe.parse("Hello Joe!")
        );
        assert_eq!(
            Ok((" Hello Robert!", ())),
            parse_joe.parse("Hello Joe! Hello Robert!")
        );
        assert_eq!(
            Err("Hello Mike!"),
            parse_joe.parse("Hello Mike!")
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
        use crate::{Parser, identifier, match_literal, pair};
        let tag_opener = pair(match_literal("<"), identifier);
        assert_eq!(
            Ok(("/>", ((), "my-first-element".to_string()))),
            tag_opener.parse("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn map_combinator() {
        use crate::{Parser, identifier, map};
        let to_upper = map(identifier, |s| s.to_uppercase());
        assert_eq!(
            Ok((" bob", "ALICE".to_string())),
            to_upper.parse("alice bob")
        );
        assert_eq!(Err("!alice"), to_upper.parse("!alice"));
    }
}
