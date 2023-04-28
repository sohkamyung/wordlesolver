// Program to display possible Wordle Words

use clap::Parser;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

mod wordle;
use wordle::WordleWords;

mod tui;

// Structure for our command line arguments
/// Program to display possible Wordle Words
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File that contains the list of possible wordle words
    #[arg(default_value = "wordle.list")]
    filename: PathBuf,

    /// Use a Text UI (TUI). This will ignore other options
    #[arg(short, long)]
    tui: bool,

    /// Exclude words with these letters
    #[arg(short, long)]
    exclude: Option<String>,

    /// Letters in correct position. '.' for those not yet known
    #[arg(short, long)]
    correct: Option<String>,

    /// Letters in incorrect positions. '.' for those not yet known. Format in "xxxxx yyyyy zzzzz ....." format
    #[arg(short, long)]
    incorrect: Option<String>,
}

fn main() {
    let args = Args::parse();

    // println!("path is {}", args.filename.display());

    // read the contents of the filename line by line
    let lines = BufReader::new(File::open(args.filename).unwrap()).lines();

    let mut v = Vec::new();
    for line in lines {
        v.push(line.unwrap());
    }

    let mut possible_list = WordleWords::new(v);

    if args.tui {
        let mut tui = tui::Tui::new(possible_list);

        tui.start();
    } else {
        match args.exclude {
            Some(x) => possible_list.remove_letters(&x),
            _ => (),
        }

        match args.correct {
            Some(x) => possible_list.correct_letters(&x),
            _ => (),
        }

        match args.incorrect {
            Some(x) => {
                // println!("String {}", x);
                let my_array: Vec<&str> = x.as_str().split(" ").collect();
                // println!("my_array {:?}", my_array);
                for word in my_array {
                    // println!("Val {}", word);
                    possible_list.incorrect_letters(&word);
                }
            }
            _ => (),
        }

        // println!("{:?}", possible_list.get_word_list());
        for line in possible_list.get_word_list() {
            println!("{}", line);
        }
    }
}
