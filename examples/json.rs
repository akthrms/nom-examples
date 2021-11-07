use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1},
    combinator::recognize,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Number(f64),
    String(String),
    Boolean(bool),
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
}

fn parse_null(input: &str) -> IResult<&str, Json> {
    let (input, _) = tag("null")(input)?;
    Ok((input, Json::Null))
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    let (input, value) = recognize(tuple((digit1, char('.'), digit1)))(input)?;
    Ok((input, value.parse().unwrap()))
}

fn parse_integer(input: &str) -> IResult<&str, f64> {
    let (input, value) = digit1(input)?;
    Ok((input, value.parse().unwrap()))
}

fn parse_number(input: &str) -> IResult<&str, Json> {
    let (input, value) = alt((parse_float, parse_integer))(input)?;
    Ok((input, Json::Number(value)))
}

fn parse_string(input: &str) -> IResult<&str, Json> {
    let (input, value) = delimited(char('"'), alphanumeric1, char('"'))(input)?;
    Ok((input, Json::String(value.to_string())))
}

fn parse_boolean(input: &str) -> IResult<&str, Json> {
    let (input, value) = alt((tag("true"), tag("false")))(input)?;

    let value = match value {
        "true" => Json::Boolean(true),
        "false" => Json::Boolean(false),
        _ => unreachable!(),
    };

    Ok((input, value))
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        match parse_null("null") {
            Ok((_, json)) => assert_eq!(json, Json::Null),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_number() {
        match parse_number("1") {
            Ok((_, json)) => assert_eq!(json, Json::Number(1.0)),
            _ => unreachable!(),
        }

        match parse_number("1.0") {
            Ok((_, json)) => assert_eq!(json, Json::Number(1.0)),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_string() {
        match parse_string(r#""abc""#) {
            Ok((_, json)) => assert_eq!(json, Json::String("abc".to_string())),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_boolean() {
        match parse_boolean("true") {
            Ok((_, json)) => assert_eq!(json, Json::Boolean(true)),
            _ => unreachable!(),
        }

        match parse_boolean("false") {
            Ok((_, json)) => assert_eq!(json, Json::Boolean(false)),
            _ => unreachable!(),
        }
    }
}
