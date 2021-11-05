use nom::bytes::complete::tag;
use nom::character::complete::satisfy;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, PartialOrd)]
struct Date {
    year: i64,
    month: i64,
    day: i64,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Time {
    hour: i64,
    minute: i64,
    second: i64,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Datetime {
    date: Date,
    time: Time,
}

fn digit(input: &str) -> IResult<&str, i64> {
    let (input, c) = satisfy(|c| c.is_digit(10))(input)?;
    Ok((input, c.to_digit(10).unwrap() as i64))
}

fn four_digits(input: &str) -> IResult<&str, i64> {
    let (input, n) = tuple((digit, digit, digit, digit))(input)?;
    Ok((input, n.0 * 1000 + n.1 * 100 + n.2 * 10 + n.3))
}

fn two_digits(input: &str) -> IResult<&str, i64> {
    let (input, n) = tuple((digit, digit))(input)?;
    Ok((input, n.0 * 10 + n.1))
}

fn date(input: &str) -> IResult<&str, Date> {
    let (input, (year, _, month, _, day)) =
        tuple((four_digits, tag("-"), two_digits, tag("-"), two_digits))(input)?;
    Ok((input, Date { year, month, day }))
}

fn time(input: &str) -> IResult<&str, Time> {
    let (input, (hour, _, minute, _, second)) =
        tuple((two_digits, tag(":"), two_digits, tag(":"), two_digits))(input)?;
    Ok((
        input,
        Time {
            hour,
            minute,
            second,
        },
    ))
}

fn datetime(input: &str) -> IResult<&str, Datetime> {
    let (input, (date, _, time, _)) = tuple((date, opt(tag("T")), time, opt(tag("Z"))))(input)?;
    Ok((input, Datetime { date, time }))
}

fn main() {
    let result = datetime("2000-01-01T01:02:03Z");
    println!("{:?}", result);
}
