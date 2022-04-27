use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Keywords: {}", query);
    println!("In file {}", filename);
    println!("Contents: {}", contents)
}
