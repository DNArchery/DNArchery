# rust-genomics

## About
A Rust library related to analyzing genomic data (reading files, parsing sequences, etc). 

RustGenomics contains tools to help with some of the major analysis of genes and genomes, including:
* parsing fasta files
* reviewing sequence validity
* viewing sequences in various reading frames
* finding open reading frames and longest open reading frames in genes
* Genome Related Structs (e.g. Sequence) with methods and associated functions

Some of these functions, such as file parsing and sequence analysis have various versions (e.g. concurrent/normal) that you can use for different sizes. These functions are benchmarked and tested with the criterion crate. 

For more information, see the documentation generated [here](https://joyliu-q.github.io/rust-genomics/rust_genomics/) or run ```cargo doc``` on your terminal.

## Main Technologies Used
* rayon: for concurrency in reading files
* criterion: for benchmarking and testing

## Usage Example
```rust
use rust_genomics::{Sequence, FASTA};

fn main() {
    let file_path = "data/haha-1.fasta";

    // Generate an instance of the FASTA struct by reading data from the given file
    let mut fasta = FASTA::rayon_read_fasta(file_path);

    // The FASTA instance has the Display trait, allowing it to be visualized in a clean fashion
    println!("{}", fasta);

    println!("Epic");

    // Find all longest open reading frame for fasta, with concurrency
    let lorfs = fasta.find_lorfs(true);

    println!("{}", lorfs.len());
}
```
