use nom::types::CompleteByteSlice;

use super::helpers::*;
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

named!(pause<CompleteByteSlice, Token>,
    map!(call!(m, 0.0), |_| Token::Pause)
);

named!(optional_pause<CompleteByteSlice, Token>,
    map!(call!(m, 1.0), |_| Token::OptionalPause)
);

named!(end_program<CompleteByteSlice, Token>, map!(
    alt!(
        recognize!(call!(m, 30.0)) |
        recognize!(call!(m, 2.0))
    ),
    |_| Token::EndProgram
));

named!(tool_change<CompleteByteSlice, Token>,
    map!(call!(m, 6.0), |_| Token::ToolChange)
);

named!(mist_coolant<CompleteByteSlice, Token>,
    map!(call!(m, 7.0), |_| Token::Coolant(Coolant::Mist))
);

named!(flood_coolant<CompleteByteSlice, Token>,
    map!(call!(m, 8.0), |_| Token::Coolant(Coolant::Flood))
);

named!(disable_coolant<CompleteByteSlice, Token>,
    map!(call!(m, 9.0), |_| Token::Coolant(Coolant::Off))
);

named!(spindle_rotation<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(m, 3.0), |_| SpindleRotation::Cw) |
        map!(call!(m, 4.0), |_| SpindleRotation::Ccw) |
        map!(call!(m, 5.0), |_| SpindleRotation::Stop)
    ),
    |res| Token::SpindleRotation(res)
));

named!(modal_state_save<CompleteByteSlice, Token>,
    map!(call!(m, 70.0), |_| Token::ModalStateSave)
);

named!(modal_state_invalidate<CompleteByteSlice, Token>,
    map!(call!(m, 71.0), |_| Token::ModalStateInvalidate)
);

named!(modal_state_restore<CompleteByteSlice, Token>,
    map!(call!(m, 72.0), |_| Token::ModalStateRestore)
);

named!(modal_state_autorestore<CompleteByteSlice, Token>,
    map!(call!(m, 73.0), |_| Token::ModalStateAutoRestore)
);

named!(pub mcode<CompleteByteSlice, Token>,
    alt_complete!(
        tool_change |
        spindle_rotation |
        mist_coolant |
        flood_coolant |
        disable_coolant |
        pause |
        optional_pause |
        modal_state_save |
        modal_state_restore |
        modal_state_invalidate |
        modal_state_autorestore |
        end_program
    )
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
    fn it_parses_pauses() {
        check_token(mcode(Cbs(b"M0")), Token::Pause);
        check_token(mcode(Cbs(b"M00")), Token::Pause);

        check_token(mcode(Cbs(b"M1")), Token::OptionalPause);
        check_token(mcode(Cbs(b"M01")), Token::OptionalPause);
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
    fn it_parses_tool_changes() {
        check_token(tool_change(Cbs(b"M6")), Token::ToolChange);
    }

    #[test]
    fn it_parses_end_program() {
        check_token(mcode(Cbs(b"M2")), Token::EndProgram);
        check_token(mcode(Cbs(b"M30")), Token::EndProgram);
    }
}
