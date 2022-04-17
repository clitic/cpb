# cpb

**This is an experimental project.**

<p align="center">
  <img src="https://img.shields.io/github/downloads/clitic/cpb/total?style=flat-square">
  <img src="https://img.shields.io/github/release/clitic/cpb?style=flat-square">
  <img src="https://img.shields.io/github/license/clitic/cpb?style=flat-square">
  <img src="https://img.shields.io/github/repo-size/clitic/cpb?style=flat-square">
  <img src="https://img.shields.io/tokei/lines/github/clitic/cpb?style=flat-square">
</p>

`cpb` stands for `copy + progress bar`, it copies files like `cp` command but with a progress bar (alternatively gui). `cpb` cli version is supported by almost any platform and gui version is also supported by many platforms. Curently `cpb` cannot be an complete alternative to `cp` but in few cases `cpb` can be helpful.

![showcase_cli](https://raw.githubusercontent.com/clitic/cpb/main/images/showcase_cli.gif)
![showcase_feature_gui](https://raw.githubusercontent.com/clitic/cpb/main/images/showcase_feature_gui.gif)

## Building From Source

- Install [Rust](https://www.rust-lang.org) 1.60 or above.

- Clone Repository

```bash
git clone https://github.com/clitic/cpb.git
```

- Build Release

```bash
cargo build --release
# TO USE GUI
# cargo build --release --features gui
```

## Usage

```
cpb 0.1.0
clitic <clitic21@gmail.com>
Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY with a progress bar

USAGE:
    cpb.exe [OPTIONS] <SOURCE>... <DEST>

ARGS:
    <SOURCE>...    List of sources to copy
    <DEST>         Destination path or directory

OPTIONS:
    -c, --chunk-size <CHUNK_SIZE>    Copy chunk size [default: 8192]
    -g, --gui                        Show a gui dialog instead of terminal output
    -h, --help                       Print help information
    -n, --no-progress                Disable progress bar
    -V, --version                    Print version information
```

## TODOs

- [] ask before overwrite and other operations
- [] gitignore support with [gitignore.rs](https://github.com/nathankleyn/gitignore.rs)

## License

&copy; 2022 clitic

This repository is licensed under the MIT license. See LICENSE for details.
