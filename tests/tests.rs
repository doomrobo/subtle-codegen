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
    let mut compiler_flags = String::new();

    for p in dyld_paths.split(":") {
        compiler_flags += &*format!(" -L {}", p);
    }

    config.target_rustcflags = Some(compiler_flags);

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}
