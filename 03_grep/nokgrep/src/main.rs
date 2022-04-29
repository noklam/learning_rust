use std::env;
use std::fs;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("{}", &config);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);


    // --snip--
}

// --snip--

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

impl fmt::Display for Config{
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Query: ({}, {})", &self.query, &self.filename)
    }
}