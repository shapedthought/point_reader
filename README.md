# Point Reader

This is a simple application that can take a VSE export JSON file with the points enabled and display them in a table and optionally to a CSV file.

## Installation

1. Clone the repository
2. Install Rust
3. Run `cargo run --release`
4. Get the exe from the `target/release` folder

OR

1. Clone the repository
2. Install Rust
3. Run `cargo install --path .`

This will install it locally on your machine directly and add it to your path.

To uninstall it, run `cargo uninstall point_reader`

### Build using Docker

1. [Install Docker](https://docs.docker.com/engine/install/)
2. Clone the repository `git clone https://github.com/shapedthought/point_reader.git`
3. `cd point_reader`
4. build the image `docker build -t point_reader .`
5. run the following:

```
docker run -it --rm -v ${PWD}:/tmp point_reader /bin/bash

root@2bf585234d8a:/app# ./point_reader -f /tmp/VSE_export.csv -t pc -s /tmp/workload1_points.csv
```

## Usage

CLI:

```
Usage: point_reader.exe [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>            Path to file
  -t, --tiers <TIERS>          [default: pca]
  -s, --save-file <SAVE_FILE>
  -h, --help                   Print help
  -V, --version                Print version
```

## Example

```
point_reader.exe -f VSE_export.csv -t pc -s points.csv
```

The -t allows you to select which tiers are displayed/saved.

- p = Performance Tier
- c = Capacity Tier
- a = Archive Tier

The -s allows you to save the points to a CSV file, you will need to specify the file name.
