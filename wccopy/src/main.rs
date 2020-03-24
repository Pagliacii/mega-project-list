#[macro_use]
extern crate clap;

use std::default::Default;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

use clap::{App, ArgMatches};

use wccopy::error::{WCError, WCErrorKind};
use wccopy::{run, Config};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config: Config = match parse(&matches) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };
    if let Err(e) = run(&config) {
        e.exit();
    }
}

fn parse(matches: &ArgMatches) -> Result<Config, WCError> {
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
            match File::open(files_from) {
                Ok(f) => Box::new(f),
                Err(e) => {
                    let mut message = format!("failed to open '{}'", files_from);
                    if e.kind() == io::ErrorKind::NotFound {
                        message = format!("'{}' : No such file or directory", files_from);
                    }
                    return Err(WCError::new(1, WCErrorKind::OpenFailed(e), &message));
                }
            }
        };
        let mut buffer = Vec::new();
        if let Err(e) = reader.read_to_end(&mut buffer) {
            return Err(WCError::new(
                1,
                WCErrorKind::ReadFailed(e),
                &format!("failed to read contents from '{}'", files_from),
            ));
        }
        for utf8_bytes in buffer.split(|&elem| elem == 0u8) {
            let file = match str::from_utf8(utf8_bytes) {
                Ok(f) => f,
                Err(e) => {
                    return Err(WCError::new(
                        1,
                        WCErrorKind::InvalidBytes(e),
                        "invalid UTF-8 bytes",
                    ))
                }
            };
            if "-" == file && "-" == files_from {
                return Err(WCError::new(
                    1,
                    WCErrorKind::NotAllowed,
                    "when reading file names from stdin, no file name of '-' allowed",
                ));
            }
            if !file.is_empty() {
                config.append_file(&file);
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
