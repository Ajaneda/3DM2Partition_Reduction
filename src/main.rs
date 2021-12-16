use ron::de;
use ron::ser::{self, PrettyConfig};
use std::fs::File;
use std::path::Path;
use clap::{Arg, App, crate_authors};

mod partition;
mod reduction;
mod tdm;
use tdm::TDM;

fn main() {
    let matches = App::new("TDM to Partition")
        .version("1.0.0")
        .author(crate_authors!("\n"))
        .about("Teaches argument parsing")
        .arg(Arg::with_name("v")
                 .short("v")
                 .long("verbose")
                 .help("Prints the intermediate steps for the reduction"))
        .arg(Arg::with_name("input file")
                 .takes_value(true)
                 .required(true)
                 .help("The input file with the TDM instance"))
        .arg(Arg::with_name("output file")
                 .takes_value(true)
                 .required(true)
                 .help("The output file that will contain the Partition instance"))
        .get_matches();

    let input_file = matches.value_of("input file").unwrap();
    let output_file = matches.value_of("output file").unwrap_or("input.txt");
    let verbose = matches.is_present("v");

    let path = Path::new(input_file);
    let file = File::open(path).expect("Could not open input file");
    let instance: TDM = de::from_reader(file).unwrap();

    let write_path = Path::new(output_file);
    let write_file = File::create(write_path).expect("Could not create file");
    let config = PrettyConfig::new();
    let partition = reduction::tdm_to_partition(&instance, verbose);
    ser::to_writer_pretty(write_file, &partition, config)
        .expect("Error serializing the Partition problem");
}
