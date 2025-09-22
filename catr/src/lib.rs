use std::error::Error;
use clap::{Arg, Command, ArgAction};
use std::fs::File;
use std::io::{self, BufRead, BufReader};


#[derive(Debug, Clone, PartialEq)]
#[deny(unused_variables)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}


type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("Failed to open {filename}: {e}"),
            Ok(mut reader) => {
                let mut line = String::new();
                let mut line_num = 1;
                
                loop {
                    let bytes_read = reader.read_line(&mut line)?;
                    if bytes_read == 0 {
                        break; // reached EOF
                    }
                    
                    // -n option: number all lines
                    if config.number_lines {
                        print!("{:>6}\t{}", line_num, line);
                        line_num += 1;
                    }
                    // -b option: number nonblank lines
                    else if config.number_nonblank_lines {
                        if line.trim().is_empty() {
                            print!("{}", line); // blank line has no line number
                        } else {
                            print!("{:>6}\t{}", line_num, line);
                            line_num += 1;
                        }
                    }
                    // no option: display lines without line numbers
                    else {
                        print!("{}", line);
                    }
                    
                    line.clear(); // clear for next line
                }
            }
        }
    }
    // dbg!(config);
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
            .num_args(0..)
            .default_value("-"),
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
        files: matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })

}