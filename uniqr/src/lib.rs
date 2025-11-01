use clap::{Arg, Command, ArgAction};
// use std::fs::File;
// use std::io::{self, BufRead, BufReader};
use std::error::Error;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("Jirok, kakaijirok@gmail.com")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
            .value_name("IN_FILE")
            .help("Input file")
            // .required(true)
            .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
            .value_name("OUT_FILE")
            .help("Output file")
            // .required(false)
            // .default_value("-"),
        )
        .arg(
            Arg::new("count")
            .short('c')
            .long("count")
            .help("Count occurrences")
            .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.get_one::<String>("in_file").unwrap().to_string(),
        out_file: matches.get_one::<String>("out_file").map(|s| s.to_string()),
        count: matches.get_flag("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}