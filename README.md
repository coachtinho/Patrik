# Patrik

## Setup

Rename `example.env` to `.env` and fill its parameters. Bot token corresponds to auth token provided by discord for your bot application. Owner ID will be used to perform authorization on owner commands. Your own user ID should be supplied.

## Usage

Running bot:
```
RUST_LOG="logging_level" cargo run
```

**Note**: The bot uses [env_logger](https://crates.io/crates/env_logger) so you should refer to that in order to set logging level. Defaults to `patrik=info`.
