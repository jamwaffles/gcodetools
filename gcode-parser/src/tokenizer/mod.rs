mod helpers;

use nom::types::CompleteByteSlice;

use self::helpers::*;

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new_from_str() -> Self {
        Tokenizer {}
    }

    pub fn tokenize(&self) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Units {
    Inch,
    Mm,
}

#[derive(Debug, PartialEq)]
pub enum DistanceMode {
    Absolute,
    Incremental,
}

#[derive(Debug, PartialEq)]
pub struct PathBlending {
    pub p: Option<f32>,
    pub q: Option<f32>,
}

#[derive(Debug, PartialEq)]
pub enum CutterCompensation {
    Off,
}

#[derive(Debug, PartialEq)]
pub enum Plane {
    Xy,
    Zx,
    Yz,
    Uv,
    Wu,
    Vw,
}

#[derive(Debug, PartialEq)]
pub enum SpindleRotation {
    Cw,
    Ccw,
    Stop,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Units(Units),
    DistanceMode(DistanceMode),
    PathBlending(PathBlending),
    CutterCompensation(CutterCompensation),
    RapidMove(Vec9),
    LinearMove(Vec9),
    ToolSelect(u32),
    ToolChange,
    PlaneSelect(Plane),
    SpindleRotation(SpindleRotation),
    SpindleSpeed(i32),
}

pub type Program = Vec<Token>;

named!(comment<CompleteByteSlice, Token>, map!(
    flat_map!(
        delimited!(tag!("("), take_until!(")"), tag!(")")),
        parse_to!(String)
    ),
    |res| Token::Comment(res.trim().into())
));

named!(units<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("G20"), |_| Units::Inch) |
        map!(tag_no_case!("G21"), |_| Units::Mm)
    ),
    |res| Token::Units(res)
));

named!(distance_mode<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("G90"), |_| DistanceMode::Absolute) |
        map!(tag_no_case!("G91"), |_| DistanceMode::Incremental)
    ),
    |res| Token::DistanceMode(res)
));

named!(path_blending<CompleteByteSlice, Token>, ws!(
    do_parse!(
        tag_no_case!("G64") >>
        p: opt!(call!(preceded_f32, "P")) >>
        q: opt!(call!(preceded_f32, "Q")) >> ({
            Token::PathBlending(PathBlending { p, q })
        })
    )
));

named!(cutter_compensation<CompleteByteSlice, Token>,
    map!(
        alt!(
            map!(tag_no_case!("G40"), |_| CutterCompensation::Off)
        ),
        |res| Token::CutterCompensation(res)
    )
);

named!(rapid_move<CompleteByteSlice, Token>,
    map!(ws!(preceded!(tag!("G0"), vec9)), |res| Token::RapidMove(res))
);

named!(linear_move<CompleteByteSlice, Token>,
    map!(ws!(preceded!(tag!("G1"), vec9)), |res| Token::LinearMove(res))
);

named!(tool_number<CompleteByteSlice, Token>,
    map!(call!(preceded_u32, "T"), |res| Token::ToolSelect(res))
);

named!(tool_change<CompleteByteSlice, Token>,
    map!(tag!("M6"), |_| Token::ToolChange)
);

named!(plane_select<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("G17.1"), |_| Plane::Uv) |
        map!(tag_no_case!("G18.1"), |_| Plane::Wu) |
        map!(tag_no_case!("G19.1"), |_| Plane::Vw) |
        map!(tag_no_case!("G17"), |_| Plane::Xy) |
        map!(tag_no_case!("G18"), |_| Plane::Zx) |
        map!(tag_no_case!("G19"), |_| Plane::Yz)
    ),
    |res| Token::PlaneSelect(res)
));

named!(spindle_rotation<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("M3"), |_| SpindleRotation::Cw) |
        map!(tag_no_case!("M4"), |_| SpindleRotation::Ccw) |
        map!(tag_no_case!("M5"), |_| SpindleRotation::Stop)
    ),
    |res| Token::SpindleRotation(res)
));

named!(spindle_speed<CompleteByteSlice, Token>, map!(
    call!(preceded_i32, "S"),
    |res| Token::SpindleSpeed(res)
));

named!(token<CompleteByteSlice, Token>,
    alt_complete!(
        comment |
        units |
        distance_mode |
        path_blending |
        cutter_compensation |
        rapid_move |
        linear_move |
        tool_number |
        tool_change |
        plane_select |
        spindle_rotation |
        spindle_speed
    )
);

named!(tokens<CompleteByteSlice, Vec<Token>>, ws!(many0!(token)));

named!(program<CompleteByteSlice, Program>,
    ws!(delimited!(tag!("%"), many0!(token), tag!("%")))
);

