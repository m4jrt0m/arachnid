#![allow(unused)]

use std::io::{stdout, Write};

use clap::{Arg, Command};
use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

const KEYWORDS: &str = "\"http";

fn main() {
    let args = Command::new("arachnid")
        .version("0.0.1")
        .about("arachnid is a webpage crawler that will list all urls found")
        .author("m4jrt0m")
        .args(&[Arg::new("url")
            .short('u') // Needs to be a char
            .long("url")
            .help("url to crawl into")
            .takes_value(true)])
        .get_matches();

    println!("");
    println!("########################");
    println!("########################");
    println!("#       Arachnid       #");
    println!("########################");
    println!("########################");
    println!("");

    let mut url: String = args.value_of_t("url").unwrap_or((&"").to_string());
    let mut page: String = download_page(&url.as_str());

    println!("Url: {}", url);
    //println!("{}", page);

    
}

fn download_page(url: &str) -> String{
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.url(url).unwrap();
    easy.perform().unwrap();
    
    assert_eq!(easy.response_code().unwrap(), 200);
    let contents = easy.get_ref();
    return String::from_utf8_lossy(&contents.0).as_ref().to_string();
}
