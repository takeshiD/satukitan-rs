use crate::ast::{Expr, Program};
use crate::error::{SatukitanError, map_nom_error};
use crate::lexer;
use crate::value::{bool_token, parse_number_token};

use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::{all_consuming, cut};
use nom::error::{Error, ErrorKind};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded};

type Res<'a, T> = nom::IResult<&'a str, T>;

pub fn parse_program(source: &str) -> Result<Program, SatukitanError> {
    let cleaned = strip_comments(source);
    let mut parser = all_consuming(program);
    match parser.parse(cleaned.as_str()) {
        Ok((_, exprs)) => Ok(exprs),
        Err(err) => Err(map_nom_error(&cleaned, err)),
    }
}

pub fn parse_single_expr(source: &str) -> Result<Expr, SatukitanError> {
    let cleaned = strip_comments(source);
    let mut parser = all_consuming(preceded(multispace0, parse_expr));
    match parser.parse(cleaned.as_str()) {
        Ok((_, expr)) => Ok(expr),
        Err(err) => Err(map_nom_error(&cleaned, err)),
    }
}

fn program(input: &str) -> Res<'_, Vec<Expr>> {
    let (input, _) = multispace0(input)?;
    many0(parse_expr_with_ws).parse(input)
}

fn parse_expr_with_ws(input: &str) -> Res<'_, Expr> {
    let (input, expr) = parse_expr(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, expr))
}

fn parse_expr(input: &str) -> Res<'_, Expr> {
    alt((
        parse_list_literal,
        parse_paren_list,
        parse_string,
        parse_number,
        parse_boolean,
        parse_call_or_symbol,
    ))
    .parse(input)
}

fn parse_list_literal(input: &str) -> Res<'_, Expr> {
    delimited(
        char('['),
        separated_list0(multispace1, preceded(multispace0, parse_argument)),
        preceded(multispace0, cut(char(']'))),
    )
    .map(Expr::ListLiteral)
    .parse(input)
}

fn parse_paren_list(input: &str) -> Res<'_, Expr> {
    delimited(
        char('('),
        separated_list0(multispace1, preceded(multispace0, parse_argument)),
        preceded(multispace0, cut(char(')'))),
    )
    .map(Expr::List)
    .parse(input)
}

fn parse_string(input: &str) -> Res<'_, Expr> {
    if !input.starts_with('"') {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Char)));
    }

    let mut escaped = false;
    for (idx, ch) in input[1..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => {
                let content = &input[1..1 + idx];
                let rest_index = 1 + idx + 1;
                let rest = &input[rest_index..];
                if let Some(string) = unescape_string(content) {
                    return Ok((rest, Expr::String(string)));
                } else {
                    return Err(nom::Err::Error(Error::new(content, ErrorKind::Escaped)));
                }
            }
            _ => {}
        }
    }

    Err(nom::Err::Error(Error::new(input, ErrorKind::Char)))
}

fn unescape_string(src: &str) -> Option<String> {
    let mut result = String::with_capacity(src.len());
    let mut chars = src.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                _ => return None,
            }
        } else {
            result.push(ch);
        }
    }
    Some(result)
}

fn parse_number(input: &str) -> Res<'_, Expr> {
    let (rest, ident) = lexer::identifier(input)?;
    if let Some(value) = parse_number_token(ident) {
        Ok((rest, Expr::Number(value)))
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}

fn parse_boolean(input: &str) -> Res<'_, Expr> {
    let (rest, ident) = lexer::identifier(input)?;
    if let Some(value) = bool_token(ident) {
        Ok((rest, Expr::Bool(value)))
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}

fn parse_call_or_symbol(input: &str) -> Res<'_, Expr> {
    let (mut rest, head) = lexer::identifier(input)?;
    let symbol = head.to_string();
    let mut args = Vec::new();

    while let Ok((next, _)) = lexer::inline_space1(rest) {
        let (next_rest, arg) = parse_argument(next)?;
        args.push(arg);
        rest = next_rest;
    }

    if args.is_empty() {
        Ok((rest, Expr::Symbol(symbol)))
    } else {
        Ok((
            rest,
            Expr::Call {
                func: Box::new(Expr::Symbol(symbol)),
                args,
            },
        ))
    }
}

fn parse_argument(input: &str) -> Res<'_, Expr> {
    alt((
        parse_list_literal,
        parse_paren_list,
        parse_string,
        parse_number,
        parse_boolean,
        parse_symbol,
    ))
    .parse(input)
}

fn parse_symbol(input: &str) -> Res<'_, Expr> {
    let (rest, ident) = lexer::identifier(input)?;
    Ok((rest, Expr::Symbol(ident.to_string())))
}

fn strip_comments(source: &str) -> String {
    source
        .lines()
        .map(strip_comment_from_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_comment_from_line(line: &str) -> String {
    let mut chars = line.char_indices().peekable();
    while let Some((idx, ch)) = chars.next() {
        if ch == '#' {
            let prev_is_ws =
                idx == 0 || line[..idx].chars().last().is_none_or(|c| c.is_whitespace());
            let next_is_ws = chars.peek().map(|(_, c)| c.is_whitespace()).unwrap_or(true);
            if prev_is_ws && next_is_ws {
                return line[..idx].trim_end().to_string();
            }
        }
    }
    line.trim_end().to_string()
}
