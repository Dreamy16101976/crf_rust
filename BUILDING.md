# Building crf_rust

## Repository

Clone down the [crf_rust repository](https://github.com/Dreamy16101976/crf_rust).

## Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install). If it's already installed, make sure it's up-to-date.
- Install [<i>libv4l</i>](https://github.com/philips/libv4l) - collection of video4linux support libraries:
  ```
  sudo apt install libv4l-dev
  ```

## Required crates

- [<i>camera_capture</i>](https://crates.io/crates/camera_capture) - for frames capture
- [<i>image</i>](https://crates.io/crates/image/) - An Image Processing Library, for camera_capture
- [<i>chrono</i>](https://crates.io/crates/chrono) - Timezone-aware date and time handling, ffor capture speed calculation

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

the binary <i>crf_rust</i> can be found in `target/release` folder.

