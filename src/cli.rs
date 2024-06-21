use crate::*;
use anyhow::Result;
use clap::{Parser, Subcommand};
use humansize::{format_size, DECIMAL};
use nalgebra::Vector3;
use ply_rs as ply;
use ply_rs::writer::Writer;
use rand::prelude::*;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
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
    /// List the formats supported by this tool
    Formats {},

    /// Get info about a gaussian splat file
    Info {
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Convert a gaussian splat file to another format
    Convert {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },

    /// Guess the format of a gaussian splat file
    GuessFormat {
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Reduce the number of splats in a gaussian splat file
    Reduce {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long)]
        limit: usize,
    },

    /// Shuffle the splats in a gaussian splat file
    Shuffle {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },

    Center {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },

    /// Dump the splats in a gaussian splat file
    Dump {
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Convert a ply file to ascii
    PlyToAscii {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },

    /// Dump a ply file
    DumpPly {
        #[arg(short, long)]
        input: PathBuf,
    },
}

impl Cli {
    pub fn main() -> Result<()> {
        let args = Cli::parse();
        match args.command {
            Some(Commands::Formats {}) => {
                println!("Supported formats:");
                for format in SplatFormats::iter() {
                    println!("{:?}: {}", format, format.description());
                }
            }
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
                Cli::shuffle(input, output).unwrap();
            }
            Some(Commands::Center { input, output }) => {
                modify_splats(&input, &output, |splats| {
                    center_splats(splats);
                })?;
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
        println!("Format: {:?} / {}", format, format.description());
        // print file size
        let metadata = std::fs::metadata(&input)?;
        let size = metadata.len();
        println!("Size: {}", format_size(size, DECIMAL));
        let splats = load_splats(&input)?;
        println!("# Splats: {}", splats.len());

        let positions = splats
            .iter()
            .map(|splat| splat.position)
            .collect::<Vec<Vector3<f32>>>();
        let mut min_position = positions[0];
        let mut max_position = positions[0];
        let mut sum_position = Vector3::new(0.0, 0.0, 0.0);

        for position in positions {
            min_position = Vector3::new(
                min_position.x.min(position.x),
                min_position.y.min(position.y),
                min_position.z.min(position.z),
            );
            max_position = Vector3::new(
                max_position.x.max(position.x),
                max_position.y.max(position.y),
                max_position.z.max(position.z),
            );
            sum_position += position;
        }

        let avg_position = sum_position / splats.len() as f32;
        println!("Min position: {:?}", min_position);
        println!("Max position: {:?}", max_position);
        println!("Avg position: {:?}", avg_position);

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

    fn shuffle(input: PathBuf, output: PathBuf) -> Result<()> {
        modify_splats(&input, &output, |splats| {
            let mut rng = thread_rng();
            splats.shuffle(&mut rng);
        })
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

fn modify_splats(input: &Path, output: &Path, closure: impl Fn(&mut Vec<UberSplat>)) -> Result<()> {
    let mut splats = load_splats(input)?;
    closure(&mut splats);
    let format = guess_format(output).unwrap();
    save_splats(splats, format, output)?;
    Ok(())
}
