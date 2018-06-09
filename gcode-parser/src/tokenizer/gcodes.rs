use nom::types::CompleteByteSlice;

use super::helpers::*;
use super::vec9::{vec9, Vec9};
use super::Token;

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
pub enum PathBlendingMode {
    Blended((Option<f32>, Option<f32>)),
    ExactPath,
    // TODO
    // ExactStop,
}

#[derive(Debug, PartialEq)]
pub enum CutterCompensation {
    Off,
    Left(Option<u32>),
    Right(Option<u32>),
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
pub enum ToolLengthCompensation {
    Disable,
    // TODO: Tool number here
    ToolNumberOffset,
    Dynamic(Vec9),
}

#[derive(Debug, PartialEq)]
pub enum WorkOffset {
    G54,
    G55,
}

#[derive(Debug, PartialEq)]
pub enum FeedrateMode {
    InverseTime,
    UnitsPerMinute,
    UnitsPerRevolution,
}

named!(units<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 20.0), |_| Units::Inch) |
        map!(call!(g, 21.0), |_| Units::Mm)
    ),
    |res| Token::Units(res)
));

named!(distance_mode<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 90.0), |_| DistanceMode::Absolute) |
        map!(call!(g, 91.0), |_| DistanceMode::Incremental)
    ),
    |res| Token::DistanceMode(res)
));

named!(path_blending<CompleteByteSlice, Token>, map!(
    alt!(
        ws!(do_parse!(
            call!(g, 64.0) >>
            p: opt!(call!(preceded_f32, "P")) >>
            q: opt!(call!(preceded_f32, "Q")) >> ({
                PathBlendingMode::Blended((p, q))
            })
        )) |
        map!(call!(g, 61.0), |_| PathBlendingMode::ExactPath)
    ),
    |res| Token::PathBlendingMode(res)
));

named!(cutter_compensation<CompleteByteSlice, Token>,
    map!(
        alt!(
            map!(call!(g, 40.0), |_| CutterCompensation::Off) |
            map!(
                ws!(preceded!(call!(g, 41.0), opt!(call!(preceded_u32, "D")))),
                |tool| CutterCompensation::Left(tool)
            ) |
            map!(
                ws!(preceded!(call!(g, 42.0), opt!(call!(preceded_u32, "D")))),
                |tool| CutterCompensation::Right(tool)
            )
        ),
        |res| Token::CutterCompensation(res)
    )
);

named!(rapid_move<CompleteByteSlice, Token>,
    map!(call!(g, 0.0), |_| Token::RapidMove)
);

named!(linear_move<CompleteByteSlice, Token>,
    map!(call!(g, 1.0), |_| Token::LinearMove)
);

named!(cw_arc<CompleteByteSlice, Token>,
    map!(call!(g, 2.0), |_| Token::ClockwiseArc)
);

named!(ccw_arc<CompleteByteSlice, Token>,
    map!(call!(g, 3.0), |_| Token::CounterclockwiseArc)
);

named!(plane_select<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 17.1), |_| Plane::Uv) |
        map!(call!(g, 18.1), |_| Plane::Wu) |
        map!(call!(g, 19.1), |_| Plane::Vw) |
        map!(call!(g, 17.0), |_| Plane::Xy) |
        map!(call!(g, 18.0), |_| Plane::Zx) |
        map!(call!(g, 19.0), |_| Plane::Yz)
    ),
    |res| Token::PlaneSelect(res)
));

named!(tool_length_compensation<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 43.0), |_| ToolLengthCompensation::ToolNumberOffset) |
        map!(
            ws!(preceded!(call!(g, 43.1), vec9)),
            |offset| ToolLengthCompensation::Dynamic(offset)
        ) |
        map!(call!(g, 49.0), |_| ToolLengthCompensation::Disable)
    ),
    |res| Token::ToolLengthCompensation(res)
));

named!(canned_cycle<CompleteByteSlice, Token>,
    alt!(
        map!(call!(g, 80.0), |_| Token::CancelCannedCycle)
    )
);

named!(coordinate_system_offset<CompleteByteSlice, Token>,
    alt!(
        map!(call!(g, 92.0), |_| Token::CoordinateSystemOffset)
    )
);

named!(work_offset<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 54.0), |_| WorkOffset::G54) |
        map!(call!(g, 55.0), |_| WorkOffset::G55)
    ),
    |res| Token::WorkOffset(res)
));

named!(dwell<CompleteByteSlice, Token>, map!(
    ws!(preceded!(
        call!(g, 4.0),
        call!(preceded_f32, "P")
    )),
    |res| Token::Dwell(res)
));

named!(feedrate_mode<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 93.0), |_| FeedrateMode::InverseTime) |
        map!(call!(g, 94.0), |_| FeedrateMode::UnitsPerMinute) |
        map!(call!(g, 95.0), |_| FeedrateMode::UnitsPerRevolution)
    ),
    |res| Token::FeedrateMode(res)
));

named!(go_to_predefined_position<CompleteByteSlice, Token>,
    map!(call!(g, 28.0), |_| Token::GoToPredefinedPosition)
);

named!(store_predefined_position<CompleteByteSlice, Token>,
    map!(call!(g, 28.1), |_| Token::StorePredefinedPosition)
);

