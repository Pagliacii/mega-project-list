#[macro_use]
extern crate clap;

use std::default::Default;
use std::error::Error;
use std::fs;

use clap::{App, ArgMatches};

use wccopy::{run, Config};

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config: Config = parse(&matches);
    run(&config)?;
    Ok(())
}

fn parse(matches: &ArgMatches) -> Config {
    let mut config: Config = Default::default();

    // parse flags
    if matches.is_present("bytes")
        || matches.is_present("chars")
        || matches.is_present("lines")
        || matches.is_present("max")
        || matches.is_present("words")
    {
        config.byte_toggle(matches.is_present("bytes"));
        config.char_toggle(matches.is_present("chars"));
        config.line_toggle(matches.is_present("lines"));
        config.width_toggle(matches.is_present("max"));
        config.word_toggle(matches.is_present("words"));
    }

    // parse values
    if let Some(files_from) = matches.value_of("file") {
        // count files in the file named `files_from`
        let files =
            fs::read_to_string(files_from).expect(&format!("Failed to read file '{}'", files_from));
        for file in files.split_whitespace() {
            config.append_file(file);
        }
    } else if let Some(files) = matches.values_of("FILE") {
        for file in files {
            config.append_file(file);
        }
    } else {
        config.append_file("-");
    }

    config
}
