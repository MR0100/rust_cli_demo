// Command Line Application.

use std::env;
use std::process;

use cli_app::parse_config::ParseConfig as pc;
use cli_app::reader::read;

fn main() {
    // fetch command line args while running the script. 
    let args: Vec<String> = env::args().collect();

    let config = pc::parse_config(&args).unwrap_or_else(|err| { 
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("Search {} in file {} | CASE SENSITIVITY -> {}", config.query, config.filename, config.is_case_sensitive);

    read(config).unwrap_or_else(|err| {
        eprintln!("Failed to read file : {}", err);
        process::exit(1);
    });  
}