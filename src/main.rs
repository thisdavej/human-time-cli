use human_time_cli::*;
use std::io::{self, BufRead, IsTerminal};

fn main() {
    let args: Args = argh::from_env();
    let config = if args.config {
        if let Some(config_path) = find_config_file() {
            read_config(config_path)
        } else {
            eprintln!("Error: Config file 'human-time.toml' not found in the executable directory or home directory.");
            std::process::exit(1);
        }
    } else {
        Config::default()
    };

    if let Err(err) = validate_config(&config) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    let time_value = if let Some(value) = args.time_value {
        value
    } else if !io::stdin().is_terminal() {
        // Read from stdin
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut input = String::new();
        if handle.read_line(&mut input).is_err() || input.trim().is_empty() {
            print_error_and_exit(
                "Error: TIME_DURATION is required either as an argument or through stdin.",
            );
        }
        input.trim().parse::<u64>().unwrap_or_else(|_| {
            eprintln!("Error: Invalid TIME_DURATION provided via stdin.");
            std::process::exit(1);
        })
    } else {
        print_error_and_exit(
            "Error: TIME_DURATION is required either as an argument or through stdin.",
        );
    };

    // Try to get the time units represented by the number from the command line, then the toml config if specified
    // or default to seconds.
    let unit = args
        .unit
        .as_deref()
        .or_else(|| Some(&config.default_time_value_units))
        .unwrap_or("seconds");

    match format_duration(time_value, unit, &config) {
        Ok(formatted_duration) => println!("{}", formatted_duration),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}