// Note: programs are either dlimited by % signs or stop at M2/M30. Anything after a trailing %/M2/
// M30 MUST be ignored

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
    fn it_parses_a_percent_delimited_program() {
        let percents = r#"%
G21
G0 x0 y0 z0
G1 Z10
%
G0 Z10
"#;

        let percents_program = program(Cbs(percents.as_bytes()));

        assert_eq!(
            percents_program,
            Ok((
                // Ignore anything after last %
                Cbs(b"G0 Z10\n"),
                vec![
                    Token::Units(Units::Mm),
                    Token::RapidMove(Vec9 {
                        x: Some(0.0),
                        y: Some(0.0),
                        z: Some(0.0),
                        ..Default::default()
                    }),
                    Token::LinearMove(Vec9 {
                        z: Some(10.0),
                        ..Default::default()
                    }),
                ]
            ))
        );
    }

    #[test]
    fn it_parses_spindle_speed() {
        check_token(spindle_speed(Cbs(b"S0")), Token::SpindleSpeed(0i32));
        check_token(spindle_speed(Cbs(b"S1000")), Token::SpindleSpeed(1000i32));
        check_token(spindle_speed(Cbs(b"S-250")), Token::SpindleSpeed(-250i32));
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
    }

    #[test]
    fn it_parses_plane_select() {
        check_token(plane_select(Cbs(b"G17")), Token::PlaneSelect(Plane::Xy));
        check_token(plane_select(Cbs(b"G18")), Token::PlaneSelect(Plane::Zx));
        check_token(plane_select(Cbs(b"G19")), Token::PlaneSelect(Plane::Yz));
        check_token(plane_select(Cbs(b"G17.1")), Token::PlaneSelect(Plane::Uv));
        check_token(plane_select(Cbs(b"G18.1")), Token::PlaneSelect(Plane::Wu));
        check_token(plane_select(Cbs(b"G19.1")), Token::PlaneSelect(Plane::Vw));
    }

    #[test]
    fn it_changes_tool() {
        check_token(tool_change(Cbs(b"M6")), Token::ToolChange);
    }

    #[test]
    fn it_parses_tool_number() {
        check_token(tool_number(Cbs(b"T0")), Token::ToolSelect(0u32));
        check_token(tool_number(Cbs(b"T99")), Token::ToolSelect(99u32));
    }

    #[test]
    fn it_parses_rapids() {
        check_token(
            rapid_move(Cbs(b"G0 X0 Y1 Z2")),
            Token::RapidMove(Vec9 {
                x: Some(0.0f32),
                y: Some(1.0f32),
                z: Some(2.0f32),
                ..Default::default()
            }),
        );
    }

    #[test]
    fn it_parses_linear_moves() {
        check_token(
            linear_move(Cbs(b"G1 X0 Y1 Z2")),
            Token::LinearMove(Vec9 {
                x: Some(0.0f32),
                y: Some(1.0f32),
                z: Some(2.0f32),
                ..Default::default()
            }),
        );
    }

    #[test]
    fn it_parses_cutter_comp() {
        check_token(
            cutter_compensation(Cbs(b"G40")),
            Token::CutterCompensation(CutterCompensation::Off),
        );

        // TODO
        // assert_eq!(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     Ok((
        //         EMPTY,
        //         Token::PathBlending(PathBlending { p: None, q: None })
        //     ))
        // );
    }

    #[test]
    fn it_parses_blending_mode() {
        check_token(
            path_blending(Cbs(b"G64")),
            Token::PathBlending(PathBlending { p: None, q: None }),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01")),
            Token::PathBlending(PathBlending {
                p: Some(0.01f32),
                q: None,
            }),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01 Q0.02")),
            Token::PathBlending(PathBlending {
                p: Some(0.01f32),
                q: Some(0.02f32),
            }),
        );

        // TODO
        // check_token(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     Token::PathBlending(PathBlending { p: None, q: None })
        // );
    }

    #[test]
    fn it_parses_distance_mode() {
        check_token(
            distance_mode(Cbs(b"G90")),
            Token::DistanceMode(DistanceMode::Absolute),
        );

        check_token(
            distance_mode(Cbs(b"G91")),
            Token::DistanceMode(DistanceMode::Incremental),
        );
    }
    #[test]
    fn it_parses_units() {
        check_token(units(Cbs(b"G20")), Token::Units(Units::Inch));
        check_token(units(Cbs(b"G21")), Token::Units(Units::Mm));
    }

    #[test]
    fn it_parses_comments() {
        check_token(
            comment(Cbs(b"(Hello world)")),
            Token::Comment("Hello world".into()),
        );

        check_token(
            comment(Cbs(b"( Hello world )")),
            Token::Comment("Hello world".into()),
        );
    }
}
