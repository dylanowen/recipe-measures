use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::ops::Range;
use std::primitive;

use nom::branch::alt;
use nom::character::complete::{alpha1, char, digit1, multispace0, one_of, u32};
use nom::combinator::{consumed, map, map_res, value};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use nom::InputLength;
use num_rational::Rational32;

use crate::parser::{CharIndexing, ParserInput};
use crate::{Measure, ParseError, Unit, UNITFUL_UNITS};

#[derive(Eq, PartialEq)]
pub struct MeasureToken<'a> {
    pub measure: Measure,
    pub number_range: Range<usize>,
    pub unit_range: Range<usize>,
    pub raw: Cow<'a, str>,
}

pub fn parse_measure(input: ParserInput) -> IResult<ParserInput, MeasureToken> {
    let (remainder, ((number_raw, number), _, (unit_raw, unit))) = alt((
        tuple((consumed(parse_integer), multispace0, consumed(parse_unit))),
        tuple((consumed(parse_decimal), multispace0, consumed(parse_unit))),
        tuple((consumed(parse_rational), multispace0, consumed(parse_unit))),
    ))(input)?;

    Ok((
        remainder,
        MeasureToken {
            measure: Measure::single(number, unit),
            number_range: number_raw.range(),
            unit_range: unit_raw.range(),
            raw: Cow::Borrowed(&input.input[..input.input_len() - remainder.input_len()]),
        },
    ))
}

fn parse_integer(input: ParserInput) -> IResult<ParserInput, Rational32> {
    map(u32, |i| Rational32::from_integer(i as i32))(input)
}

fn parse_decimal(input: ParserInput) -> IResult<ParserInput, Rational32> {
    map_res(
        separated_pair(
            u32::<ParserInput, _>,
            tuple((multispace0, char('.'), multispace0)),
            digit1,
        ),
        |(integer, fraction)| {
            let parsed_fraction = fraction.input.parse::<primitive::u32>()?;
            let fraction_rational = Rational32::new(
                parsed_fraction as i32,
                10_i32.pow(fraction.input.len() as u32),
            );

            Ok::<_, ParseIntError>(Rational32::from_integer(integer as i32) + fraction_rational)
        },
    )(input)
}

fn parse_rational(input: ParserInput) -> IResult<ParserInput, Rational32> {
    alt((multi_rational, simple_rational))(input)
}

/// Parse something of the form `<rational>`
fn simple_rational(input: ParserInput) -> IResult<ParserInput, Rational32> {
    alt((ascii_rational, unicode_rational))(input)
}

/// Parse something of the form `<real> <rational>`
fn multi_rational(input: ParserInput) -> IResult<ParserInput, Rational32> {
    map(
        separated_pair(u32, multispace0, simple_rational),
        |(integer, rational)| Rational32::from_integer(integer as i32) + rational,
    )(input)
}

fn ascii_rational(input: ParserInput) -> IResult<ParserInput, Rational32> {
    map_res(
        separated_pair(u32, tuple((multispace0, one_of("/⁄"), multispace0)), u32),
        |(numer, denom)| {
            if denom != 0 {
                Ok(Rational32::new(numer as i32, denom as i32))
            } else {
                Err(ParseError::InfiniteNumber)
            }
        },
    )(input)
}

fn unicode_rational(input: ParserInput) -> IResult<ParserInput, Rational32> {
    alt((
        // https://en.wikipedia.org/wiki/Latin-1_Supplement
        value(Rational32::new(1, 4), char('¼')),
        value(Rational32::new(1, 2), char('½')),
        value(Rational32::new(3, 4), char('¾')),
        // https://en.wikipedia.org/wiki/Number_Forms
        value(Rational32::new(1, 7), char('⅐')),
        value(Rational32::new(1, 9), char('⅑')),
        value(Rational32::new(1, 10), char('⅒')),
        value(Rational32::new(1, 3), char('⅓')),
        value(Rational32::new(2, 3), char('⅔')),
        value(Rational32::new(1, 5), char('⅕')),
        value(Rational32::new(2, 5), char('⅖')),
        value(Rational32::new(3, 5), char('⅗')),
        value(Rational32::new(4, 5), char('⅘')),
        value(Rational32::new(1, 6), char('⅙')),
        value(Rational32::new(5, 6), char('⅚')),
        value(Rational32::new(1, 8), char('⅛')),
        value(Rational32::new(3, 8), char('⅜')),
        value(Rational32::new(5, 8), char('⅝')),
        value(Rational32::new(7, 8), char('⅞')),
    ))(input)
}

