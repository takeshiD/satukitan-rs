use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{take_while, take_while_m_n};
use nom::combinator::recognize;
use nom::sequence::pair;

pub fn inline_space0(input: &str) -> IResult<&str, &str> {
    take_while(is_inline_space)(input)
}

pub fn inline_space1(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, usize::MAX, is_inline_space)(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while_m_n(1, 1, is_identifier_start),
        take_while(is_identifier_continue),
    ))
    .parse(input)
}

fn is_inline_space(c: char) -> bool {
    matches!(c, ' ' | '\t')
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || matches!(c, '_' | '#')
}

fn is_identifier_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '-')
}
