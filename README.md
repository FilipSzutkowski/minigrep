# Minigrep - A simple grep-like tool implemented in Rust

This simple CLI tool searches for a given pattern in a specified file and as a result, it prints all the lines that contain the pattern.

This program is a simple implementation of the `grep` command-line tool in Rust. It is an exercise present in the [Rust Programming Language book](https://doc.rust-lang.org/book/).

Working through the exercise, the developer gets introduced to handling command-line arguments, reading files, error handling, and other concepts in Rust.

## Usage

The requirement for running/building this program is having the Rust Programming language instaled. Learn how to get started on the [official website](https://www.rust-lang.org/learn/get-started).

After cloning this repository, navigate to the project's root directory and run the following command:

```bash
$ cargo run <pattern> <file>
```

Where `<pattern>` is the pattern you want to search for and `<file>` is a file path for the file you want to search the contents of.

### Example

```bash
$ cargo run to poem.txt

Are you nobody, too?
How dreary to be somebody!
```

#### Case insensitive search

To perform a case-insensitive search, set the `IGNORE_CASE` environment variable to any value.

```bash
$ IGNORE_CASE=1 cargo run to poem.txt

Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
