use std::path::Path;
use std::fs::File;
use ron::de;

mod tdm;
use tdm::TDM;

fn main() {
    let path = Path::new("instances/3dm/test.ron");
    let file = File::open(&path).expect("Could not open input file");
    let instance: TDM = de::from_reader(file).unwrap();
    dbg!(instance);
}
