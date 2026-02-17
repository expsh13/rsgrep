use std::{env::args, fs::read_to_string};
use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: String,
}

impl GrepArgs {
    fn new(pattern: String, path: String) -> Self {
        Self { pattern, path }
    }

    fn print_pattern(self, user: String) {
        let pat = self.pattern;
        println!("{}: {}", user, pat);
    }
}

fn main() {
    run(GrepArgs::from_args());
}

fn grep(state: &GrepArgs, content: String) {
    for line in content.lines() {
        if line.contains(state.pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(state: GrepArgs) {
    match read_to_string(&state.path) {
        Ok(content) => grep(&state, content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
