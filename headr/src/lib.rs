use clap::{Arg, Command, ArgAction};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq)]
#[deny(unused_variables)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Jirok, kakaijirok@gmail.com")
        .about("Rust head")
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
            .long("lines")
            .help("Number lines")
            .value_name("NUM")
            .default_value("10")
            .action(ArgAction::Set),
        )
        .arg(
            Arg::new("bytes")
            .short('c')
            .long("bytes")
            .help("Number bytes")
            .value_name("NUM")
            .conflicts_with("number_lines")
            .action(ArgAction::Set),
        )
        .get_matches();

    let lines = parse_positive_int(
        matches.get_one::<String>("number_lines").unwrap()
    ).map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = match matches.get_one::<String>("bytes") {
        Some(s) => Some(parse_positive_int(s).map_err(|e| format!("illegal byte count -- {}", e))?),
        None => None,
    };

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect(),
        lines: lines,
        bytes: bytes,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config)-> MyResult<()> {
    let num_files =config.files.len();
    
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(e) => eprint!("{}: {}", filename, e),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {} <==", if file_num > 0 {"\n"} else {""}, filename);
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes_read = file.read_line(&mut line)?;
                        if bytes_read == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            },
        }
    }
    Ok(())
}

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>(){
        Ok(v) if v > 0 =>Ok(v),
        _ => Err(From::from(val)),
    }
}



//-------------------- tests --------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive_int(){
        // try 3 that's positive number and results in Ok
        let res = parse_positive_int("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 3);

        // try non-numeric string that results in Err
        let res = parse_positive_int("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

        // try 0 that's not positive number and results in Err
        let res = parse_positive_int("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "0".to_string());
    }
}




