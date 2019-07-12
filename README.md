# rustget (rget)

A parallel downloader, like wget.

## Installation

The binary name for rustget is rget

If you're a **Rust Programmer**, rustget can be installed through cargo by running:

`$ cargo install rustget`

## Building

rustget is built with Rust, so you need to have a version of Rust installed.
To build rustget:

~~~~bash
$ git clone https://github.com/SummaDiaboli/rustget
$ cd rustget
$ cargo build --release
$ ./target/release/rget --version
0.1.2
~~~~

## Using rustget

### Usage

```rust
rget [FLAGS] [OPTIONS] --dest <filename> --url <url>
```

The options and flags can come in any order as long as the required values are used.

### Examples

Valid uses of rget:

```bash
$ rget -u https://www.google.com -d index.html
$ rget -d index -u https://www.google.com
$ rget -d index.html -u https://www.google.com -t 6
$ rget -t 6 -d ~/Documents/index.html -r 4 -u https://www.google.com
```

Invalid uses of rget:

```bash
$ rget -d index.html -u www.google.com
$ rget -u https://www.google.com -d index.html -t fish
$ rget -u https://www.google.com -r fish -d index.html
```

### Flags

`-h`: Show rustget's condensed help output.
`--help`: Show rustget's longer form help output.
`-V/--version`: Prints the version information.
`-v/--verbosity`: Pass many times for more log output. By default, it'll only report errors. Passing `-v` one time also prints warnings, `-vv` enables info logging, `-vvv` debug, and `-vvvv` trace.

### Options

`-d/--dest <filename>`: Create filename. It could also the path, including the filename. **This is required**.
`-r/--retry <retry>`: Set number of time to retry download after failures in downloading. It defaults to `0`.
`-t/--threads <threads>`: Set the number of threads to use for the download. It defaults to `4`.
`-u/--url <url>`: The url to fetch the file from. **This is required**.
