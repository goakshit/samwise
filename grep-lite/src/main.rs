use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1.0")
        .about("searches for the given pattern")
        .arg(Arg::with_name("pattern")
            .help("the pattern to search for")
            .required(true)
            .index(1)
        ).get_matches();
    
        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap();

    let quote = "It is the same with books. What do we seek through millions of pages?
It is the same with books";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
