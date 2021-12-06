use std::path::Path;
use std::fs::File;
use ron::ser::{self, PrettyConfig};
use ron::de;

mod tdm;
mod partition;
mod reduction;
use tdm::TDM;
use partition::Partition;

fn main() {
    let path = Path::new("instances/3dm/test.ron");
    let file = File::open(path).expect("Could not open input file");
    let instance: TDM = de::from_reader(file).unwrap();
    println!("{:?}", instance);

    let write_path = Path::new("instances/partition/test.ron");
    let write_file = File::create(write_path).expect("Could not create file");
    let config = PrettyConfig::new();
    let partition = reduction::tdm_to_partition(&instance);
    ser::to_writer_pretty(write_file, &partition, config).expect("soy viren y soy muy guapo uwu *shy*"); // TODO cambiar esto
}