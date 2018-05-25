extern crate chardet;
extern crate encoding;
extern crate gcode_parser;
extern crate nom;

use chardet::{detect, charset2encoding};
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use gcode_parser::tokenizer::Tokenizer;

pub fn collect_source_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                match path.extension().unwrap_or(OsStr::new("")).to_str() {
                    Some("ngc") | Some("gcode") | Some("nc") | Some("txt") => {
                        files.push(path.to_path_buf())
                    }
                    _ => (),
                }
            } else if path.is_dir() {
                let sub = collect_source_files(&path).unwrap();
                for subfile in sub {
                    files.push(subfile);
                }
            }
        }
    }

    files.sort();

    Ok(files)
}

pub fn read(path: &Path) -> String {
    // open text file
    let mut fh = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Could not open file");
    let mut reader: Vec<u8> = Vec::new();

    // read file
    fh.read_to_end(&mut reader).expect("Could not read file");

    // detect charset of the file
    let result = detect(&reader);
    // result.0 Encode
    // result.1 Confidence
    // result.2 Language

    // decode file into utf-8
    let coder = encoding_from_whatwg_label(charset2encoding(&result.0));

    let utf8reader = coder
        .unwrap()
        .decode(&reader, DecoderTrap::Ignore)
        .expect("Error");

    utf8reader
}

pub fn test_parse(filepath: &Path) -> Result<(), String> {
    let file = read(filepath);

    // let parser = gcode_parser::lexer::from_str(&file);
    let tokenizer = Tokenizer::new_from_str(&file);

    let out = tokenizer.tokenize();

    match out {
        Ok((rest, _parsed)) => {
            if rest.len() > 0 {
                Err(format!(
                    "{} remaining bytes to parse: {:?}",
                    rest.len(),
                    String::from_utf8(rest.to_vec()).unwrap_or("(error)".to_string())
                ))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e.description().to_string()),
    }
}
