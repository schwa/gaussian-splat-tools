use crate::*;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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
}

impl Cli {
    pub fn main() -> Result<()> {
        let args = Cli::parse();
        match args.command {
            Some(Commands::Convert { input, output }) => {
                Cli::convert(input, output)?;
            }
            Some(Commands::GuessFormat { input }) => {
                let format = guess_format(&input).unwrap();
                println!("{:?}", format);
            }
            None => {
                println!("No command provided");
            }
        }
        Ok(())
    }

    fn convert(input: PathBuf, output: PathBuf) -> Result<()> {
        let input_format = guess_format(&input).unwrap();
        let output_format = guess_format(&output).unwrap();
        match (input_format, output_format) {
            (SplatFormats::SplatB, SplatFormats::SplatC) => {
                println!("Converting SplatB to SplatC...");
                let splats_b = SplatB::load(&input)?;
                let splats_c: Vec<SplatC> = splats_b.iter().map(SplatC::from).collect();
                SplatC::save(&splats_c, &output)?;
            }
            _ => {
                println!("Unsupported conversion");
            }
        }
        Ok(())
    }
}
