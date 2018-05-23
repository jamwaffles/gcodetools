use nom::types::CompleteByteSlice;
use nom::*;

named_args!(
    pub preceded_f32<'a>(preceding: &str)<CompleteByteSlice<'a>, f32>,
    flat_map!(preceded!(tag_no_case!(preceding), recognize!(recognize_float)), parse_to!(f32))
);

named_args!(
    pub preceded_i32<'a>(preceding: &str)<CompleteByteSlice<'a>, i32>,
    flat_map!(preceded!(tag_no_case!(preceding), recognize!(preceded!(opt!(one_of!("+-")), digit))), parse_to!(i32))
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    #[test]
    fn it_parses_preceded_floats() {
        assert_eq!(preceded_f32(Cbs(b"x1.23"), "X"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"y-1.23"), "Y"), Ok((EMPTY, -1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"z+1.23"), "Z"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"a123"), "A"), Ok((EMPTY, 123.0f32)));

        assert_eq!(preceded_f32(Cbs(b"X1.23"), "X"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"Y-1.23"), "Y"), Ok((EMPTY, -1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"Z+1.23"), "Z"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"A123"), "A"), Ok((EMPTY, 123.0f32)));
    }

    #[test]
    fn it_parses_preceded_signed_integers() {
        assert_eq!(preceded_i32(Cbs(b"x123"), "X"), Ok((EMPTY, 123i32)));
        assert_eq!(preceded_i32(Cbs(b"y-123"), "Y"), Ok((EMPTY, -123i32)));

        assert_eq!(preceded_i32(Cbs(b"X123"), "X"), Ok((EMPTY, 123i32)));
        assert_eq!(preceded_i32(Cbs(b"Y-123"), "Y"), Ok((EMPTY, -123i32)));
    }
}
