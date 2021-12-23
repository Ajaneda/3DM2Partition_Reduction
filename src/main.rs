use clap::{crate_authors, App, Arg};
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

fn main() {
    let matches = App::new("TDM to Partition")
        .version("1.0.0")
        .author(crate_authors!("\n"))
        .about("Converts a TDM instance to a Partition instance")
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .help("Prints the intermediate steps for the reduction"),
        )
        .arg(
            Arg::with_name("input file")
                .takes_value(true)
                .required(true)
                .help("The input file with the TDM instance"),
        )
        .arg(
            Arg::with_name("output file")
                .takes_value(true)
                .required(true)
                .help("The output file that will contain the Partition instance"),
        )
        .arg(
            Arg::with_name("dot file")
                .short("d")
                .long("dot")
                .takes_value(true)
                .help("The output file used to convert the TDM instance into a dot file format"),
        )
        .get_matches();

    let input_file = matches.value_of("input file").unwrap();
    let output_file = matches.value_of("output file").unwrap();
    let dot_file = matches
        .value_of("dot file")
        .unwrap_or("dot-files/output.dot");
    let verbose = matches.is_present("v");

    let path = Path::new(input_file);
    let file = File::open(path).expect("Could not open input file");
    let instance: TDM = de::from_reader(file).unwrap();

    let dot_path = Path::new(dot_file);
    let dot_result = instance.to_dot();
    let mut output = File::create(dot_path).expect("Could not create file for dot output");
    write!(output, "{}", dot_result).expect("Could not write the dot file");

    let write_path = Path::new(output_file);
    let write_file =
        File::create(write_path).expect("Could not create file for the Partition instance");
    let config = PrettyConfig::new();
    let partition = reduction::tdm_to_partition(&instance, verbose);
    ser::to_writer_pretty(write_file, &partition, config)
        .expect("Error serializing the Partition problem");
}
