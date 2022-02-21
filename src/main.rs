#![allow(unused)]

use std::io::{stdout, Write};

use clap::{Arg, Command};
use curl::easy::Easy;

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

    let mut url: String = args.value_of_t("url").unwrap_or("".to_string());
    download_page(url.as_ref);

    println!("{}", url);

    
}

fn download_page(url: &String) {
    let mut easy = Easy::new();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    println!("{}", easy.response_code().unwrap());
}
