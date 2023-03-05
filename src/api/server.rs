use actix_web::{get, App, HttpServer, Responder};
use actix_cors::Cors;
use actix_files as fs;

const PORT: u16 = 1337;

/// All endpoints
use super::endpoints::{
    fasta::{
        lorf_from_fasta
    },
    sequence::{
        nucleotide_at_index,
        codon_frames,
        seq_lorf,
        seq_random
    },
    dna::{
        dna_to_protein,
        dna_to_circular_svg,
        dna_to_circular_png,
        dna_to_circular_png_bw,
        dna_to_amino_acids,
        kmer_substring_from,
        compute_dna_ndiffs,
        compute_dna_hamming_distance,
        compute_dna_levenshtein_distance,
        calculate_sparse_alignments,
        align_needleman_wunsch,
        align_smith_waterman,
    }
};

#[get("/")]
async fn index() -> impl Responder {
    "DNArchery API Server v0.1.0"
}

#[actix_web::main]
pub async fn spin() -> Result<(), std::io::Error> {
    info!("DNArchery API server listening on port {}", PORT);
    info!("Browse to http://127.0.0.1:1337/ui for the UI");

    HttpServer::new(||
        App::new()
            .wrap(
                Cors::permissive()
            )
            .service(index)
            .service(lorf_from_fasta)
            .service(nucleotide_at_index)
            .service(codon_frames)
            .service(seq_lorf)
            .service(seq_random)
            .service(dna_to_circular_svg)
            .service(dna_to_circular_png)
            .service(dna_to_circular_png_bw)
            .service(dna_to_amino_acids)
            .service(dna_to_protein)
            .service(kmer_substring_from)
            .service(compute_dna_ndiffs)
            .service(compute_dna_hamming_distance)
            .service(compute_dna_levenshtein_distance)
            .service(calculate_sparse_alignments)
            .service(align_needleman_wunsch)
            .service(align_smith_waterman)
            .service(
                fs::Files::new("/ui", "ui/build")
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            .service(
                fs::Files::new("/static", "ui/build/static")
                    .use_last_modified(true),
            )            
        )
        .bind(("127.0.0.1", PORT)).unwrap()
        .run()
        .await
}