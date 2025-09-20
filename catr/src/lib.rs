use std::error::Error;
use clap::{Arg, Command, ArgAction};


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}


type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Jirok, kakaijirok@gmail.com")
        .about("Rust cat")
        .arg(
            Arg::new("files")
            .value_name("FILE")
            .help("Input file(s)")
            .required(true)
            .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
            .short('n')
            .help("Number lines")
            .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank_lines")
            .short('b')
            .help("Number nonblank lines")
            .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        files: matches.get_many::<String>("files").unwrap_or_default().map(|s| s.to_string()).collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })

}