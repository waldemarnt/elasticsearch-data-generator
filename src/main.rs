extern crate clap;
extern crate curl;
extern crate rustc_serialize;
extern crate chrono;
extern crate elastic_data_generator;

use std::io::Read;
use std::fs::File;
use curl::easy::{Easy,List};
use rustc_serialize::json::Json;
use elastic_data_generator::*;
use std::io::{stdout, Write};

fn main() {
    let input: cli::InputValues = cli::parse();
    let mut easy = Easy::new();
    easy.url(&input.hostname).unwrap();
    easy.perform().unwrap();
    println!("Elasticsearch returned {}", easy.response_code().unwrap());
    let mut f = File::open("structure.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let decoded = Json::from_str(&s).unwrap();
    let obj = decoded.as_object().unwrap();
    let mut map = generator::generate(obj);
    let mut data = map.as_bytes();
    easy = Easy::new();
    easy.url(&input.hostname).unwrap();
    easy.post(true).unwrap();
    let total_size = map.chars().count() as u64;
    easy.post_field_size(total_size);
    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap())
        }).unwrap();
        transfer.write_function(|data| {
           Ok(stdout().write(data).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }
}
