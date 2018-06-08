pub mod parser;

use super::expression::Expression;
use super::tokenizer::ProgramTokens;

#[derive(Clone, Debug, PartialEq)]
pub enum SubroutineName {
    Number(u32),
}

impl From<SubroutineName> for String {
    fn from(sub: SubroutineName) -> Self {
        match sub {
            SubroutineName::Number(num) => format!("O{}", num),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Subroutine {
    pub name: SubroutineName,
    pub tokens: ProgramTokens,
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub name: SubroutineName,
    pub tokens: ProgramTokens,
    pub condition: Expression,
}
