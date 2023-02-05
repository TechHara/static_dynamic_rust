use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Parser)]
#[command(name = "ntokens")]
#[command(author = "TechHara")]
#[command(version = "0.1.0")]
#[command(about = "print out number of tokens in each line", long_about = None)]
struct Cli {
    /// select delimiter
    #[arg(value_enum, long)]
    delimiter: Delimiter,
    /// select dispatch
    #[arg(value_enum, long)]
    dispatch: Dispatch,
    /// Input file
    input: Option<String>,
}

#[derive(ValueEnum, Clone, Copy)]
enum Delimiter {
    Tab,
    Whitespace,
}

#[derive(ValueEnum, Clone, Copy)]
enum Dispatch {
    Static,
    Dynamic,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let input_file = args.input.unwrap_or("/dev/stdin".to_owned());
    let ifs = BufReader::new(File::open(input_file)?);
    match args.dispatch {
        Dispatch::Static => run_static_dispatch(args.delimiter, ifs),
        Dispatch::Dynamic => run_dynamic_dispatch(args.delimiter, ifs),
    }
}

fn static_dispatcher(
    tokenize_and_count: impl Fn(String) -> usize,
    ifs: BufReader<File>,
) -> Result<()> {
    ifs.lines()
        .map(|line| tokenize_and_count(line.unwrap()))
        .for_each(move |n| {
            println!("{n}");
        });
    Ok(())
}

fn run_static_dispatch(delim: Delimiter, ifs: BufReader<File>) -> Result<()> {
    match delim {
        Delimiter::Tab => static_dispatcher(|line| line.split('\t').count(), ifs),
        Delimiter::Whitespace => static_dispatcher(|line| line.split_whitespace().count(), ifs),
    }
}

fn run_dynamic_dispatch(delim: Delimiter, ifs: BufReader<File>) -> Result<()> {
    let tokenize: Box<dyn for<'a> Fn(&'a str) -> Box<dyn Iterator<Item = &'a str> + 'a>> =
        match delim {
            Delimiter::Tab => Box::new(move |line| Box::new(line.split('\t'))),
            Delimiter::Whitespace => Box::new(move |line| Box::new(line.split_whitespace())),
        };

    ifs.lines()
        .map(|line| tokenize(&line.unwrap()).count())
        .for_each(move |n| {
            println!("{n}");
        });
    Ok(())
}
