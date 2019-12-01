use std::fs::File;
use std::io::BufReader;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    #[structopt(long, short)]
    input: std::path::PathBuf,
}

pub fn get_config() -> Config {
    Config::from_args()
}

pub fn open_input(c: &Config) -> BufReader<File> {
    BufReader::new(File::open(&c.input).expect("could not open input file"))
}
