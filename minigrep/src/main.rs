use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args()
        .collect();
    // dbg!(args); // Prints the value of args to the console for debugging purposes.

    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            // Problem parsing arguments: not enough arguments
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    let elapsed = now.elapsed();
    println!("Execution time: {elapsed:.2?}");
}

pub struct Config {
    query: String,
    file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if 3 > args.len() {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var(
            "IGNORE_CASE"
        ).is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(
        config.file_path
    )?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}