extern crate clap;

use self::clap::{Arg, App};

pub struct InputValues {
    pub hostname: String,
    pub count_gen: String
}

pub fn parse() -> InputValues {
    let matches = App::new("Elasticsearch data generator")
        .version("0.1")
        .author("Waldemar Neto. <waldemarnt@gmail.com>")
        .about("Elasticsearch dummy structured data generator")
        .arg(Arg::with_name("hostname")
             .short("h")
             .long("hostname")
             .help("Sets elasticsearch hostname with port")
             .takes_value(true))
        .arg(Arg::with_name("count")
             .help("Sets how many times the loop will insert data"))
        .get_matches();

    let mut host = "192.168.99.100:9200";
    let mut count = "1";

    if let Some(input_hostname) = matches.value_of("hostname") {
        host = input_hostname;
    }
    if let Some(input_count) = matches.value_of("count") {
        count = input_count;
    }
    let input = InputValues { hostname:host.to_string(), count_gen:count.to_string() };
    input
}
