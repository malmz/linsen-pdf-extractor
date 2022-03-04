use chrono::{DateTime, Datelike, TimeZone, Utc, Weekday};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, not_line_ending, space0, space1},
    combinator::{map, map_res},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn line_end(input: &str) -> IResult<&str, ()> {
    map(tuple((space0, newline)), |_| ())(input)
}

fn parse_week(input: &str) -> IResult<&str, u32> {
    let week_parser = delimited(tag("Cafè Linsen Vecka "), digit1, line_end);
    map_res(week_parser, |week: &str| week.parse::<u32>())(input)
}

fn parse_day(input: &str) -> IResult<&str, &str> {
    nom::branch::alt((
        tag("Måndag"),
        tag("Tisdag"),
        tag("Onsdag"),
        tag("Torsdag"),
        tag("Fredag"),
    ))(input)
}

fn parse_dishes(input: &str) -> IResult<&str, Vec<&str>> {
    let dish_parse = preceded(space1, not_line_ending);
    let parse_lines = separated_list1(line_end, dish_parse);
    let parse_first = not_line_ending;
    let (input, (first, mut rest)) =
        terminated(separated_pair(parse_first, line_end, parse_lines), newline)(input)?;

    rest.insert(0, first);
    Ok((input, rest))
}

fn parse_menu(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, day) = parse_day(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, dishes) = parse_dishes(input)?;
    Ok((input, (day, dishes)))
}

fn weekday(weekday: &str) -> Weekday {
    match weekday {
        "Måndag" => Weekday::Mon,
        "Tisdag" => Weekday::Tue,
        "Onsdag" => Weekday::Wed,
        "Torsdag" => Weekday::Thu,
        "Fredag" => Weekday::Fri,
        _ => Weekday::Mon,
    }
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub date: DateTime<Utc>,
    pub dishes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct WeekMenu {
    pub days: Vec<Menu>,
}

pub fn parse(input: &str) -> IResult<&str, WeekMenu> {
    let (input, week) = parse_week(input)?;
    let today = Utc::today();
    let year = today.year();

    let parse_menu = map(parse_menu, |(day, dishes)| Menu {
        date: Utc.isoywd(year, week, weekday(day)).and_hms(0, 0, 0),
        dishes: dishes.into_iter().map(|d| d.to_string()).collect(),
    });

    let (input, menu) = many0(parse_menu)(input)?;

    Ok((input, WeekMenu { days: menu }))
}
