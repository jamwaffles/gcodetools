#![deny(/*missing_docs,*/
        missing_debug_implementations, /*missing_copy_implementations,*/
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

#[macro_use]
#[cfg(test)]
extern crate maplit;

#[macro_use]
extern crate nom;

pub mod expression;
pub mod subroutine;
pub mod tokenizer;
