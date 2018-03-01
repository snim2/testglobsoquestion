#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate glob;
#[macro_use]
extern crate lazy_static;

use glob::glob;

lazy_static! {
    /// Glob all example files in the `tests/` directory.
    static ref TEST_FILES: Vec<String> = glob("tests/*.java")
        .expect("Failed to read glob pattern")
        .into_iter()
        .map(|res| res.unwrap().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
}

fn check_files(path1: &str, path2: &str, msg: &str) {
    assert!(true, "FAILURE: {}: {} and {}.", msg, path1, path2);
}

#[test]
fn test_glob_runner() {
    // Define unit tests for a single pair of filenames.
    macro_rules! define_tests {
        ($name1:tt, $name2:tt, $fname1:expr, $fname2:expr) => ( interpolate_idents! {
            #[test]
            fn [test_globbed_ $name1 _ $name2 _null]() {
                check_files($fname1, $fname2, "null test");
            }
            #[test]
            fn [test_globbed_ $name1 _ $name2 _non_null]() {
                check_files($fname1, $fname2, "non-null test");
            }
        } )
    }
    // Write out unit tests for all pairs of given list of filenames.
    macro_rules! test_globbed_files {
        ($d:expr) => {
            for fname1 in $d.iter() {
                for fname2 in $d.iter() {
                    // Remove directory and extension from `fname1`, `fname2`.
                    let name1 = fname1.get(6..).unwrap().split(".").collect::<Vec<&str>>()[0];
                    let name2 = fname2.get(6..).unwrap().split(".").collect::<Vec<&str>>()[0];
                    || { define_tests!(name1, name2, fname1, fname2) };
                }
            }
        }
    }
    // Test all pairs of files in the `tests/` directory.
    test_globbed_files!(TEST_FILES);
}
