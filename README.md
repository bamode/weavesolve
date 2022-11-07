# Weavesolve

A simple command-line utility written in Rust to solve the word ladder game [weaver](https://wordwormdormdork.com). Might be eventually expanded to download and cache a larger dictionary in order to expand beyond the 4-letter word ladder game. Feel free to fork, download, etc. Should compile with stable 2021 Rust, and the only dependent crates are [clap](https://crates.io/crates/clap) and [colored](https://crates.io/crates/colored).

## Install

An install script, `install.sh` is provided for installation on *NIX systems. It assumes that `/bin/bash` exists and that an appropriate version of the `rustc` and `cargo` toolchain are installed. The command `chmod +x install.sh` might be necessary to make the install script executable after download.

## Usage

```
$ weavesolve word dork

word -> work -> dork
```
