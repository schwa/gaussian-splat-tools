use crate::*;
use anyhow::Result;
use clap::{Parser, Subcommand};
use ply_rs as ply;
use rand::prelude::*;
use std::path::{Path, PathBuf};
use tabled::{settings::Style, Table};
use ply_rs::writer::{ Writer };

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Info {
        #[arg(short, long)]
        input: PathBuf,
    },
    Convert {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    GuessFormat {
        #[arg(short, long)]
        input: PathBuf,
    },
    Reduce {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long)]
        limit: usize,
    },

    Shuffle {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },

    Dump {
        #[arg(short, long)]
        input: PathBuf,
    },

    Ply2Ascii {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
}

impl Cli {
    pub fn main() -> Result<()> {
        let args = Cli::parse();
        match args.command {
            Some(Commands::Info { input }) => {
                Cli::info(input)?;
            }
            Some(Commands::Convert { input, output }) => {
                Cli::convert(input, output)?;
            }
            Some(Commands::GuessFormat { input }) => {
                Cli::guess_format(&input)?;
            }
            Some(Commands::Reduce {
                input,
                output,
                limit,
            }) => {
                Cli::reduce(input, output, limit).unwrap();
            }
            Some(Commands::Shuffle { input, output }) => {
                Cli::shuffle(input, output, 100).unwrap();
            }
            Some(Commands::Dump { input }) => {
                Cli::dump(input)?;
            }
            Some(Commands::Ply2Ascii { input, output }) => {
                Cli::ply2ascii(input, output)?;
            }
            None => {
                println!("No command provided");
            }
        }
        Ok(())
    }

    fn guess_format(input: &Path) -> Result<()> {
        let ordered_results = vec![
            (SplatFormats::SplatA, SplatA::is_format(input)),
            (SplatFormats::SplatB, SplatB::is_format(input)),
            (SplatFormats::SplatC, SplatC::is_format(input)),
        ];
        for result in &ordered_results {
            println!("{:?}", result);
        }
        let format = guess_format(input).unwrap();
        println!("{:?}", format);
        Ok(())
    }

    fn info(input: PathBuf) -> Result<()> {
        let format = guess_format(&input).unwrap();
        println!("Format: {:?}", format);
        let splats = load_splats(&input)?;
        println!("Splats: {}", splats.len());
        Ok(())
    }

    fn convert(input: PathBuf, output: PathBuf) -> Result<()> {
        let splats = load_splats(&input)?;

        if output.extension() == Some("json".as_ref()) {
            save_to_json(splats, &output)?;
        } else {
            let output_format = guess_format(&output).unwrap();
            save_splats(splats, output_format, &output)?;
        }
        Ok(())
    }

    fn reduce(input: PathBuf, output: PathBuf, limit: usize) -> Result<()> {
        let mut splats = load_splats(&input)?;
        splats.truncate(limit);
        let output_format = guess_format(&output).unwrap();
        save_splats(splats, output_format, &output)?;
        Ok(())
    }

    fn shuffle(input: PathBuf, output: PathBuf, limit: usize) -> Result<()> {
        let mut splats = load_splats(&input)?;
        splats.shuffle(&mut thread_rng());
        splats.truncate(limit);
        let output_format = guess_format(&output).unwrap();
        save_splats(splats, output_format, &output)?;
        Ok(())
    }

    fn dump(input: PathBuf) -> Result<()> {
        let splats = load_splats(&input)?;

        let table = Table::new(splats).with(Style::modern()).to_string();
        println!("{}", table);

        Ok(())
    }

    fn ply2ascii(input: PathBuf, output: PathBuf) -> Result<()> {
        // set up a reader, in this a file.
        let mut f = std::fs::File::open(input).unwrap();

        // create a parser
        let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();

        // use the parser: read the entire file
        let mut ply = p.read_ply(&mut f).unwrap();
        ply.header.encoding = ply::ply::Encoding::Ascii;

        let mut buf = Vec::<u8>::new();
        let w = Writer::new();
        let written = w.write_ply(&mut buf, &mut ply).unwrap();

        // write the buffer to a file
        std::fs::write(output, &buf).unwrap();
        Ok(())
    }
}

fn save_to_json(splats: Vec<UberSplat>, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(&splats)?;
    std::fs::write(path, json)?;
    Ok(())
}
