use std::{io, process};

use clap::Parser;
use rand::prelude::IndexedRandom;
use squirrel_rng::SquirrelRng;

static MALE_NAMES: &str = include_str!("../resource/male-names.txt");

static FEMALE_NAMES: &str = include_str!("../resource/female-names.txt");

static SURNAMES: &str = include_str!("../resource/surnames.txt");

/// A command that prints random names.
#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Male(InnerCommand),
    Female(InnerCommand),
    Surname(InnerCommand),
}

#[derive(Debug, Parser)]
struct InnerCommand {
    #[arg(default_value_t = 1)]
    count: usize,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: Args) -> io::Result<()> {
    match args.command {
        Command::Male(InnerCommand { count }) => list_male_names(count),
        Command::Female(InnerCommand { count }) => list_female_names(count),
        Command::Surname(InnerCommand { count }) => list_surnames(count),
    }

    Ok(())
}

fn list_male_names(count: usize) {
    list_random_names(MALE_NAMES, count);
}

fn list_female_names(count: usize) {
    list_random_names(FEMALE_NAMES, count);
}

fn list_surnames(count: usize) {
    list_random_names(SURNAMES, count);
}

fn list_random_names(source: &str, count: usize) {
    let names: Vec<_> = source.lines().filter(|s| is_name(s)).collect();
    for name in names.choose_multiple(&mut SquirrelRng::new(), count.max(1)) {
        println!("{name}");
    }
}

#[inline]
fn is_name(s: &str) -> bool {
    let s = s.trim();
    !s.is_empty() && !s.starts_with('#')
}
