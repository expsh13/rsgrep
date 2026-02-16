use std::{env::args, fs::read_to_string};

struct GrepArgs {
    pattern: String,
    path: String,
}

impl GrepArgs {
    fn new(pattern: String, path: String) -> Self {
        Self { pattern, path }
    }
}

fn main() {
    let pattern = args().nth(1);
    let path = args().nth(2);

    if let (Some(pattern), Some(path)) = (pattern, path) {
        run(path, pattern);
    }
}

// fn run_cat(path: String) {
//     match read_to_string(path) {
//         Ok(content) => print!("{}", content),
//         Err(e) => eprintln!("Error reading file: {}", e),
//     }
// }

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(path: String, pattern: String) {
    match read_to_string(path) {
        Ok(content) => grep(content, pattern),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
