use std::fmt::{Debug, Formatter};

use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::fold_many0;
use nom::{Finish, InputLength};

use crate::parser::{parse_measure, MeasureToken, ParserInput};

pub struct Recipe<'a> {
    pub tokens: Vec<MeasureToken<'a>>,
    pub raw: &'a str,
}
// pub enum DocumentToken<'a> {
//     MeasureToken {
//         measure: Measure,
//         raw: &'a str,
//         range: Range<usize>,
//     },
//     Other {
//         raw: &'a str,
//         range: Range<usize>,
//     },
// }

pub fn parse_recipe<'a, I: Into<ParserInput<'a>>>(
    input: I,
) -> Result<Recipe<'a>, nom::error::Error<ParserInput<'a>>> {
    let input = input.into();
    let (remainder, tokens) = fold_many0(
        // try to parse a measure, if we can't just remove a char off the front and try again
        alt((map(parse_measure, Some), map(take(1usize), |_| None))),
        Vec::new,
        |mut tokens, token| {
            if let Some(token) = token {
                tokens.push(token);
            }
            tokens
        },
    )(input)
    .finish()?;

    if remainder.input_len() == 0 {
        Ok(Recipe {
            tokens,
            raw: input.input,
        })
    } else {
        panic!(
            "We should have consumed the entire document but found: {}",
            remainder.input
        )
    }
}

// pub fn parse<'a, I: Into<ParserInput<'a>>>(input: I) -> IResult<ParserInput<'a>, Document<'a>> {
//     let input = input.into();
//     let (remainder, (mut tokens, other)) = fold_many0(
//         alt((parse_measure, parse_other)),
//         || (vec![], None),
//         |(mut tokens, other): (_, Option<Range<usize>>), token| match (other, token) {
//             (Some(other_range), DocumentToken::Other { range, .. }) => {
//                 (tokens, Some(other_range.start..range.end))
//             }
//             (None, DocumentToken::Other { range, .. }) => (tokens, Some(range)),
//             (other, token @ DocumentToken::MeasureToken { .. }) => {
//                 if let Some(range) = other {
//                     tokens.push(DocumentToken::Other {
//                         raw: &input.input[range.clone()],
//                         range,
//                     })
//                 }
//                 tokens.push(token);
//                 (tokens, None)
//             }
//         },
//     )(input)?;
//
//     if let Some(range) = other {
//         tokens.push(DocumentToken::Other {
//             raw: &input.input[range.clone()],
//             range,
//         })
//     }
//
//     Ok((remainder, Document { tokens }))
// }

// fn parse_other(input: ParserInput) -> IResult<ParserInput, DocumentToken> {
//     let (remainder, token) = alt((multispace1, take_till1(primitive::char::is_whitespace)))(input)?;
//
//     Ok((
//         remainder,
//         DocumentToken::Other {
//             raw: token.input,
//             range: input.range(&token),
//         },
//     ))
// }

impl<'a> Debug for Recipe<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tokens)
    }
}

// impl<'a> Debug for DocumentToken<'a> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             DocumentToken::MeasureToken {
//                 measure,
//                 raw,
//                 range,
//             } => {
//                 write!(f, "({range:?}: {measure:?} @ \"{raw}\")")
//             }
//             DocumentToken::Other { raw, range } => write!(f, "({range:?}: \"{raw}\")"),
//         }
//     }
// }

#[cfg(test)]
mod test {

    #[test]
    fn test_parse() {
        // println!("Document: {:?}", parse_recipe("(2 large lemons)"));
        // println!("Document: {:?}", parse_recipe("(2 1/3 cups)"));
        // assert_eq!(
        //     parse_recipe("½2 cup cooked red lentils").unwrap().tokens[0].range,
        //     1..6
        // );
    }
}
