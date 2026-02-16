use std::{
    fs::{File, read_to_string},
    io::Write,
};

fn main() {
    run_cat();
}

fn run_cat() -> std::io::Result<()> {
    let path = "src/main.rs";
    // match read_to_string(path) {
    //     Ok(contents) => println!("{}", contents),
    //     Err(error) => println!("Error reading file: {}", error),
    // }
    let mut file = File::create(path)?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