fn parse_unit(input: ParserInput) -> IResult<ParserInput, Unit> {
    let (remainder, raw_unit) = alpha1(input)?;

    let mut secondary = None;
    for unit in UNITFUL_UNITS.iter() {
        for &alias in unit.aliases() {
            if raw_unit.input == alias {
                // if we have an exact match return immediately
                return Ok((remainder, unit.clone()));
            } else if raw_unit.input.to_lowercase() == alias.to_lowercase() {
                // only use a secondary match if we never find an exact match (for cases like t & T)
                secondary = Some(unit.clone())
            }
        }
    }

    Ok((
        remainder,
        secondary.unwrap_or(Unit::unitless(raw_unit.input.to_string())),
    ))
}

impl MeasureToken<'_> {
    pub fn new<'a, S: Into<Cow<'a, str>>>(
        measure: Measure,
        number_range: Range<usize>,
        unit_range: Range<usize>,
        raw: S,
    ) -> MeasureToken<'a> {
        MeasureToken {
            measure,
            number_range,
            unit_range,
            raw: raw.into(),
        }
    }

    pub fn into_owned(self) -> MeasureToken<'static> {
        Self::new(
            self.measure,
            self.number_range,
            self.unit_range,
            self.raw.into_owned(),
        )
    }
}

impl<'a> MeasureToken<'a> {
    pub fn full_range(&self) -> Range<usize> {
        self.number_range.start..self.unit_range.end
    }

    pub fn number_text(&self) -> Cow<'a, str> {
        self.raw
            .char_slice(0..self.number_range.end - self.number_range.start)
            .expect("number_range is outside of our raw text")
    }

    pub fn unit_text(&self) -> Cow<'a, str> {
        self.raw
            .char_slice(
                self.unit_range.start - self.number_range.start
                    ..self.unit_range.end - self.number_range.start,
            )
            .expect("number_range is outside of our raw text")
    }
}

impl<'a> Debug for MeasureToken<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let MeasureToken {
            measure,
            number_range,
            unit_range,
            raw,
            ..
        } = self;
        write!(
            f,
            "[{number_range:?}-{unit_range:?}): {measure:?} @ \"{raw}\"",
        )
    }
}

#[cfg(test)]
mod test {
    use crate::parser::test::raw;

    use super::*;

