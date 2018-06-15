use nom::types::CompleteByteSlice;

use super::super::helpers::preceded_u32;
use super::MCode;

/// What kind of event to wait for
#[derive(Debug, PartialEq)]
pub enum InputTrigger {
    /// Sample the current value
    Immediate,
    /// Wait for a rising edge
    RisingEdge,
    /// Wait for a falling edge
    FallingEdge,
    /// Wait for the input to go high
    High,
    /// Wait for the input to go low
    Low,
}

/// Analog or digital
#[derive(Debug, PartialEq)]
pub enum InputType {
    Analog,
    Digital,
}

/// Wait for input
#[derive(Debug, PartialEq)]
pub struct WaitForInput {
    input_number: u32,
    input_type: InputType,
    timeout: Option<u32>,
    trigger: InputTrigger,
}

named!(analog_sample<CompleteByteSlice, WaitForInput>, ws!(do_parse!(
    m_code!("66") >>
    input_number: call!(preceded_u32, "E") >>
    tag_no_case!("L0") >> ({
        WaitForInput {
            input_number,
            input_type: InputType::Analog,
            timeout: None,
            trigger: InputTrigger::Immediate,
        }
    })
)));

named!(digital_sample<CompleteByteSlice, WaitForInput>, ws!(do_parse!(
    m_code!("66") >>
    input_number: call!(preceded_u32, "P") >>
    trigger_type: call!(preceded_u32, "L") >>
    timeout: opt!(call!(preceded_u32, "Q")) >> ({
        WaitForInput {
            input_number,
            input_type: InputType::Digital,
            timeout: timeout,
            trigger: match trigger_type {
                0 => InputTrigger::Immediate,
                1 => InputTrigger::RisingEdge,
                2 => InputTrigger::FallingEdge,
                3 => InputTrigger::High,
                4 => InputTrigger::Low,
                _ => unimplemented!()
            },
        }
    })
)));

// TODO: Good place to prototype parse error handling
named!(pub wait_for_input<CompleteByteSlice, MCode>, map!(
    alt_complete!(analog_sample | digital_sample),
    |res| MCode::WaitForInput(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    macro_rules! assert_expr {
        ($to_check:expr, $against:expr) => {
            assert_eq!($to_check, Ok((EMPTY, $against)))
        };
    }

    #[test]
    fn it_parses_wait_for_input() {
        assert_expr!(
            wait_for_input(Cbs(b"M66 P0 L3 Q5")),
            MCode::WaitForInput(WaitForInput {
                input_number: 0,
                input_type: InputType::Digital,
                timeout: Some(5),
                trigger: InputTrigger::High,
            })
        );
        assert_expr!(
            wait_for_input(Cbs(b"M66 P0 L1 Q5")),
            MCode::WaitForInput(WaitForInput {
                input_number: 0,
                input_type: InputType::Digital,
                timeout: Some(5),
                trigger: InputTrigger::RisingEdge,
            })
        );
        assert_expr!(
            wait_for_input(Cbs(b"M66 P2 L0")),
            MCode::WaitForInput(WaitForInput {
                input_number: 2,
                input_type: InputType::Digital,
                timeout: None,
                trigger: InputTrigger::Immediate,
            })
        );
        assert_expr!(
            wait_for_input(Cbs(b"M66 E3 L0")),
            MCode::WaitForInput(WaitForInput {
                input_number: 3,
                input_type: InputType::Analog,
                timeout: None,
                trigger: InputTrigger::Immediate,
            })
        );
    }
}
