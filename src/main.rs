use clap::Parser;
use ron::de;
use ron::ser::{self, PrettyConfig};
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod partition;
mod reduction;
mod tdm;
mod utility;
use tdm::TDM;

/// Converts a TDM instance to a Partition instance
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Prints the intermediate steps for the reduction
    #[clap(short, long)]
    verbose: bool,

    /// The input file with the TDM instance
    input_file: String,

    /// The output file that will contain the Partition instance
    output_file: String,

    /// The output file used to convert the TDM instance into a dot file format
    #[clap(short, long)]
    dot_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.input_file);
    let file = File::open(path).expect("Could not open input file");
    let instance: TDM = de::from_reader(file).unwrap();

    if let Some(dot_file) = args.dot_file.as_deref() {
        let dot_path = Path::new(dot_file);
        let dot_result = instance.to_dot();
        let mut output = File::create(dot_path).expect("Could not create file for dot output");
        write!(output, "{}", dot_result).expect("Could not write the dot file");
    }

    let write_path = Path::new(&args.output_file);
    let write_file =
        File::create(write_path).expect("Could not create file for the Partition instance");
    let config = PrettyConfig::new();
    let partition = reduction::tdm_to_partition(&instance, args.verbose);
    ser::to_writer_pretty(write_file, &partition, config)
        .expect("Error serializing the Partition problem");
}
