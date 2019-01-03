use nom::types::CompleteByteSlice;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<CompleteByteSlice<'a>>;
