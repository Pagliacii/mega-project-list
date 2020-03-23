#[macro_use]
extern crate clap;

use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

use clap::{App, ArgMatches};

use wccopy::{run, Config};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config: Config = match parse(&matches) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {:?}", e);
            process::exit(1);
        }
    };
    if let Err(e) = run(&config) {
        eprintln!("error: {:?}", e);
        process::exit(1);
    }
}

fn parse(matches: &ArgMatches) -> Result<Config, Box<dyn Error>> {
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
        // Note 1: files in the `files_from` are NUL-terminated names.
        // Note 2: `files_from` can be "-" which means the stdin.
        // Note 3: wc: when reading file names from stdin, no file name of '-' allowed
        let mut reader: Box<dyn io::Read> = if "-" == files_from {
            // read file names from the stdin
            Box::new(io::stdin())
        } else {
            // read file names from the file
            Box::new(File::open(files_from)?)
        };
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        for utf8_bytes in buffer.split(|&elem| elem == 0u8) {
            let file = &String::from_utf8(utf8_bytes.to_vec())?;
            if "-" == file && "-" == files_from {
                panic!("wc: when reading file names from stdin, no file name of '-' allowed");
            }
            if !file.is_empty() {
                config.append_file(file);
            }
        }
    } else if let Some(files) = matches.values_of("FILE") {
        for file in files {
            config.append_file(file);
        }
    } else {
        config.append_file("-");
    }

    Ok(config)
}
