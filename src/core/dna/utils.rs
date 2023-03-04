/// Credits:
/// 
/// Index Tables and conversions (protein) from: https://github.com/dweb0/protein-translate/blob/master/src/lib.rs
/// 

use plasmid::prelude::*;
use plasmid::seq::DnaSequence;

use debruijn::Kmer;
use debruijn::dna_string::*;
use debruijn::kmer::Kmer16;
use debruijn::Vmer;

use bio::alignment::distance::*;

use nsvg;
use resvg::usvg_text_layout::{fontdb, TreeTextToPath};

use actix_web::web::Bytes; // for SVG byte object
use std::io::Cursor; // in-memory buffer for PNG

// https://en.wikipedia.org/wiki/DNA_and_RNA_codon_tables
static ASCII_TO_INDEX: [usize; 128] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 0-15
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 16-31
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 32-47
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 48-63
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, // 64-79    (65 = A, 67 = C, 71 = G)
    4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 80-95    (84 = T, 85 = U)
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, // 96-111   (97 = a, 99 = c, 103 = g)
    4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 112-127  (116 = t, 117 = u)
];

/// https://www.ncbi.nlm.nih.gov/Taxonomy/Utils/wprintgc.cgi
static AA_TABLE_CANONICAL: [[[char; 4]; 4]; 4] = [
    [
        ['K', 'N', 'K', 'N'], // AAA, AAC, AAG, AAU/AAT
        ['T', 'T', 'T', 'T'], // ACA, ACC, ACG, ACU/ACT
        ['R', 'S', 'R', 'S'], // AGA, AGC, AGG, AGU/AGT
        ['I', 'I', 'M', 'I'], // AUA/ATA, AUC/ATC, AUG/ATG, AUU/ATT
    ],
    [
        ['Q', 'H', 'Q', 'H'], // CAA, CAC, CAG, CAU/CAT
        ['P', 'P', 'P', 'P'], // CCA, CCC, CCG, CCU/CCT
        ['R', 'R', 'R', 'R'], // CGA, CGC, CGG, CGU/CGT
        ['L', 'L', 'L', 'L'], // CUA/CTA, CUC/CTC, CUG/CTG, CUU/CTT
    ],
    [
        ['E', 'D', 'E', 'D'], // GAA, GAC, GAG, GAU/GAT
        ['A', 'A', 'A', 'A'], // GCA, GCC, GCG, GCU/GCT
        ['G', 'G', 'G', 'G'], // GGA, GGC, GGG, GGU/GGT
        ['V', 'V', 'V', 'V'], // GUA/GTA, GUC/GTC, GUG/GTG, GUU/GTT
    ],
    [
        ['*', 'Y', '*', 'Y'], // UAA/TAA, UAC/TAC, UAG/TAG, UAU/TAT
        ['S', 'S', 'S', 'S'], // UCA/TCA, UCC/TCC, UCG/TCG, UCU/TCT
        ['*', 'C', 'W', 'C'], // UGA/TGA, UGC/TGC, UGG/TGG, UGU/TGT
        ['L', 'F', 'L', 'F'], // UUA/TTA, UUC/TTC, UUG/TTG, UUU/TTT
    ],
];

/// Convert a DNA sequence to a protein sequence.
/// Takes a string of DNA and returns a string of amino acids.
pub fn dna_to_protein(seq: String) -> String {
    let seq = seq.as_bytes();

    let mut peptide = String::with_capacity(seq.len() / 3);

    'outer: for triplet in seq.chunks_exact(3) {
        for c in triplet {
            if !c.is_ascii() {
                peptide.push('X');
                continue 'outer;
            }
        }

        let c1 = ASCII_TO_INDEX[triplet[0] as usize];
        let c2 = ASCII_TO_INDEX[triplet[1] as usize];
        let c3 = ASCII_TO_INDEX[triplet[2] as usize];

        let amino_acid = if c1 == 4 || c2 == 4 || c3 == 4 {
            'X'
        } else {
            AA_TABLE_CANONICAL[c1][c2][c3]
        };

        peptide.push(amino_acid);
    }
    peptide
}

/// Break DNA into codon sets.
pub fn codon_chunks_from_dna(seq: String) -> Vec<String> {
    let cod_len = 3;

    let divide = seq.as_bytes()
        .chunks(cod_len)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let mut codon_chunks: Vec<String> = Vec::new();

    for i in &divide {
        codon_chunks.push(i.to_string());
    }

    return codon_chunks;    
}

