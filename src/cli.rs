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
    Reduce {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long)]
        limit: usize,
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
            Some(Commands::Reduce { input, output, limit }) => {
                Cli::reduce(input, output, limit).unwrap();
            }
            None => {
                println!("No command provided");
            }
        }
        Ok(())
    }

    fn convert(input: PathBuf, output: PathBuf) -> Result<()> {
        let Some(input_format) = guess_format(&input) else {
            println!("Could not guess input format");
            return Ok(());
        };
        let Some(output_format) = guess_format(&output) else {
            println!("Could not guess output format");
            return Ok(());
        };
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

    fn reduce(input: PathBuf, output: PathBuf, limit: usize) -> Result<()>{
        let Some(input_format) = guess_format(&input) else {
            println!("Could not guess input format");
            return Ok(());
        };
        let Some(output_format) = guess_format(&output) else {
            println!("Could not guess output format");
            return Ok(());
        };
        match (input_format, output_format) {
            (SplatFormats::SplatB, SplatFormats::SplatC) => {
                let splats_b: Vec<SplatB> = SplatB::load(&input)?.into_iter().take(limit).collect();
                let splats_c: Vec<SplatC> = splats_b.iter().map(SplatC::from).collect();
                SplatC::save(&splats_c, &output)?;
            }
            (SplatFormats::SplatB, SplatFormats::SplatB) => {
                let splats: Vec<SplatB> = SplatB::load(&input)?.into_iter().take(limit).collect();
                SplatB::save(&splats, &output)?;
            }
            (SplatFormats::SplatC, SplatFormats::SplatC) => {
                let splats: Vec<SplatC> = SplatC::load(&input)?.into_iter().take(limit).collect();
                SplatC::save(&splats, &output)?;
            }
            _ => {
                println!("Unsupported conversion");
            }
        }
        Ok(())
    }
}
