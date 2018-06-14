extern crate gcode_parser;
extern crate nom;

use gcode_parser::prelude::*;
use gcode_parser::*;
use nom::types::CompleteByteSlice as Cbs;

const EMPTY: Cbs = Cbs(b"");

#[test]
fn it_parses_expressions_in_arcs() {
    let input = r#"
    g3 x#1 y[#2 + [#5 / 2]] i0 j[0 - [#5 / 2]] z#8
"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::GCode(GCode::CounterclockwiseArc),
                Token::CenterArc(CenterArc {
                    x: Some(Value::Parameter(Parameter::Numbered(1))),
                    y: Some(Value::Expression(vec![
                        ExpressionToken::Parameter(Parameter::Numbered(2)),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Expression(vec![
                            ExpressionToken::Parameter(Parameter::Numbered(5)),
                            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
                            ExpressionToken::Literal(2.0),
                        ]),
                    ])),
                    i: Some(Value::Float(0.0)),
                    j: Some(Value::Expression(vec![
                        ExpressionToken::Literal(0.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                        ExpressionToken::Expression(vec![
                            ExpressionToken::Parameter(Parameter::Numbered(5)),
                            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
                            ExpressionToken::Literal(2.0),
                        ]),
                    ])),
                    z: Some(Value::Parameter(Parameter::Numbered(8))),
                    ..Default::default()
                }),
            ]
        ))
    );
}
