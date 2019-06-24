use super::super::core;

pub fn from_file(fname: &str) {
    core::lox::Lox::run_file(fname);
}
