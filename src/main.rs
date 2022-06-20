use std::error::Error;
use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::csv::CsvTransformation;
use crate::structures::entry::Entry;

mod structures;
mod loading;
mod csv;

pub trait Transformation<E> {
    fn transform(input: Vec<Entry>) -> Result<(Vec<u8>, String), E>;
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_parser)]
    input_folder: PathBuf,

    #[clap(long, value_parser)]
    output_folder: PathBuf,

    #[clap(subcommand)]
    transformation: Transformations,
}

#[derive(Subcommand)]
enum Transformations {
    ToCsv,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    let entries = Entry::load_entries(&args.input_folder)?;
    let (value, extension) = match args.transformation {
        Transformations::ToCsv => CsvTransformation::transform(entries)?,
    };

    let mut output_file = args.output_folder.clone();
    output_file.push(format!("output.{}", extension));
    let output_file = output_file;

    if let Err(_e) = fs::write(&output_file, value.clone()) {
        fs::create_dir_all(args.output_folder)?;
        fs::write(&output_file, value.clone())?;
    }

    Ok(())
}
