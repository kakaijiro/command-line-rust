use clap::{Command, Arg, ArgAction};  // Command is the equivalent of App in clap 4.x

fn main() {
    let matches = Command::new("echor")
    .version("0.1.0")
    .author("Jirok <kakaijirok@gmail.com>")
    .about("Rust echo")
    .arg(
        Arg::new("text")
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .num_args(1..),
    )
    .arg(
        Arg::new("omit_newline")
        .short('n')
        .help("Do not print newline")
        .action(ArgAction::SetTrue)
    )
    .get_matches();

    println!("{:#?}", matches)

    // // get values from arguments
    // let text_values: Vec<&String> = matches.get_many::<String>("text")
    //     .unwrap_or_default()
    //     .collect();
    
    // // if -n option is specified
    // let omit_newline = matches.get_flag("omit_newline");
    
    // // join text values
    // let output = text_values.join(" ");
    
    // if omit_newline {
    //     print!("{}", output);
    // } else {
    //     println!("{}", output);
    // }
}