use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader};
use unicode_width::UnicodeWidthChar;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
    max_line_length: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    lines_counts: usize,
    words_counts: usize,
    bytes_counts: usize,
    chars_counts: usize,
    max_line_length: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .about("Word Count in Rust")
        .author("Kashif Yousuf, <kashifyousuf.sc@gmail.com")
        .version("0.3.0")
        .arg(
            Arg::with_name("files")
                .help("Input Files")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("from_files")
                .help("read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input")
                .long("files0-from")
                .takes_value(true)
                .conflicts_with("files")
                .value_name("File with Null seperator")
                .multiple(false)
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
        .arg(
            Arg::with_name("max_line_length")
                .help("print the maximum display width")
                .short("L")
                .long("max-line-length")
                .takes_value(false),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");
    let max_line_length = matches.is_present("max_line_length");

    if [lines, words, bytes, chars, max_line_length]
        .iter()
        .all(|x| !x)
    {
        lines = true;
        words = true;
        bytes = true;
    }

    let files = if matches.is_present("from_files") {
        let file_list = matches.value_of_lossy("from_files").unwrap();
        let handle = open(&file_list).map_err(|e| format!("{}: {}", file_list, e))?;
        let files = read_null_separated(handle)?;
        files
    } else {
        matches.values_of_lossy("files").unwrap()
    };

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
        max_line_length,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines: usize = 0;
    let mut total_words: usize = 0;
    let mut total_bytes: usize = 0;
    let mut total_chars: usize = 0;
    let mut total_max_line_length: usize = 0;

    for file in &config.files {
        match open(file) {
            Err(e) => eprintln!("{}: {}", file, e),
            Ok(mut reader) => {
                let file_info = count(&mut reader)?;
                total_lines += file_info.lines_counts;
                total_words += file_info.words_counts;
                total_bytes += file_info.bytes_counts;
                total_chars += file_info.chars_counts;
                total_max_line_length += file_info.max_line_length;

                println!(
                    "{}{}{}{}{}{}",
                    format_field(config.lines, file_info.lines_counts),
                    format_field(config.words, file_info.words_counts),
                    format_field(config.bytes, file_info.bytes_counts),
                    format_field(config.chars, file_info.chars_counts),
                    format_field(config.max_line_length, file_info.max_line_length),
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
            "{}{}{}{}{} total",
            format_field(config.lines, total_lines),
            format_field(config.words, total_words),
            format_field(config.bytes, total_bytes),
            format_field(config.chars, total_chars),
            format_field(config.max_line_length, total_max_line_length),
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

fn read_null_separated(mut handle: impl BufRead) -> MyResult<Vec<String>> {
    let mut files = vec![];

    loop {
        let mut bytes_buff = Vec::new();
        let bytes_read = handle.read_until(b'\0', &mut bytes_buff)?;
        if bytes_read == 0 {
            break;
        }
        if bytes_buff.last() == Some(&0) {
            bytes_buff.pop();
        }
        files.push(String::from_utf8(bytes_buff)?);
    }

    Ok(files)
}

fn count(mut reader: impl BufRead) -> MyResult<FileInfo> {
    let mut string_buff = String::new();
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;
    let mut chars = 0;
    let mut max_line_len = 0;

    loop {
        let bytes_read = reader.read_line(&mut string_buff)?;
        if bytes_read == 0 {
            break;
        }
        lines += 1;
        words += string_buff.split_ascii_whitespace().count();
        bytes += bytes_read;
        chars += string_buff.chars().count();
        max_line_len = max_line_len.max(compute_line_len(&string_buff));
        string_buff.clear();
    }

    Ok(FileInfo {
        lines_counts: lines,
        words_counts: words,
        bytes_counts: bytes,
        chars_counts: chars,
        max_line_length: max_line_len,
    })
}

fn format_field(cond: bool, value: usize) -> String {
    if cond {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

fn compute_line_len(line: &str) -> usize {
    let tab_width: usize = 8;
    let mut current_max_line_len = 0;
    for ch in line.chars() {
        match ch {
            '\t' => {
                let next_tab = tab_width - (current_max_line_len % tab_width);
                current_max_line_len += next_tab;
            }
            '\n' | '\r' => {
                break;
            }
            _ => {
                current_max_line_len += ch.width().unwrap_or(0);
            }
        }
    }
    current_max_line_len
}

#[cfg(test)]
mod tests {
    use crate::{compute_line_len, format_field};

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
            max_line_length: 46,
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
            max_line_length: 0,
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

    #[test]
    fn test_compute_line_len() {
        let line = "I don't want the world, I just want your half.\r\n";
        assert_eq!(compute_line_len(line), 46);

        let line = "Two.";
        assert_eq!(compute_line_len(line), 4);

        let line = "";
        assert_eq!(compute_line_len(line), 0);
    }
}
