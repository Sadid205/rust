use minigrep::search;
use std::env;
use std::error::Error; // Box<dyn Error> এর জন্য
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // কনফিগারেশন তৈরি
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    // run ফাংশন কল এবং এরর হ্যান্ডলিং
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ফাইল পড়া (একবারই)
    let contents = fs::read_to_string(config.filename)?;

    // search কল করে লাইনগুলো প্রিন্ট করা
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