    #[test]
    fn test_parse_measure() {
        assert_eq!(
            raw(parse_measure)("3/4 teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(3, 4), Unit::Teaspoon),
                    0..3,
                    4..12,
                    "3/4 teaspoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("3/4  tablespoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(3, 4), Unit::Tablespoon),
                    0..3,
                    5..15,
                    "3/4  tablespoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("3/4teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(3, 4), Unit::Teaspoon),
                    0..3,
                    3..11,
                    "3/4teaspoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("3 /4teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(3, 4), Unit::Teaspoon),
                    0..4,
                    4..12,
                    "3 /4teaspoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("3/ 4teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(3, 4), Unit::Teaspoon),
                    0..4,
                    4..12,
                    "3/ 4teaspoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("3 / 4 teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(3, 4), Unit::Teaspoon),
                    0..5,
                    6..14,
                    "3 / 4 teaspoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("1 3 / 4 teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(7, 4), Unit::Teaspoon),
                    0..7,
                    8..16,
                    "1 3 / 4 teaspoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("1 3⁄4 teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(7, 4), Unit::Teaspoon),
                    0..5,
                    6..14,
                    "1 3⁄4 teaspoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("13⁄4 teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(13, 4), Unit::Teaspoon),
                    0..4,
                    5..13,
                    "13⁄4 teaspoon"
                )
            ))
        );
        assert_eq!(
            raw(parse_measure)("1 ¾ teaspoon other"),
            Ok((
                " other",
                MeasureToken::new(
                    Measure::single(Rational32::new(7, 4), Unit::Teaspoon),
                    0..3,
                    4..12,
                    "1 ¾ teaspoon"
                )
            ))
        );
        // Real Life Tests
        assert!(raw(parse_measure)("3. Line").is_err());
        assert_eq!(
            raw(parse_measure)("10 times").unwrap().1.measure,
            Measure::single(
                Rational32::from_integer(10),
                Unit::unitless("times".to_string())
            )
        );
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(
            raw(parse_integer)("1"),
            Ok(("", Rational32::from_integer(1)))
        );
        assert_eq!(
            raw(parse_integer)("1 cup"),
            Ok((" cup", Rational32::from_integer(1)))
        );
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(raw(parse_decimal)("0.2"), Ok(("", Rational32::new(1, 5))));
        assert_eq!(raw(parse_decimal)("0 .2"), Ok(("", Rational32::new(1, 5))));
        assert_eq!(raw(parse_decimal)("0. 2"), Ok(("", Rational32::new(1, 5))));
        assert_eq!(raw(parse_decimal)("0 . 2"), Ok(("", Rational32::new(1, 5))));
        assert_eq!(raw(parse_decimal)("1.2"), Ok(("", Rational32::new(6, 5))));
        assert_eq!(
            raw(parse_decimal)("1.12"),
            Ok(("", Rational32::new(112, 100)))
        );
        assert_eq!(
            raw(parse_decimal)("1.012"),
            Ok(("", Rational32::new(1012, 1000)))
        );
        assert_eq!(
            raw(parse_decimal)("0.2 cups"),
            Ok((" cups", Rational32::new(1, 5)))
        );
    }

    #[test]
    fn test_parse_rational() {
        assert_eq!(raw(parse_rational)("3/4"), Ok(("", Rational32::new(3, 4))));
        assert_eq!(raw(parse_rational)("3 /4"), Ok(("", Rational32::new(3, 4))));
        assert_eq!(raw(parse_rational)("3/ 4"), Ok(("", Rational32::new(3, 4))));
        assert_eq!(
            raw(parse_rational)("3 / 4"),
            Ok(("", Rational32::new(3, 4)))
        );
        assert_eq!(raw(parse_rational)("3⁄4"), Ok(("", Rational32::new(3, 4))));
        assert_eq!(raw(parse_rational)("¼"), Ok(("", Rational32::new(1, 4))));
        assert_eq!(
            raw(parse_rational)("1 3/4"),
            Ok(("", Rational32::new(7, 4)))
        );
        assert_eq!(
            raw(parse_rational)("13⁄4"),
            Ok(("", Rational32::new(13, 4)))
        );
        assert_eq!(
            raw(parse_rational)("1 3⁄4"),
            Ok(("", Rational32::new(7, 4)))
        );
        assert_eq!(raw(parse_rational)("1 ¾"), Ok(("", Rational32::new(7, 4))));
        assert_eq!(
            raw(parse_rational)("3/4 cups"),
            Ok((" cups", Rational32::new(3, 4)))
        );
        assert!(raw(parse_rational)("1").is_err());
        assert!(raw(parse_rational)("1.1").is_err());
        assert!(raw(parse_rational)("1 cups").is_err());
        assert!(raw(parse_rational)("1/0").is_err());
        assert!(raw(parse_rational)("1⁄0").is_err());
    }

    #[test]
    fn test_parse_units() {
        assert_eq!(raw(parse_unit)("drop"), Ok(("", Unit::Drop)));
        assert_eq!(raw(parse_unit)("t"), Ok(("", Unit::Teaspoon)));
        assert_eq!(raw(parse_unit)("T"), Ok(("", Unit::Tablespoon)));
        assert_eq!(raw(parse_unit)("Tb"), Ok(("", Unit::Tablespoon)));
        assert_eq!(raw(parse_unit)("c"), Ok(("", Unit::Cup)));
        assert_eq!(raw(parse_unit)("C"), Ok(("", Unit::Cup)));
        assert_eq!(
            parse_unit(ParserInput::from("C other")),
            Ok((ParserInput::new(" other", 1), Unit::Cup))
        );
    }
}
