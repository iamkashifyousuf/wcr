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

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    lines_counts: usize,
    words_counts: usize,
    bytes_counts: usize,
    chars_counts: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .about("Word Count in Rust")
        .author("Kashif Yousuf, <kashifyousuf.sc@gmail.com")
        .version("0.1.0")
        .arg(
            Arg::with_name("files")
                .help("Input Files")
                .value_name("FILE")
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

    if [lines, words, bytes, chars].iter().all(|x| !x) {
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
    let mut total_lines: usize = 0;
    let mut total_words: usize = 0;
    let mut total_bytes: usize = 0;
    let mut total_chars: usize = 0;

    for file in &config.files {
        match open(file) {
            Err(e) => eprintln!("{}: {}", file, e),
            Ok(mut reader) => {
                let file_info = count(&mut reader)?;
                total_lines += file_info.lines_counts;
                total_words += file_info.words_counts;
                total_bytes += file_info.bytes_counts;
                total_chars += file_info.chars_counts;

                println!(
                    "{}{}{}{}{}",
                    format_field(config.lines, file_info.lines_counts),
                    format_field(config.words, file_info.words_counts),
                    format_field(config.bytes, file_info.bytes_counts),
                    format_field(config.chars, file_info.chars_counts),
                    if file == "-" {
                        "".to_string()
                    } else {
                        format!(" {}", file)
                    }
                );
            }
        }
    }

    if config.files.len() != 1 {
        println!(
            "{}{}{}{} total",
            format_field(config.lines, total_lines),
            format_field(config.words, total_words),
            format_field(config.bytes, total_bytes),
            format_field(config.chars, total_chars)
        )
    }

    Ok(())
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

fn count(mut reader: impl BufRead) -> MyResult<FileInfo> {
    let mut lines_counts = 0;
    let mut words_counts = 0;
    let mut bytes_counts = 0;
    let mut chars_counts = 0;
    let mut string_buff = String::new();
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

    Ok(FileInfo {
        lines_counts,
        words_counts,
        bytes_counts,
        chars_counts,
    })
}

fn format_field(cond: bool, value: usize) -> String {
    if cond {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::format_field;

    use super::{FileInfo, count};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text_1 = "I don't want the world, I just want your half.\r\n";
        let info = count(Cursor::new(text_1));
        assert!(info.is_ok());
        assert!(info.is_ok());
        let expected = FileInfo {
            lines_counts: 1,
            words_counts: 10,
            bytes_counts: 48,
            chars_counts: 48,
        };
        assert_eq!(info.unwrap(), expected);

        let text_2 = "";
        let info = count(Cursor::new(text_2));
        assert!(info.is_ok());
        assert!(info.is_ok());
        let expected = FileInfo {
            lines_counts: 0,
            words_counts: 0,
            bytes_counts: 0,
            chars_counts: 0,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        // for True Values
        let res = format_field(true, 33);
        let expected = format!("{:>8}", 33);
        assert_eq!(res, expected);

        let res = format_field(true, 221);
        let expected = format!("{:>8}", 221);
        assert_eq!(res, expected);

        // for False Values
        let res = format_field(false, 33);
        let expected = format!("");
        assert_eq!(res, expected);
    }
}
