use chrono::{Datelike, TimeZone, Utc, Weekday};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, not_line_ending, space0, space1},
    combinator::{map, map_res},
    multi::{fold_many1, many0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{Dishes, Menu, WeekMenu};

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

fn parse_dish_line(input: &str) -> IResult<&str, &str> {
    delimited(space1, not_line_ending, line_end)(input)
}

fn parse_first_dish(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, swedish) = terminated(not_line_ending, newline)(input)?;
    let (input, english) = parse_dish_line(input)?;
    Ok((input, (swedish, english)))
}

fn parse_dishes(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, first_dish) = parse_first_dish(input)?;
    let dishes = vec![first_dish];
    let (input, dishes) = fold_many1(
        pair(parse_dish_line, parse_dish_line),
        || dishes.clone(),
        |mut acc, val| {
            acc.push(val);
            acc
        },
    )(input)?;

    Ok((input, dishes))
}

fn parse_menu(input: &str) -> IResult<&str, (&str, Vec<(&str, &str)>)> {
    let (input, day) = terminated(parse_day, tag(": "))(input)?;
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

pub fn parse(input: &str) -> IResult<&str, WeekMenu> {
    let (input, week) = parse_week(input)?;
    let today = Utc::today();
    let year = today.year();

    let parse_menu = map(parse_menu, |(day, dishes)| Menu {
        date: Utc.isoywd(year, week, weekday(day)).and_hms(0, 0, 0),
        dishes: dishes
            .into_iter()
            .map(|(s, e)| (s.to_owned(), e.to_owned()))
            .fold(Dishes::default(), |mut acc, (s, e)| {
                acc.swedish.push(s);
                acc.english.push(e);
                acc
            }),
    });

    let (input, menu) = many0(parse_menu)(input)?;

    Ok((input, WeekMenu { days: menu }))
}
