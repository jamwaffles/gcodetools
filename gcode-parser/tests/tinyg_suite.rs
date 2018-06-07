extern crate chardet;
extern crate encoding;
extern crate gcode_parser;
extern crate nom;

mod helpers;

use helpers::*;
use std::path::Path;

#[test]
fn profile_tinyg_samples() {
    let dir = Path::new("./tests/test_files/tinyg")
        .canonicalize()
        .unwrap();

    let files = collect_source_files(&dir).expect("Could not get list of files");

    let results = files.iter().map(|fpath| test_parse(fpath));

    let mut num_errors = 0;

    start_profile();

    for (file, result) in files.iter().zip(results) {
        if result.is_err() {
            num_errors += 1;
        }

        println!(
            "{0: <50} {1:?}",
            file.file_name().unwrap().to_str().unwrap(),
            result
        );
    }

    end_profile();

    println!(
        "\n{} out of {} files passed ({} failed)\n",
        (files.len() - num_errors),
        files.len(),
        num_errors
    );

    assert_eq!(num_errors, 0, "Not all files parsed successfully");
}
