# Minigrep - A simple grep-like tool implemented in Rust

This simple CLI tool searches for a given pattern in a specified file and as a result, it prints all the lines that contain the pattern.

This program is a simple implementation of the `grep` command-line tool in Rust. It is an exercise present in the [Rust Programming Language book](https://doc.rust-lang.org/book/). It includes an extra feature of searching for a pattern in multiple files, which is not present in the original exercise. This feature was added to practice mutli-threading in Rust.

Working through the exercise, the developer gets introduced to handling command-line arguments, reading files, error handling, and other concepts in Rust.

## Usage

The requirement for running/building this program is having the Rust Programming language instaled. Learn how to get started on the [official website](https://www.rust-lang.org/learn/get-started).

After cloning this repository, navigate to the project's root directory and run the following command:

```bash
$ cargo run <pattern> <file path | multiple paths>
```

Where `<pattern>` is the pattern you want to search for and `<file>` is a file path for the file you want to search the contents of.

### Example

#### Single input

```bash
$ cargo run to _poem.txt

Are you nobody, too?
How dreary to be somebody!
```

#### Multiple inputs

```bash
$ cargo run to _poem.txt _funny.txt _serious.txt

[_poem.txt]:
Are you nobody, too?
How dreary to be somebody!

[_funny.txt]:
Lived a man with too many felines.
Remember this whimsical town.

[_serious.txt]:
A journey embarked, yet often untold.
Echoing stories only time can tell.
Love's gentle touch, a soothing balm,
```

#### Case insensitive search

To perform a case-insensitive search, set the `IGNORE_CASE` environment variable to any value.

```bash
$ IGNORE_CASE=1 cargo run to _poem.txt

Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
