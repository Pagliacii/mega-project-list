# The Rust Implementation Of `wc`

This package isn't only finished the "**Count Words in a String**" in the **Text** section, but also is a reproduction of `wc`.

## Description

1. Using [clap](https://crates.io/crates/clap) and [cli.yml](src/cli.yml) to handle CLI arguments.
2. The `parse` function in [main.rs](src/main.rs) converts the `clap::ArgMatches` to [`crate::config::Config`](src/config.rs).
3. The main logic is the [`run`](src/run.rs) function, which in charge of counting and printing results.
4. The [`crate::count::Counter`](src/count.rs) is in charge of counting contents from files or `stdin`.
5. The [`error.rs`](src/error.rs) is for error handling. `WCError` is using for uniforming other errors and `WCErrorKind` represents other errors' kind.
