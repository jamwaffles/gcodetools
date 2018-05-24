use nom::types::CompleteByteSlice;

use super::Token;

#[derive(Debug, PartialEq)]
pub enum SpindleRotation {
    Cw,
    Ccw,
    Stop,
}

#[derive(Debug, PartialEq)]
pub enum Coolant {
    Mist,
    Flood,
    Off,
}

named!(tool_change<CompleteByteSlice, Token>,
    map!(tag!("M6"), |_| Token::ToolChange)
);

named!(mist_coolant<CompleteByteSlice, Token>,
    map!(tag!("M7"), |_| Token::Coolant(Coolant::Mist))
);

named!(flood_coolant<CompleteByteSlice, Token>,
    map!(tag!("M8"), |_| Token::Coolant(Coolant::Flood))
);

named!(disable_coolant<CompleteByteSlice, Token>,
    map!(tag!("M9"), |_| Token::Coolant(Coolant::Off))
);

named!(spindle_rotation<CompleteByteSlice, Token>, map!(
    alt!(
        map!(terminated!(tag_no_case!("M3"), not!(char!('0'))), |_| SpindleRotation::Cw) |
        map!(tag_no_case!("M4"), |_| SpindleRotation::Ccw) |
        map!(tag_no_case!("M5"), |_| SpindleRotation::Stop)
    ),
    |res| Token::SpindleRotation(res)
));

named!(pub mcode<CompleteByteSlice, Token>,
    alt_complete!(tool_change | spindle_rotation | mist_coolant | flood_coolant | disable_coolant)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, Token), nom::Err<CompleteByteSlice>>,
        against: Token,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_coolant() {
        check_token(mist_coolant(Cbs(b"M7")), Token::Coolant(Coolant::Mist));
        check_token(flood_coolant(Cbs(b"M8")), Token::Coolant(Coolant::Flood));
        check_token(disable_coolant(Cbs(b"M9")), Token::Coolant(Coolant::Off));
    }

    #[test]
    fn it_parses_spindle_rotation() {
        check_token(
            spindle_rotation(Cbs(b"M3")),
            Token::SpindleRotation(SpindleRotation::Cw),
        );
        check_token(
            spindle_rotation(Cbs(b"M4")),
            Token::SpindleRotation(SpindleRotation::Ccw),
        );
        check_token(
            spindle_rotation(Cbs(b"M5")),
            Token::SpindleRotation(SpindleRotation::Stop),
        );

        // It gets confused with M30
        assert_eq!(
            spindle_rotation(Cbs(b"M30")),
            Err(nom::Err::Error(nom::simple_errors::Context::Code(
                CompleteByteSlice(b"M30"),
                nom::ErrorKind::Alt
            )))
        );
    }

    #[test]
    fn it_changes_tool() {
        check_token(tool_change(Cbs(b"M6")), Token::ToolChange);
    }
}
