use std::fmt::{Debug, Formatter};
use std::ops::{Range, RangeFrom, RangeTo};
use std::str::{CharIndices, Chars, FromStr};

use nom::InputLength;
use nom::{
    AsBytes, Compare, CompareResult, InputIter, InputTake, Needed, Offset, ParseTo, Slice,
    UnspecializedInput,
};

pub use char_indexing::*;
pub use parse_measure::*;
pub use parse_recipe::*;

mod char_indexing;
mod parse_measure;
mod parse_recipe;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct ParserInput<'a> {
    input: &'a str,
    /// The index of the chars in the original string, not the bytes
    char_index: usize,
}

impl<'a> ParserInput<'a> {
    fn new(input: &'a str, char_index: usize) -> Self {
        Self { input, char_index }
    }

    fn char_index(&self, byte_offset: usize) -> usize {
        self.char_index + self.input.char_index_for_byte(byte_offset)
    }

    fn range(&self) -> Range<usize> {
        self.char_index..self.char_index + self.input.chars().count()
    }
}

impl<'a> InputLength for ParserInput<'a> {
    fn input_len(&self) -> usize {
        self.input.len()
    }
}
impl<'a> InputIter for ParserInput<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    fn iter_indices(&self) -> Self::Iter {
        self.input.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.input.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.input.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.input.slice_index(count)
    }
}

impl<'a> InputTake for ParserInput<'a> {
    fn take(&self, count: usize) -> Self {
        Self {
            input: self.input.take(count),
            char_index: self.char_index,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (suffix, prefix) = self.input.take_split(count);
        (
            Self::new(suffix, self.char_index(count)),
            Self::new(prefix, self.char_index),
        )
    }
}

impl<'a> Slice<Range<usize>> for ParserInput<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        let index = self.char_index + range.start;
        Self::new(self.input.slice(range), index)
    }
}

impl<'a> Slice<RangeFrom<usize>> for ParserInput<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        let index = self.char_index(range.start);
        Self::new(self.input.slice(range), index)
    }
}

impl<'a> Slice<RangeTo<usize>> for ParserInput<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        Self::new(self.input.slice(range), self.char_index)
    }
}

impl<'a> Offset for ParserInput<'a> {
    fn offset(&self, second: &Self) -> usize {
        self.input.offset(second.input)
    }
}

impl<'a, R: FromStr> ParseTo<R> for ParserInput<'a> {
    fn parse_to(&self) -> Option<R> {
        self.input.parse().ok()
    }
}

impl<'a> AsBytes for ParserInput<'a> {
    fn as_bytes(&self) -> &[u8] {
        self.input.as_bytes()
    }
}

impl<'a> UnspecializedInput for ParserInput<'a> {}

impl<'a, 'b> Compare<&'b str> for ParserInput<'a> {
    fn compare(&self, t: &'b str) -> CompareResult {
        self.input.compare(t)
    }

    fn compare_no_case(&self, t: &'b str) -> CompareResult {
        self.input.compare_no_case(t)
    }
}

impl<'a, 'b> Compare<&'b [u8]> for ParserInput<'a> {
    fn compare(&self, t: &'b [u8]) -> CompareResult {
        self.input.compare(t)
    }

    fn compare_no_case(&self, t: &'b [u8]) -> CompareResult {
        self.input.compare_no_case(t)
    }
}

impl<'a> From<&'a str> for ParserInput<'a> {
    fn from(input: &'a str) -> Self {
        Self::new(input, 0)
    }
}

impl<'a> From<ParserInput<'a>> for &'a str {
    fn from(val: ParserInput<'a>) -> Self {
        val.input
    }
}

impl<'a> Debug for ParserInput<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}->{}: {}",
            self.char_index,
            self.char_index + self.input.len(),
            self.input
        )
    }
}

#[cfg(test)]
mod test {
    use nom::{IResult, Parser};

    use crate::parser::ParserInput;

    #[test]
    fn test_char_offset() {
        assert_eq!(ParserInput::new("½2", 0).char_index(2), 1);
        assert_eq!(ParserInput::new("½2", 0).char_index(3), 2);
    }

    #[test]
    fn test_range() {
        assert_eq!(ParserInput::new("½2", 0).range(), 0..2);
        assert_eq!(ParserInput::new("12", 0).range(), 0..2);
        assert_eq!(ParserInput::new("½½", 0).range(), 0..2);
        assert_eq!(ParserInput::new("1½1", 0).range(), 0..3);
        assert_eq!(ParserInput::new("1½1", 1).range(), 1..4);
        assert_eq!(ParserInput::new("1½1", 2).range(), 2..5);
    }

    /// Drop ParserInput for testing
    pub fn raw<'a, P, O, E>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        P: Parser<ParserInput<'a>, O, E>,
    {
        move |input: &str| {
            let (remainder, result) = parser.parse(input.into())?;
            Ok((remainder.into(), result))
        }
    }
}
