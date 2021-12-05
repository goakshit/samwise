use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1.0")
        .about("searches for the given pattern")
        .arg(Arg::with_name("pattern")
            .help("the pattern to search for")
            .takes_value(true)
            .required(true)
        ).
        arg(Arg::with_name("input")
            .help("the input to search from")
            .takes_value(true)
            .required(false)
        ).get_matches();
    
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    
    let input = args.value_of("input").unwrap_or("stdin");
    if input == "stdin" {
      let stdin = io::stdin();
      let reader = stdin.lock();
      process_lines(reader, re);
    } else {
      let f = File::open(input).unwrap();
      let reader = BufReader::new(f);
      process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
  for line in reader.lines() {
    let line_ = line.unwrap();
      match re.find(&line_) {
        Some(_) => println!("{}", line_),
        None => (),
      }
  }
}
