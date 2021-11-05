//! <expr>   ::= <term> [ ('+'|'-') <term> ]*
//! <term>   ::= <factor> [ ('*'|'/') <factor> ]*
//! <factor> ::= <number> | '(' <expr> ')'
//! <number> :== 数値

use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn number(input: &str) -> IResult<&str, Expr> {
    let (input, n) = digit1(input)?;
    let expr = Expr::Num(n.parse().unwrap());
    Ok((input, expr))
}

fn factor(input: &str) -> IResult<&str, Expr> {
    let (input, expr) = alt((number, paren))(input)?;
    Ok((input, expr))
}

fn paren(input: &str) -> IResult<&str, Expr> {
    let (input, expr) = delimited(char('('), expr, char(')'))(input)?;
    Ok((input, expr))
}

fn term(input: &str) -> IResult<&str, Expr> {
    let mul_or_div = alt((char('*'), char('/')));
    let mul_or_div_calculates = many0(tuple((mul_or_div, factor)));
    let (input, (expr, exprs)) = tuple((factor, mul_or_div_calculates))(input)?;

    let expr = exprs.iter().fold(expr, |left, (op, right)| match op {
        '*' => Expr::Mul(Box::new(left), Box::new(right.clone())),
        '/' => Expr::Div(Box::new(left), Box::new(right.clone())),
        _ => left,
    });

    Ok((input, expr))
}

fn expr(input: &str) -> IResult<&str, Expr> {
    let add_or_sub = alt((char('+'), char('-')));
    let add_or_sub_calculates = many0(tuple((add_or_sub, term)));
    let (input, (expr, exprs)) = tuple((term, add_or_sub_calculates))(input)?;

    let expr = exprs.iter().fold(expr, |left, (op, right)| match op {
        '+' => Expr::Add(Box::new(left), Box::new(right.clone())),
        '-' => Expr::Sub(Box::new(left), Box::new(right.clone())),
        _ => left,
    });

    Ok((input, expr))
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(left, right) => eval(left) + eval(right),
        Expr::Sub(left, right) => eval(left) - eval(right),
        Expr::Mul(left, right) => eval(left) * eval(right),
        Expr::Div(left, right) => eval(left) / eval(right),
    }
}

fn main() {
    let result = expr("(1+2)-3*4/5");

    match result {
        Ok((_, ast)) => {
            println!("ast = {:?}", ast);
            println!("result = {}", eval(&ast));
        }
        Err(e) => println!("err = {:?}", e),
    }
}
