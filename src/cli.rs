use crate::*;
use anyhow::Result;
use clap::{Parser, Subcommand};
use ply_rs as ply;
use ply_rs::writer::Writer;
use rand::prelude::*;
use std::path::{Path, PathBuf};
use tabled::builder::Builder;
use tabled::{settings::Style, Table};

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

    PlyToAscii {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },

    DumpPly {
        #[arg(short, long)]
        input: PathBuf,
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
            Some(Commands::PlyToAscii { input, output }) => {
                Cli::ply_to_ascii(input, output)?;
            }
            Some(Commands::DumpPly { input }) => {
                Cli::dump_ply(input)?;
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

    fn ply_to_ascii(input: PathBuf, output: PathBuf) -> Result<()> {
        let mut f = std::fs::File::open(input).unwrap();
        let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();
        let mut ply = p.read_ply(&mut f).unwrap();
        ply.header.encoding = ply::ply::Encoding::Ascii;
        let mut buf = Vec::<u8>::new();
        let w = Writer::new();
        w.write_ply(&mut buf, &mut ply).unwrap();
        std::fs::write(output, &buf).unwrap();
        Ok(())
    }

    fn dump_ply(input: PathBuf) -> Result<()> {
        let mut f = std::fs::File::open(input).unwrap();
        let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();
        let ply = p.read_ply(&mut f).unwrap();

        for (element, element_def) in &ply.header.elements {
            println!("{}", element);

            let property_names = element_def
                .properties
                .iter()
                .map(|p| p.0.clone())
                .collect::<Vec<String>>();

            let mut builder = Builder::default();
            builder.push_record(property_names);

            for properties in ply.payload.get(element).unwrap() {
                let values: Vec<String> = element_def
                    .properties
                    .iter()
                    .map(|(property, _)| {
                        let property = properties.get(property).unwrap();
                        match property {
                            ply::ply::Property::Char(v) => format!("{}", v),
                            ply::ply::Property::UChar(v) => format!("{}", v),
                            ply::ply::Property::Short(v) => format!("{}", v),
                            ply::ply::Property::UShort(v) => format!("{}", v),
                            ply::ply::Property::Int(v) => format!("{}", v),
                            ply::ply::Property::UInt(v) => format!("{}", v),
                            ply::ply::Property::Float(v) => format!("{}", v),
                            ply::ply::Property::Double(v) => format!("{}", v),
                            ply::ply::Property::ListFloat(v) => format!("{:?}", v),
                            ply::ply::Property::ListDouble(v) => format!("{:?}", v),
                            ply::ply::Property::ListChar(v) => format!("{:?}", v),
                            ply::ply::Property::ListUChar(v) => format!("{:?}", v),
                            ply::ply::Property::ListShort(v) => format!("{:?}", v),
                            ply::ply::Property::ListUShort(v) => format!("{:?}", v),
                            ply::ply::Property::ListInt(v) => format!("{:?}", v),
                            ply::ply::Property::ListUInt(v) => format!("{:?}", v),
                        }
                    })
                    .collect();
                builder.push_record(&values);
            }
            // println!("{}", table);

            // for (property, _) in &element_def.properties {

            //     let values: Vec<String> = ply.payload.get(element).unwrap().iter().map(|payload| {
            //         let value = format!("{:?}", payload.get(property).unwrap());
            //         value
            //     }).collect();
            //     // builder.push_record(&values);

            //     println!("{} {:?}", values.len(), values);

            // }

            let mut table = builder.build();
            table.with(Style::rounded());
            println!("{}", table);
        }

        Ok(())
    }
}

fn save_to_json(splats: Vec<UberSplat>, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(&splats)?;
    std::fs::write(path, json)?;
    Ok(())
}
