mod create;
mod build;

use std::env::{ args };

enum CommandType {
    Scaffold,
    Build
}

fn main () {
    start_process();
}

fn start_process() {
    match build_type() {
        CommandType::Scaffold => create::run(),
        CommandType::Build => build::run()
    }
}

fn build_type() -> CommandType {
    let args = args().any(|p| p == "build");

    match args {
        true => CommandType::Build,
        false => CommandType::Scaffold
    }
}

