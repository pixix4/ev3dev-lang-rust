# Rust language bindings for ev3dev

[![Build Status](https://travis-ci.org/pixix4/ev3dev-lang-rust.svg?branch=master)](https://travis-ci.org/pixix4/ev3dev-lang-rust)

## Warning

This is a work in progress project. It is NOT ready for use!

## Building

1. Build docker image
    ```bash
    docker build . -t pixix4/ev3dev-rust-cross
    ```

2. Start docker image
    ```bash
    docker run -it --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust-cross
    ```

3. Build binary for ev3dev
    ```bash
    cargo build --release --target armv5te-unknown-linux-gnueabi
    ```
    The `--release` flag is optional. Nevertheless it can speedup the execution time by a factor of 30.
    
The target binary is now in `target/armv5te-unknown-linux-gnueabi/release/libev3dev_lang_rust.rlib`