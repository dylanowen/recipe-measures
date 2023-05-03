use std::borrow::Cow;
use std::ops::Range;

pub trait CharIndexing {
    fn char_slice(&self, range: Range<usize>) -> Option<Self>
    where
        Self: Sized;

    fn char_index_for_byte(&self, byte_offset: usize) -> usize;
}

impl<'a> CharIndexing for &'a str {
    fn char_slice(&self, char_range: Range<usize>) -> Option<Self> {
        if !char_range.is_empty() {
            let mut iter = self.char_indices().map(|(i, _)| i);
            let byte_start = iter.nth(char_range.start);
            let byte_end = iter.nth(char_range.end - char_range.start - 1);
            if let (Some(start), Some(end)) = (byte_start, byte_end) {
                Some(&self[start..end])
            } else {
                None
            }
        } else {
            Some("")
        }
    }

    fn char_index_for_byte(&self, mut byte_offset: usize) -> usize {
        let char_offset = self.chars().enumerate().find_map(|(i, c)| {
            if byte_offset > 0 {
                // if we index into the middle of a char we'll still get it
                byte_offset = byte_offset.checked_sub(c.len_utf8()).unwrap_or_default();
                None
            } else {
                Some(i)
            }
        });
        char_offset.unwrap_or_else(|| self.chars().count())
    }
}

impl CharIndexing for String {
    fn char_slice(&self, range: Range<usize>) -> Option<Self> {
        self.as_str().char_slice(range).map(ToString::to_string)
    }

    fn char_index_for_byte(&self, byte_offset: usize) -> usize {
        self.as_str().char_index_for_byte(byte_offset)
    }
}

impl<'a> CharIndexing for Cow<'a, str> {
    fn char_slice(&self, range: Range<usize>) -> Option<Self> {
        match self {
            Cow::Borrowed(s) => s.char_slice(range).map(Into::into),
            Cow::Owned(s) => s.char_slice(range).map(Into::into),
        }
    }

    fn char_index_for_byte(&self, byte_offset: usize) -> usize {
        self.as_ref().char_index_for_byte(byte_offset)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::char_indexing::CharIndexing;

    #[test]
    fn test_char_slice() {
        assert_eq!("12345".char_slice(0..1).unwrap(), "1");
        assert_eq!("⅑45".char_slice(0..2).unwrap(), "⅑4");
        assert_eq!("⅑45".char_slice(1..1).unwrap(), "");
        assert_eq!("⅑45".char_slice(1..2).unwrap(), "4");
        assert_eq!("456⅐⅙⅑45".char_slice(4..5).unwrap(), "⅙");
        assert_eq!("1⁄945".char_slice(1..4).unwrap(), "⁄94");
        assert_eq!("1234".char_slice(0..0).unwrap(), "");
        assert_eq!("".char_slice(0..1), None);
        assert_eq!(
            "⅑45".to_string().char_slice(0..2).unwrap(),
            "⅑4".to_string()
        );
    }

    #[test]
    fn test_char_for_byte_index() {
        assert_eq!("456⅐⅙⅑45".char_index_for_byte(0), 0);
        assert_eq!("456⅐⅙⅑45".char_index_for_byte(1), 1);
        assert_eq!("456⅐⅙⅑45".char_index_for_byte(4), 4);
        assert_eq!("456⅐⅙⅑45".char_index_for_byte(5), 4);
        assert_eq!("456⅐⅙⅑45".char_index_for_byte(6), 4);
        assert_eq!("456⅐⅙⅑45".char_index_for_byte(7), 5);
        assert_eq!("456⅐⅙⅑45".char_index_for_byte(10), 6);
        assert_eq!("".char_index_for_byte(10), 0);
        assert_eq!("1".char_index_for_byte(10), 1);
        assert_eq!("456⅐⅙⅑45".to_string().char_index_for_byte(10), 6);
    }
}
