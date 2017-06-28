extern crate compiletest_rs as compiletest;

use std::env;
use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::default_config();
    let cfg_mode = mode.parse().ok().expect("Invalid mode");

    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));

    // Add our dependencies to the library search path
    let dyld_paths = env::var("DYLD_LIBRARY_PATH").unwrap();

    // This is hacky. We can only add one search path, so pick the one that has ends with "/deps"
    for path in dyld_paths.split(":").map(PathBuf::from) {
        if path.ends_with("deps") {
            config.build_base = path;
        }
    }

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}
