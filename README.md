# Patrik

## Setup

Rename `example.env` to `.env` and fill its parameters. Bot token corresponds to auth token provided by discord for your bot application. Owner ID will be used to perform authorization on owner commands. Your own user ID should be supplied. Prefix specifies the command prefix. Optionally you can set the logging level according to the specification in the [env_logger](https://crates.io/crates/env_logger) crate's page.

## Usage

Running bot:

```
cargo run
```

You can also specify log level as an eviroment variable like so:

```
RUST_LOG="logging_level" cargo run
```

If the log level isn't set using any of the methods it defaults to `patrik=info`.
