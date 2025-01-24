[![Rust](https://github.com/stela2502/regionomics/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/stela2502/regionomics/actions/workflows/rust.yml)

# Regionomics

**Welcome to regionomics!**  
Where genomic regions meet the future of bioinformatics. 🌿🧬  
It's time to take your BED files, GTF annotations, and genomic insights to a whole new level! Whether you're hunting for transcription factor binding sites or exploring the entire genomic landscape, regionomics is your trusty guide.

Or at least we could probably get there ;-) 


## Features
- **BED-to-GTF Proximity Mapping**: Discover genes in proximity to your transcription factor peaks.
- **Flexible Analysis**: Expand your genomic exploration beyond just one task—regionomics grows with your needs!
- **High Performance**: Built with Rust, for when your genomic data needs to be handled swiftly and efficiently.

## Installation

To get started with regionomics, follow these easy steps:

### Prerequisites
Make sure you have [Rust](https://www.rust-lang.org/learn/get-started) installed. If not, install it using [rustup](https://rustup.rs/).

### Install regionomics
Clone this repository and compile it:

```bash
git clone https://github.com/yourusername/regionomics.git
cd regionomics
cargo build --release
```

This will install **regionomics** on your machine. You can now use it to explore your genomic regions!

Alternatively, if you're just looking to use regionomics, you can also install it via Cargo directly (once it's published to crates.io):

```bash
#cargo install regionomics
cargo install --git https://github.com/stela2502/regionomics
```

## Usage

Once installed, run the tool with a command like:

```bash
regulatus -h
regionomics 0.1.0
Stefan L. <stefan.lang@med.lu.se>
regulatus - A tool identifying potentially regulated genes in a gtf file based on distance to regions in
a bed like file

USAGE:
    regulatus [OPTIONS] --bed <BED> --gtf <GTF>

OPTIONS:
    -b, --bed <BED>              Path to the input BED file containing genomic regions
    -d, --distance <DISTANCE>    The distance in base pairs to search for genes within the region
                                 (default is 1000) [default: 1000]
    -g, --gtf <GTF>              Path to the input GTF file containing gene annotations
    -h, --help                   Print help information
    -o, --outfile <OUTFILE>      Path to the output file to save the results (default is stdout)
                                 [default: stdout]
    -V, --version                Print version information
```

Or
```bash
regulatus -b input myfile.bed -o result.txt -d 5000 -g my_genes.gtf
```

This will analyze your BED file, looking for genes within 5kb of the peaks. Expand functionality as your research grows! 🌱

**Note**: Replace `myfile.bed` with your BED file, and tweak the parameters to your needs.

## Speed

On a rather old Ubuntu desktop (AMD Ryzen 5 3600X 6-Core Processor (12 logical processors), 64Gb of Ram) regulatus intersected 3000 bam entries with the gencode.v38.chr_patch_hapl_scaff.annotation.gtf.gz human annotation in a little over 6 seconds.

## Contributing

We welcome contributions! Feel free to fork the repo, make changes, and open pull requests. There's always more to explore in the world of genomic regions.

### Contributing Guidelines

- **Add your region**: If you’ve got a new genomic analysis task or feature, we’d love to hear it!
- **Bug fixes**: Found an issue? Fix it and submit a PR—let’s keep regionomics running smoothly!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Shoutout to the Rust community and all bioinformaticians who keep pushing the boundaries of genomic research. 🧬✨

---

### Why "regionomics"?

Because **regionomics** is where *regions* and *omics* converge! Whether you're dealing with genomic regions, epigenomics, or anything in between, regionomics is your toolkit for understanding the landscape.

Let the exploration begin! 🌍🔬
