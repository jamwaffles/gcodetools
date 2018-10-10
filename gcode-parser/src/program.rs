use crate::program;
use crate::ProgramTokens;
use crate::Token;
use nom::types::CompleteByteSlice;
use std::fs;
use std::path::Path;

/// Program struct
#[derive(Debug)]
pub struct Program {
    /// The list of parsed tokens that comprise a program
    tokens: ProgramTokens,
}

impl Program {
    // TODO: Error handling
    /// Parse GCode from an input string into a list of tokens
    pub fn from_str(input: &str) -> Self {
        let (_remaining, tokens) =
            program(CompleteByteSlice(input.as_bytes())).expect("Could not parse program");

        Self { tokens }
    }

    // TODO: Streaming parser
    // TODO: Error handling
    /// Parse a file into a token list
    ///
    /// This does not currently stream the file, so is not memory efficient
    pub fn from_file(path: &Path) -> Self {
        let contents = fs::read_to_string(path).expect("Could not read file");

        let (_remaining, tokens) =
            program(CompleteByteSlice(contents.as_bytes())).expect("Could not parse program");

        Self { tokens }
    }

    /// Get the tokens for this program
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
