// TODO: Is there a better place to keep `CodeNumber`? In this file perhaps?
// use crate::token::CodeNumber;
use nom::types::CompleteByteSlice;
use nom::*;

pub type CodeNumber = f32;

// named!(digit_u16<CompleteByteSlice, u16>, flat_map!(complete!(digit1), parse_to!(u16)));

// named!(code_number_decimal<CompleteByteSlice, f32>,
//     flat_map!(
//         recognize!(
//             delimited!(
//                 digit1,
//                 ws!(char!('.')),
//                 digit1
//             )
//         ),
//         parse_to!(f32)
//     )
// );

// named!(pub code_number_int<CompleteByteSlice, CodeNumber>,
//     map!(terminated!(digit_u16, not!(char!('.'))), |res| CodeNumber::Int(res))
// );

// named!(pub code_number<CompleteByteSlice, CodeNumber>,exact!(
//     alt_complete!(
//         map!(code_number_decimal, |res| CodeNumber::Float(res)) |
//         code_number_int
//     )
// ));

named!(pub code_number<CompleteByteSlice, CodeNumber>,
    flat_map!(
        recognize!(
            terminated!(
                digit1,
                opt!(terminated!(char!('.'), digit1))
            )
        ),
        parse_to!(f32)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal() {
        assert_parse!(code_number, CompleteByteSlice(b"59.1"), 59.1f32);
    }

    #[test]
    fn parse_int() {
        assert_parse!(code_number, CompleteByteSlice(b"54"), 54.0f32);
    }
}
