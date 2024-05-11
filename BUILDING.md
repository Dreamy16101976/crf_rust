# Building crf_rust

## Repository

Clone down the [crf_rust repository](https://github.com/Dreamy16101976/crf_rust).

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install). If it's already installed, make sure it's up-to-date:

## Required crates

- camera_capture - for frames capture
- image - for camera_capture
- chrono - for capture speed calculation

## Building from source

Once you have the dependencies installed, you can build crf_rust using [Cargo](https://doc.rust-lang.org/cargo/).

For a debug build:

```
cargo run
```

For a release build:

```
cargo run --release
```

For a release package:

```
cargo build --release
```

the binary <i>crf</i> can be found in `target/release` folder.

