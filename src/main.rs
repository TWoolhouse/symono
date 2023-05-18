use std::path::PathBuf;
use thiserror::Error;

fn main() -> Result<(), Error> {
    let cli = cli().get_matches();
    transpile::entry(&cli)
}

mod transpile {
    use super::Error;
    use std::{fs, path::PathBuf};
    use symono::Transpile;

    pub fn entry(command: &clap::ArgMatches) -> Result<(), Error> {
        let input = fs::read_to_string(command.get_one::<PathBuf>("ifile").expect("required"))?;

        match command.get_one::<Language>("lang").expect("default") {
            Language::Latex => symono::parse(&input)
                .map_err(Into::into)
                .map(|sym| sym.latex()),
        }
        .map(|str| {
            print!("{}", str);
        })
    }

    #[derive(Debug, Clone, clap::ValueEnum)]
    pub enum Language {
        Latex,
    }
}

fn cli() -> clap::Command {
    use clap::*;
    command!()
        .arg(
            Arg::new("lang")
                .required(true)
                .action(ArgAction::Set)
                .value_parser(value_parser!(transpile::Language)),
        )
        .arg(
            Arg::new("ifile")
                .required(true)
                .action(ArgAction::Set)
                .value_parser(value_parser!(PathBuf)),
        )
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Input can not be parsed: {0}")]
    Parse(#[from] pest::error::Error<symono::Rule>),
}
