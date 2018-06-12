pub mod parser;

use super::expression::Expression;
use super::tokenizer::ProgramTokens;

/// Subroutine name
#[derive(Clone, Debug, PartialEq)]
pub enum SubroutineName {
    /// Numbered subroutine (normal)
    Number(u32),

    /// External subroutine (referenced from a file)
    ///
    /// The name of the subroutine is the name of the file to refer to (no extension)
    External(String),
}

impl From<SubroutineName> for String {
    fn from(sub: SubroutineName) -> Self {
        match sub {
            SubroutineName::Number(num) => format!("O{}", num),
            SubroutineName::External(name) => format!("O<{}>", name),
        }
    }
}

/// Subroutine definition
#[derive(Debug, PartialEq)]
pub struct Subroutine {
    pub name: SubroutineName,
    pub tokens: ProgramTokens,
}

/// `while` block
#[derive(Debug, PartialEq)]
pub struct While {
    pub name: SubroutineName,
    pub tokens: ProgramTokens,
    pub condition: Expression,
}

// TODO: Elseif
/// `if`/`else` block
#[derive(Debug, PartialEq)]
pub struct If {
    pub name: SubroutineName,
    pub condition: Expression,
    pub if_tokens: ProgramTokens,
    pub else_tokens: Option<ProgramTokens>,
}

/// A subroutine call with optional arguments
#[derive(Debug, PartialEq)]
pub struct SubroutineCall {
    pub name: SubroutineName,
    pub args: Option<Vec<Expression>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subroutine_name_into_string() {
        let numbered: String = SubroutineName::Number(10u32).into();
        let external: String = SubroutineName::External("external_file".into()).into();

        assert_eq!(numbered, String::from("O10"));
        assert_eq!(external, String::from("O<external_file>"));
    }
}
