extern crate compiletest_rs as compiletest;

use std::path::PathBuf;
use std::env;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();
    let mut base_path = env::current_dir().expect("Current directory is invalid");
    base_path.pop();
    let debug_path = base_path.join("target").join("debug");
    let deps_path = debug_path.join("deps");

    config.target_rustcflags = Some(format!(
        "-L {} -L {}",
        debug_path.display(),
        deps_path.display()
    ));
    config.mode = mode.parse().ok().expect("Invalid mode");
    config.src_base = PathBuf::from("tests").join(mode);

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
    run_mode("run-pass");
}
