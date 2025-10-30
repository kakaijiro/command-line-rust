use clap::{Arg, Command, ArgAction};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Jirok, kakaijirok@gmail.com")
        .about("Rust wc")
        .arg(
            Arg::new("files")
            .value_name("FILE")
            .help("Input file(s)")
            .num_args(0..)
            .default_value("-"),
        )
        .arg(
            Arg::new("lines")
            .short('l')
            .long("lines")
            .help("Show line count")
            .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
            .short('w')
            .long("words")
            .help("Show word count")
            .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bytes")
            .short('c')
            .long("bytes")
            .help("Show byte count")
            .action(ArgAction::SetTrue)
            .conflicts_with("chars"),
        )
        .arg(
            Arg::new("chars")
            .short('m')
            .long("chars")
            .help("Show char count")
            .action(ArgAction::SetTrue)
            .conflicts_with("bytes"),
        )
        .get_matches();
    
    let mut lines =matches.get_flag("lines");
    let mut words =matches.get_flag("words");
    let mut bytes =matches.get_flag("bytes");
    let chars =matches.get_flag("chars");
    if [lines, words, bytes, chars].iter().all(|x| x == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(
        FileInfo {
            num_lines,
            num_words,
            num_bytes,
            num_chars,
        }
    )   
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let info = count(file)?;
                println!("{:>8}{:>8}{:>8}{:>8} {}", info.num_lines, info.num_words, info.num_bytes, info.num_chars, filename);
            }
        }
    }
    Ok(())
}

// -------------------- tests --------------------
#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        // let text = "";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}