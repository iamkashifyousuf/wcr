use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::io::stdin;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .about("Word Count in Rust")
        .author("Kashif Yousuf")
        .version("0.1.0")
        .arg(
            Arg::with_name("files")
                .help("Input Files")
                .value_name("Input File")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .help("print the newline counts")
                .short("l")
                .long("lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .help("print the words counts")
                .short("w")
                .long("words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .help("print the bytes counts")
                .short("c")
                .long("bytes")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .help("print the chars counts")
                .short("m")
                .long("chars")
                .takes_value(false),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|x| x == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);

    for file in config.files {
        match open(&file) {
            Err(e) => eprintln!("{}: {}", file, e),
            Ok(mut reader) => {
                // Req - line, words, bytes, char counts
                let mut lines_counts: usize = 0;
                let mut words_counts: usize = 0;
                let mut bytes_counts: usize = 0;
                let mut chars_counts: usize = 0;

                let mut string_buff = String::new();
                // Here Loop Starts
                loop {
                    let bytes_read = reader.read_line(&mut string_buff)?;
                    if bytes_read == 0 {
                        break;
                    }
                    lines_counts += 1;
                    words_counts += string_buff.split_ascii_whitespace().count();
                    bytes_counts += bytes_read;
                    chars_counts += string_buff.chars().count();

                    string_buff.clear();
                }

                println!(
                    "{} {} {} {} {}",
                    if config.lines {
                        format!("{}", lines_counts)
                    } else {
                        "".to_string()
                    },
                    if config.words {
                        format!("{}", words_counts)
                    } else {
                        "".to_string()
                    },
                    if config.bytes {
                        format!("{}", bytes_counts)
                    } else {
                        "".to_string()
                    },
                    if config.chars {
                        format!("{}", chars_counts)
                    } else {
                        "".to_string()
                    },
                    file
                );
                // println!("lines: {}, Words: {}, Bytes: {}, Chars: {}", lines_counts, words_counts, bytes_counts, chars_counts);
                // println!("{}", string_buff);
            }
        }
    }

    Ok(())
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
