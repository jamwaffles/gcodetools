//! Useful general purpose parsers

use nom::{error::ParseError, AsBytes, AsChar, Err, IResult, InputIter, Slice};
use std::ops::RangeFrom;

/// Like `tag_no_case`, but matches against a single char. Assumes input is ASCII as it operates on
/// bytes, not characters
pub fn char_no_case<I, Error: ParseError<I>>(c: char) -> impl Fn(I) -> IResult<I, char, Error>
where
    I: AsBytes + Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    move |i: I| {
        if i.as_bytes()
            .first()
            .map(|f| f.eq_ignore_ascii_case(&(c as u8)))
            .unwrap_or(false)
        {
            Ok((i.slice(1..), c.as_char()))
        } else {
            Err(Err::Error(Error::from_char(i, c)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_case_char_comp() {
        assert!(char_no_case::<&str, ()>('A')("A").is_ok());
        assert!(char_no_case::<&str, ()>('A')("a").is_ok());
    }
}
