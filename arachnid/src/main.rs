#![allow(unused)]

use clap::{Command, Arg};


fn main() {
	let args = Command::new("arachnid")
			.version("0.0.1")
			.about("arachnid is a webpage crawler that will list all urls found")
			.author("m4jrt0m")
			.args(&[
				Arg::new("url")
					.short('u') // Needs to be a char
					.long("url")
					.help("url to crawl into")
					.takes_value(true)
			]).get_matches();

	println!("");
	println!("########################");
	println!("########################");
	println!("####### Arachnid #######");
	println!("########################");
	println!("########################");
	println!("");

	let url: String = args.value_of_t("url").unwrap_or("".to_string());
	println!("{}", url);
}
