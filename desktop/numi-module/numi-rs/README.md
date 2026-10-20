**Numi** is a calculator app with natural language expressions.

## CLI commands

Cd into cli `cd cli` first, then use `cargo run [expr] [args]`. Can install with `cargo install` and use globally `numi-cli`.

#### Supported arguments

-   `-p`, `--precision` number of decimal digits
-   `-n`, `--no-cache` disable any cache for remote sources
-   `-v`, `--verbose` for a verbose output

Numi uses [log](https://docs.rs/log/latest/log/) crate for logging, can specify log level with env variable: `RUST_LOG=debug cargo run "2+3" --verbose`.
Additional arguments to the cli directly can be passed after `--`: `cargo run -- "2 cm" --precision 2`.

Here is example with many arguments: `RUST_LOG=debug cargo run --features default-tls -- "2 см" --locale ru --verbose`

## Docker

You can use included docker file to test/run the app. The version is very similar to what about to be used in CI. Here is a list of commonly used commands.

Build image & run default cargo test:

```
docker build -t numi . && docker run --rm -v .:/home/runner/src numi && docker image prune -f
```

Build image:

```
docker build --progress=plain -t numi .
```

Run container, mount sources and enter bash:

```
docker run -it --rm -v .:/home/runner/src --name numi_cont numi /bin/bash
```

Combined:

```
docker build -t numi . && docker run -it --rm -v .:/home/runner/src --name numi_cont numi /bin/bash && docker image prune -f
```

## Testing library

When working on library, to test specific test use `cargo test [testname]`.

To debug specific test for errors and see all debug! statements, make sure env_logger [initialized properly](https://docs.rs/env_logger/latest/env_logger/#capturing-logs-in-tests) by adding such line to the test or `init()`:

```
let _ = env_logger::builder().is_test(true).try_init();
```

and run the test with

```
RUST_LOG=debug cargo test [testname]
```

Alternatively, can use `dbg!` to print out debug values quickly and run a test with `nocapture`:

```
cargo test [testname] -- --nocapture
```

## Testling cli

Testing expression with locale: `RUST_LOG=debug cargo run --features default-tls -- "2 см" --locale ru --verbose`

## Sending patches via email

Make sure you configured git for sending patches via email:

```
git config --global --edit
```

Sample config:

```
[sendemail]
	smtpserver = smtp.migadu.com
	smtpuser = mail@nyko.cc
	smtpencryption = ssl
	smtpserverport = 465
```

Configure default sendemail "to" for the repo:

```
git config sendemail.to "~nyko/numi@lists.sr.ht"
```

Then you can send your changes for review:

```
git send-email HEAD^
```

[Read more](https://git-send-email.io) about email workflow.
