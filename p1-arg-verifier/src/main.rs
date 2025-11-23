// https://github.com/badprog/badprog-grcov-demo

// use
use p1_arg_verifier::program::{self, utils::MessageOutput};

use std::env;

// ============================================================================
// main
fn main() {
    let args: Vec<std::string::String> = env::args().collect();
    let message = program::start::start(&args);

    match message {
        MessageOutput::Success(state) => {
            println!("{state}");
        }
        MessageOutput::Error(state) => {
            eprintln!("{state}");
        }
    }
}
