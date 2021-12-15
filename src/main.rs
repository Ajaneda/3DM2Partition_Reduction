use ron::de;
use ron::ser::{self, PrettyConfig};
use std::fs::File;
use std::path::Path;

mod partition;
mod reduction;
mod tdm;
use partition::Partition;
use tdm::TDM;

fn main() {
    let path = Path::new("instances/3dm/riera.ron");
    let file = File::open(path).expect("Could not open input file");
    let instance: TDM = de::from_reader(file).unwrap();
    println!("{:?}", instance);

    let write_path = Path::new("instances/partition/riera.ron");
    let write_file = File::create(write_path).expect("Could not create file");
    let config = PrettyConfig::new();
    let partition = reduction::tdm_to_partition(&instance);
    ser::to_writer_pretty(write_file, &partition, config)
        .expect("soy viren y soy muy guapo uwu *shy*"); // TODO cambiar esto
}
