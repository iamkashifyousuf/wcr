# wcr 🦀

**wcr** is a simple, fast Word Count utility written in Rust, inspired by the classic Unix `wc` command. It can count **lines, words, characters, and bytes** from one or more input files, or from standard input.

---

## Version

```
wcr 0.2.0
```

Author: **Kashif Yousuf, <kashifyousuf.sc@gmail.com**

---

## Features

* Count lines, words, characters, and bytes
* Accept input from files or **stdin**
* Supports multiple input files
* Lightweight and fast (Rust-powered ⚡)
* Familiar `wc`-style CLI interface

---

## Installation

### Build from source

Make sure you have **Rust** installed.

```bash
git clone <repository-url>
cd wcr
cargo build --release
```

The binary will be available at:

```bash
target/release/wcr
```

Optionally, install it globally:

```bash
cargo install --path .
```

---

## Usage

```text
wcr [FLAGS] [Input File(s)]...
```

If no input file is provided, or if `-` is used, input is read from **stdin**.

---

## Flags

| Flag | Long Flag   | Description              |
| ---- | ----------- | ------------------------ |
| `-c` | `--bytes`   | Print byte count         |
| `-m` | `--chars`   | Print character count    |
| `-l` | `--lines`   | Print newline count      |
| `-w` | `--words`   | Print word count         |
| `-L` | `--max-line-length`   | Print the maximum display width         |
| `-h` | `--help`    | Show help information    |
| `-V` | `--version` | Show version information |

---

## Arguments

| Argument          | Description                                      |
| ----------------- | ------------------------------------------------ |
| `<Input File>...` | One or more input files (default: `-` for stdin) |

---

## Options

| Options          | Description                                      |
| ----------------- | ------------------------------------------------ |
| `--files0-from <File with Null seperator>` | Read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input |

---

## Examples

### Count words in a file

```bash
wcr -w file.txt
```

### Count lines and bytes

```bash
wcr -l -c file.txt
```

### Read from stdin

```bash
echo "Hello world" | wcr -w
```

### Multiple files

```bash
wcr -l -w file1.txt file2.txt
```

---

## Output Behavior

* When **multiple flags** are provided, counts are printed in the order specified.
* When **multiple files** are provided, each file is processed independently.
* When reading from stdin, output corresponds to standard input only.

---

## Motivation

This project was built to:

* Learn Rust I/O and ownership patterns
* Understand CLI argument parsing
* Recreate a core Unix utility in Rust

---

## License

MIT License

---

## Contributing

Pull requests and improvements are welcome. Feel free to fork and experiment 🚀

---

## License

This project is licensed under the MIT License.
