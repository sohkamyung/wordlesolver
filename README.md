# wordlesolver
Display possible wordle words based on provided clues

## Introduction

This is my first attempt as a Rust program. Treat me gently. :-)

## Purpose

The program prints out a list of possible
[Wordle](https://en.wikipedia.org/wiki/Wordle) words, based on:

- the initial list of words
- a list of letters that are know to be not present
- a list of letters that is present and in the correct location
- more than one list of letters that is present but in the wrong
  location

Use the standard `cargo` based commands to build, test, produce
documentation and run the program.

The code uses the [clap](https://docs.rs/clap/latest/clap/) crate for
command line processing.

```
wordlesolver -h
Program to display possible Wordle Words

Usage: wordlesolver.exe [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]  File that contains the list of possible wordle words [default: wordle.list]

Options:
  -t, --tui                    Use a Text UI (TUI). This will ignore other options
  -e, --exclude <EXCLUDE>      Exclude words with these letters
  -c, --correct <CORRECT>      Letters in correct position. '.' for those not yet known
  -i, --incorrect <INCORRECT>  Letters in incorrect positions. '.' for those not yet known. Format in "xxxxx yyyyy zzzzz ....." format
  -h, --help                   Print help
  -V, --version                Print version
```

## External Requirement

By default, the program expects a `wordle.list` file in the current
subdirectory. This is a simple line terminated list of possible wordle
words. You can get this from multiple sources.

## Examples:

`cargo run -- -h`

Displays the help page.

`cargo run -- -t`

This will launch a Text UI (using the
[Cursive](https://docs.rs/cursive/latest/cursive/) crate). Navigate
the UI components to enter letters that are excluded, included and
incorrect. Selecting the 'Update' button will then update the possible
list of Wordle words based on the current selection of letters.

`cargo run -- -e "steamginml" -c "b..o." -i "d.... ..e.."`

This will print out a list of wordle words that do not contain the
letters "steamginml" and has 'b' and 'o' in the correct locations, and
has 'd' and 'e', but in the wrong locations.

For `-i`, the options should be included as "xxxxx xxxxx xxxxx ...",
where xxxxx can be a letter or a '.' (dot).