named!(pub gcode<CompleteByteSlice, Token>,
    alt_complete!(
        plane_select |
        units |
        distance_mode |
        path_blending |
        cutter_compensation |
        rapid_move |
        linear_move |
        tool_length_compensation |
        cw_arc |
        ccw_arc |
        canned_cycle |
        work_offset |
        dwell |
        coordinate_system_offset |
        feedrate_mode |
        go_to_predefined_position |
        store_predefined_position |
        path_blending
    )
);

#[cfg(test)]
mod tests {
    use super::super::value::Value;
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
    fn it_parses_plane_select() {
        check_token(plane_select(Cbs(b"G17")), Token::PlaneSelect(Plane::Xy));
        check_token(plane_select(Cbs(b"G18")), Token::PlaneSelect(Plane::Zx));
        check_token(plane_select(Cbs(b"G19")), Token::PlaneSelect(Plane::Yz));
        check_token(plane_select(Cbs(b"G17.1")), Token::PlaneSelect(Plane::Uv));
        check_token(plane_select(Cbs(b"G18.1")), Token::PlaneSelect(Plane::Wu));
        check_token(plane_select(Cbs(b"G19.1")), Token::PlaneSelect(Plane::Vw));
    }

    #[test]
    fn it_parses_dwells() {
        check_token(dwell(Cbs(b"G04 P10")), Token::Dwell(10.0));
        check_token(dwell(Cbs(b"G04 P3")), Token::Dwell(3.0));
        check_token(dwell(Cbs(b"G04 P0.5")), Token::Dwell(0.5));
        check_token(dwell(Cbs(b"G4 P0.5")), Token::Dwell(0.5));
        check_token(dwell(Cbs(b"g4p0.5")), Token::Dwell(0.5));
    }

    #[test]
    fn it_parses_work_offsets() {
        check_token(work_offset(Cbs(b"G54")), Token::WorkOffset(WorkOffset::G54));
        check_token(work_offset(Cbs(b"G55")), Token::WorkOffset(WorkOffset::G55));
    }

    #[test]
    fn it_parses_rapids() {
        check_token(rapid_move(Cbs(b"G0")), Token::RapidMove);
        check_token(rapid_move(Cbs(b"G00")), Token::RapidMove);
    }

    #[test]
    fn it_parses_linear_moves() {
        check_token(linear_move(Cbs(b"G1")), Token::LinearMove);
        check_token(linear_move(Cbs(b"G01")), Token::LinearMove);
    }

    #[test]
    fn it_parses_arcs() {
        check_token(cw_arc(Cbs(b"G2")), Token::ClockwiseArc);
        check_token(cw_arc(Cbs(b"G02")), Token::ClockwiseArc);
        check_token(ccw_arc(Cbs(b"G3")), Token::CounterclockwiseArc);
        check_token(ccw_arc(Cbs(b"G03")), Token::CounterclockwiseArc);
    }

    #[test]
    fn it_parses_cutter_comp() {
        check_token(
            cutter_compensation(Cbs(b"G40")),
            Token::CutterCompensation(CutterCompensation::Off),
        );

        check_token(
            cutter_compensation(Cbs(b"G41 D1")),
            Token::CutterCompensation(CutterCompensation::Left(Some(1u32))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42 D1")),
            Token::CutterCompensation(CutterCompensation::Right(Some(1u32))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42 D0")),
            Token::CutterCompensation(CutterCompensation::Right(Some(0u32))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42")),
            Token::CutterCompensation(CutterCompensation::Right(None)),
        );
    }

    #[test]
    fn it_parses_dynamic_tool_length_offset() {
        check_token(
            tool_length_compensation(Cbs(b"G43.1 Z0.250")),
            Token::ToolLengthCompensation(ToolLengthCompensation::Dynamic(Vec9 {
                z: Some(Value::Float(0.250)),
                ..Default::default()
            })),
        );
    }

    #[test]
    fn it_parses_blending_mode() {
        check_token(
            path_blending(Cbs(b"G64")),
            Token::PathBlendingMode(PathBlendingMode::Blended((None, None))),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01")),
            Token::PathBlendingMode(PathBlendingMode::Blended((Some(0.01f32), None))),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01 Q0.02")),
            Token::PathBlendingMode(PathBlendingMode::Blended((Some(0.01f32), Some(0.02f32)))),
        );

        // TODO
        // check_token(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     Token::PathBlendingMode(PathBlendingMode { p: None, q: None })
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
    fn it_parses_canned_cycles() {
        check_token(canned_cycle(Cbs(b"G80")), Token::CancelCannedCycle);
    }

    #[test]
    fn it_parses_feedrate_mode() {
        check_token(
            feedrate_mode(Cbs(b"G93")),
            Token::FeedrateMode(FeedrateMode::InverseTime),
        );
        check_token(
            feedrate_mode(Cbs(b"G94")),
            Token::FeedrateMode(FeedrateMode::UnitsPerMinute),
        );
        check_token(
            feedrate_mode(Cbs(b"G95")),
            Token::FeedrateMode(FeedrateMode::UnitsPerRevolution),
        );
    }

    #[test]
    fn it_goes_to_predefined_position() {
        check_token(
            go_to_predefined_position(Cbs(b"G28")),
            Token::GoToPredefinedPosition,
        );
    }

    #[test]
    fn it_stores_predefined_position() {
        check_token(
            store_predefined_position(Cbs(b"G28.1")),
            Token::StorePredefinedPosition,
        );
    }
}
