//! Useful general purpose parsers

use nom::{error::ParseError, AsChar, Err, IResult, InputIter, Slice};
use std::ops::RangeFrom;

/// Like `tag_no_case`, but matches against a single char
pub fn char_no_case<I, Error: ParseError<I>>(c: char) -> impl Fn(I) -> IResult<I, char, Error>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    move |i: I| match (i).iter_elements().next().map(|t| {
        let b = t.as_char().eq_ignore_ascii_case(&c);
        (&c, b)
    }) {
        Some((c, true)) => Ok((i.slice(c.len()..), c.as_char())),
        _ => Err(Err::Error(Error::from_char(i, c))),
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
