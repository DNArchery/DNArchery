<img src="https://raw.githubusercontent.com/DNArchery/DNArchery/main/assets/dnarchery-logo.png" alt="binserve logo" width="250" align="right">

# `DNArchery` 🧬

> A free and open-source cross-platform DNA Sequencing/Visualization Software for bioinformatics research.

A toolkit for instantly performing sequencing, alignment, distance algorithms on DNA strings and visualizing their structure. It's like [GParted](https://github.com/GNOME/gparted) but for genome sequencing.

The core goal of the project is to be a good general-purpose toolkit for DNA sequencing/alignment/distance analysis and to bring more open-source attention to bioinformatics by promoting more research software in the industry to be open-source. (Read [Goals](#goals))

Built with ❤️ at [**FOSSHack 3.0**](https://fossunited.org/fosshack/2023)!

<p align="left">
    <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" title="version" alt="version">
    <a href="https://github.com/dnarchery/dnarchery/blob/master/LICENSE"><img alt="gitHub license" src="https://img.shields.io/github/license/dnarchery/dnarchery.svg"></a>
</p>

## Table of Contents

- [Goals](#goals)
- [Features](#features)
- [Installation](#installation)
- [Build From Source](#build-from-source)
- [GUI](#gui)
- [Web API (OpenAPI Schema)](#web-api)
- [Screenshots](#screenshots)
- [Citations & Acknowledgments](#citations--acknowledgements)
- [Contribution](#contribution)
- [License](#license)

## Goals

- First goal is to make **DNArchery** a good general-purpose toolkit for DNA sequencing/alignment/distance analysis.
- Most utility bioinformatics software used by the industry are commercial (See [List](https://github.com/cmdcolin/awesome-genome-visualization#commercial)), this project is intended to bring a groundwork into making more utilities in the industry open-source.
- The structure of the project is written in such a way that implementing new functionality like algorithms/visualizations are just plugging new functions to the set of [utils](https://github.com/DNArchery/DNArchery/tree/main/src/core) already present, this is intended to reduce the barrier of entry in encouraging more open-source contributions. (Read [Contribution](#contribution))
- This tool comprises of an amalgamation of sequencing/alignment/distance algorithms like [Needleman–Wunsch algorithm
](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm), [Smith–Waterman algorithm](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm), and [LCSk++](https://arxiv.org/abs/1407.2407) etc. in a single package such that for a general-purpose DNA analysis this could be a go-to utility.
- A secondary goal being, promoting the usage of Rust :crab: in building more open-source research software as it brings the performance and correctness required for parallel processing like in the case of this utility.

## Features

- The core of the utility is exposed as a Webservice API along with an OpenAPI schema such that UIs or Apps can be built on top, just the like main UI toolkit DNArchery comes with. (See [GUI](#gui))
- **Toolsets:**
  - **Conversions** - From DNA to Amino Acids, Proteins, Codon frames, etc.
  - **Sequencing Algorithms** - [K-mer](https://en.wikipedia.org/wiki/K-mer), Ndiffs, and more.
  - **Distance/Compare Algorithms** - [Needleman–Wunsch algorithm
](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm), [Smith–Waterman algorithm](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm), [LCSk++](https://arxiv.org/abs/1407.2407), [Hamming distance](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5410656/), Sparse alignments, etc.
- **Performance** - Processing large FASTA files/DNA sequences are pretty fast as most of them utilize vectorized or parallelized algorithms<sup>[<a href="https://github.com/DNArchery/DNArchery/blob/main/src/core/fasta/utils.rs#L22-L23">example</a>]</sup>.

## Installation

Download the executable from [**Releases**](https://github.com/DNArchery/DNArchery/releases) OR Install with `cargo`:

```sh
$ cargo install --git https://github.com/DNArchery/DNArchery.git
$ dnarchery
```

[Install Rust/Cargo](https://rust-lang.org/tools/install)

### Build From Source

**Prerequisites:**

* [Git](https://git-scm.org/downloads)
* [Rust](https://rust-lang.org/tools/install)
* Cargo (Automatically installed when installing Rust)
* A C linker (Only for Linux, generally comes pre-installed)

```sh
$ git clone https://github.com/DNArchery/DNArchery.git
$ cd DNArchery/
$ # < GUI dependencies >
$ sudo apt-get install -y gir1.2-javascriptcoregtk-4.0
$ sudo apt-get install -y libwebkit2gtk-4.1-dev
$ # </ GUI dependencies >
$ cargo build --release
$ ./target/release/dnarchery
```

The first command clones this repository into your local machine and the last two commands enters the directory and builds the source in release mode.

## Web API

You can see the OpenAPI Schema on the Swagger UI at http://127.0.0.1:1337/swagger/ui

<table>
  <tr>
    <td><img src="https://user-images.githubusercontent.com/PLACEHOLDER"></td>
   </tr>
</table>

## Contribution

As stated in the goals section, one of our primary goal is to provide a low barrier contributing opportunity to the bioinformatics open-source space. If you want to add more DNA sequencing/alignment/conversion algorithms, you can browse to `src -> core ->` and chose which part you want to extend. Every super module in the tree have the same structure, a `utils.rs` file which contain all functions, you can add a new function and implement it as an exposed actix-web endpoint in the `src -> api -> endpoints.rs` and that's it.

**Ways to contribute:**

- Suggest a feature
- Report a bug
- Fix something and open a pull request
- Help me document the code
- Spread the word

## License

Licensed under the MIT License, see <a href="https://github.com/DNArchery/DNArchery/blob/master/LICENSE">LICENSE</a> for more information.

## Citations & Acknowledgements

This project wouldn't exist without these resources (libraries/blogs):

- [rust-bio](https://github.com/rust-bio/rust-bio)
- [rust-genomics](https://github.com/joyliu-q/rust-genomics)
- [rust-debruijn](https://github.com/10XGenomics/rust-debruijn)
- [resvg](https://github.com/RazrFalcon/resvg)

---