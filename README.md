<img src="https://raw.githubusercontent.com/DNArchery/DNArchery/main/assets/dnarchery-logo.png" alt="dnarchery logo" width="250" align="right">

# `DNArchery` 🧬

> A free and open-source cross-platform DNA Sequencing/Visualization Software for bioinformatics research.

A toolkit for instantly performing sequencing, alignment, distance algorithms on DNA strings and visualizing their structure. It's like [GParted](https://github.com/GNOME/gparted) but for genome sequencing.

The core goal of the project is to be a good general-purpose toolkit for DNA sequencing/alignment/distance analysis and to bring more open-source attention to bioinformatics by promoting more research software in the industry to be open-source. (Read [Goals](#goals))

Built with ❤️ at [**FOSSHack 3.0**](https://fossunited.org/fosshack/2023)!

<p align="left">
    <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" title="version" alt="version">
    <a href="https://github.com/dnarchery/dnarchery/blob/master/LICENSE"><img alt="github license" src="https://img.shields.io/github/license/dnarchery/dnarchery.svg"></a>
</p>

## Table of Contents

- [Goals](#goals)
- [Features](#features)
- [Installation](#installation)
- [Build From Source](#build-from-source)
- [Web API (OpenAPI Schema)](#web-api)
- [GUI](#gui)
- [Citations & Acknowledgments](#citations--acknowledgements)
- [Contribution](#contribution)
- [License](#license)
- [Project Progress](#faq)

## Goals

- First goal is to make **DNArchery** a good general-purpose toolkit for DNA sequencing/alignment/distance analysis.
- Most utility bioinformatics software used by the industry are commercial (See [List](https://github.com/cmdcolin/awesome-genome-visualization#commercial)), this project is intended to bring a groundwork into making more utilities in the industry open-source.
- The structure of the project is written in such a way that implementing new functionality like algorithms/visualizations are just plugging new functions to the set of [utils](https://github.com/DNArchery/DNArchery/tree/main/src/core) already present, this is intended to reduce the barrier of entry in encouraging more open-source contributions. (Read [Contribution](#contribution))
- This tool comprises of an amalgamation of sequencing/alignment/distance algorithms like [Needleman–Wunsch algorithm
](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm), [Smith–Waterman algorithm](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm), and [LCSk++](https://arxiv.org/abs/1407.2407) etc. in a single package such that for a general-purpose DNA analysis this could be a go-to utility.
- A secondary goal being, promoting the usage of Rust :crab: in building more open-source research software as it brings the performance and correctness required for parallel processing like in the case of this utility.

## Features

- The core of the utility is exposed as a Webservice API (Rust backend) along with an OpenAPI schema such that UIs or Apps can be built on top, just the like main UI toolkit DNArchery comes with. (See [GUI](#gui))
- **Toolsets:**
  - **Conversions** - From DNA to Amino Acids, Proteins, Codon frames, etc.
  - **Sequencing Algorithms** - [K-mer](https://en.wikipedia.org/wiki/K-mer), Ndiffs, and more.
  - **Distance/Compare Algorithms** - [Needleman–Wunsch algorithm
](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm), [Smith–Waterman algorithm](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm), [LCSk++](https://arxiv.org/abs/1407.2407), [Hamming distance](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5410656/), Sparse alignments, etc.
- **Performance** - Processing large FASTA files/DNA sequences are pretty fast as most of them utilize vectorized or parallelized algorithms<sup>[<a href="https://github.com/DNArchery/DNArchery/blob/main/src/core/fasta/utils.rs#L22-L23">example</a>]</sup>.

## Installation

Install the executable with the following command:

> **Note**
> You would need the assets for the GUI integration to work, follow the [Build From Source](#build-from-source) instructions to set up.

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

You can see the OpenAPI Schema (Swagger UI) at http://127.0.0.1:1337/swagger-ui/.

<table>
  <tr>
    <td><img src="https://raw.githubusercontent.com/DNArchery/DNArchery/main/assets/swagger-ui-screenshot.png"></td>
   </tr>
</table>

## GUI

_The integrated GUI is a proof-of-concept into how the underlying API schema can be utilized._


<table>
  <tr>
<td><img src="https://raw.githubusercontent.com/DNArchery/DNArchery/main/assets/screenshot-1.png"></td>
  <td><img src="https://raw.githubusercontent.com/DNArchery/DNArchery/main/assets/screenshot-2.png"></td>
  <td><img src="https://raw.githubusercontent.com/DNArchery/DNArchery/main/assets/screenshot-3.png"></td>
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

## FAQ

**_FOSSHack Questionnaire:_**

Q. What was the initial stage of the project?

> The idea of the project is to create a utility box of various DNA sequencing algorithms exposed via a API schema such that UIs, Apps can be built on top. The integratred GUI is an example of this.
>
> The initial stage is just a code structure that provides an easy way to embed new algorithms just by adding a new function to one of the `utils.rs` files. (Either in `dna`/`sequence`/`fasta`)
>
> This is then exposed via a Web API (Rust backend) (See [Code](https://github.com/DNArchery/DNArchery/tree/main/src/api)). This API is utilized for the integrated GUI app.

Q. What stage is it in now?

> The Rust backend is complete. The GUI on top is a proof-of-concept stage.

Q. How did you get there?

> Authors of the project are interested in bioinformatics, during the course of development of this project, we researched about the basic needs in a DNA sequencing software and looked into resources to implement it in code. Thanks to the resources (See [Citations & Acknowledgments](#citations--acknowledgements) for Credits), we were able to utilize some libraries for algorithms and we implemented the rest with the documentation.
>
> We chose Rust as the primary language as that's what we are comfortable with and it provides the necessary performance in running compute intensive algorithms.

Q. What is working/not working?

> The backend is stable and works according to the specified schema. The GUI is just a wrapper around it that does I/O from the same API.

---