/// Derive amino acid sets from DNA sequence.
/// https://en.wikipedia.org/wiki/DNA_and_RNA_codon_tables#Inverse_RNA_codon_table
pub fn amino_acids_from_dna(seq: String) -> Vec<String> {
    let mut amino_acids = Vec::new();

    // break DNA into codons
    let codons = codon_chunks_from_dna(seq);

    for codon in codons {
        // codon to amino acid jump table
        match codon.as_str() {
            "GUU" | "GUC" | "GUA" | "GUG" =>                 amino_acids.push("Valine".into()),
            "GCU" | "GCC" | "GCA" | "GCG" =>                 amino_acids.push("Alanine".into()),
            "GAU" | "GAC" =>                                 amino_acids.push("Aspartic Acid".into()),
            "GAA" | "GAG" =>                                 amino_acids.push("Glutamic Acid".into()),
            "GGU" | "GGC" | "GGA" | "GGG" =>                 amino_acids.push("Glycine".into()),
            "UUU" | "UUC" =>                                 amino_acids.push("Phenylalanine".into()),
            "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => amino_acids.push("Leucine".into()),
            "UCU" | "UCC" | "UCA" | "UCG" | "AGU" | "AGC" => amino_acids.push("Serine".into()),
            "UAU" | "UAC" =>                                 amino_acids.push("Tyrosine".into()),
            "UAA" | "UAG" =>                                 amino_acids.push("STOP".into()),
            "UGU" | "UGC" =>                                 amino_acids.push("Cysteine".into()),
            "UGA" =>                                         amino_acids.push("STOP".into()),
            "UGG" =>                                         amino_acids.push("Tryptophan".into()),
            "CCU" | "CCC" | "CCA" | "CCG" =>                 amino_acids.push("Proline".into()),
            "CAU" | "CAC" =>                                 amino_acids.push("Histidine".into()),
            "CAA" | "CAG" =>                                 amino_acids.push("Glutamine".into()),
            "CGU" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => amino_acids.push("Arginine".into()),
            "AUU" | "AUC" | "AUA" =>                         amino_acids.push("Isoleucine".into()),
            "AUG" =>                                         amino_acids.push("Methionine".into()),
            "ACU" | "ACC" | "ACA" | "ACG" =>                 amino_acids.push("Threonine".into()),
            "AAU" | "AAC" =>                                 amino_acids.push("Asparginine".into()),
            "AAA" | "AAG" =>                                 amino_acids.push("Lysine".into()),
            _ => ()            
        }
    }

    amino_acids
}

/// Generate a circular structure of DNA sequence in SVG format.
pub fn gen_dna_circular_svg(seq: String) -> Bytes {
    let mut seq: DnaSequence = DnaSequence::from_str(seq).unwrap();

    // Annotate restriction enzyme cut sites
    seq.annotate_restriction_enzymes();

    // Generate SVG of circular DNA
    let conf = SvgExportConfig::circular();
    let svg = SvgExport::new(conf, seq.as_nucleotides());    

    Bytes::from(svg.export())
}

/// Generate a circular structure of DNA sequence in B/W PNG format.
/// This essentially converts the above SVG generation to raw PNG file.
pub fn gen_dna_circular_png_bw(seq: String) -> Bytes {
    let svg = gen_dna_circular_svg(seq);

    // parse RAW svg as UTF8 (handle replacement chars if any)
    let svg =  String::from_utf8_lossy(&svg);

    let parse_svg = nsvg::parse_str(&svg, nsvg::Units::Pixel, 96.0).unwrap();

    // Rasterize the loaded SVG and return an RgbaImage
    let image = parse_svg.rasterize(2.0).unwrap();

    let (width, height) = image.dimensions();

    let mut raw_bytes = Cursor::new(Vec::new());

    image::write_buffer_with_format(
        &mut raw_bytes,
        &image.into_raw(),
        width,
        height,
        image::ColorType::Rgba8,
        image::ImageOutputFormat::Png
    ).expect("Failed to render png.");

    Bytes::from(raw_bytes.into_inner())
}

/// Generate a circular structure of DNA sequence in PNG format.
/// This essentially converts the above SVG generation to raw PNG file.
pub fn gen_dna_circular_png(seq: String) -> Bytes {
    let svg = gen_dna_circular_svg(seq);

    let opt = usvg::Options::default();

    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();

    let mut tree = usvg::Tree::from_str(&String::from_utf8_lossy(&svg), &opt).unwrap();
    tree.convert_text(&fontdb);

    let pixmap_size = tree.size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(
        &tree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();

    Bytes::from(pixmap.encode_png().unwrap())
}

/// Derive K-mer substring from DNA Sequence
pub fn derive_kmer_substring_from_dna(seq: String) -> String {
    let dna_string = DnaString::from_dna_string(&seq);
    let first_kmer: Kmer16 = dna_string.get_kmer(0);
    first_kmer.to_string()
}

/// Compute ndiffs of two DNA Sequences
pub fn compute_dna_ndiffs(dna_a: String, dna_b: String) -> Option<usize> {
    if dna_a.len() != dna_b.len() {
        return None;
    }

    let dna_a = DnaString::from_dna_string(&dna_a);
    let dna_b = DnaString::from_dna_string(&dna_b);

    Some(ndiffs(&dna_a, &dna_b))
}

/// Compute hamming distance of two DNA Sequences
pub fn compute_dna_hamming_distance(dna_a: String, dna_b: String) -> u64 {
    hamming(
        dna_a.as_bytes(),
        dna_b.as_bytes()
    )
}

/*
TODO: Implement this

// Check if NdeI cut site exists
if let Some(ann) = seq.annotation_iter().find(|ann| ann.text == "NdeI") {
    println!("Found NdeI cut site (start: {}; end: {})", ann.start, ann.end);
}
 */