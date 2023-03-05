use actix_web::{get, App, HttpServer, Responder};
use actix_cors::Cors;
use actix_files as fs;

const PORT: u16 = 1337;

use utoipa_swagger_ui::SwaggerUi;

use utoipa::OpenApi;

/// All endpoints
use super::endpoints::{
    dna::{
        align_needleman_wunsch, align_smith_waterman, calculate_sparse_alignments,
        compute_dna_hamming_distance, compute_dna_levenshtein_distance, compute_dna_ndiffs,
        dna_to_amino_acids, dna_to_circular_png, dna_to_circular_png_bw, dna_to_circular_svg,
        dna_to_protein, kmer_substring_from,
    },
    fasta::lorf_from_fasta,
    sequence::{codon_frames, nucleotide_at_index, seq_lorf, seq_random},
};

#[get("/")]
async fn index() -> impl Responder {
    "DNArchery API Server v0.1.0"
}

#[actix_web::main]
pub async fn spin() -> Result<(), std::io::Error> {
    info!("DNArchery API server listening on port {}", PORT);
    info!("Browse to http://127.0.0.1:1337/ui for the UI");

    #[derive(OpenApi)]
    #[openapi(info(
        description = "A free and open-source DNA Sequencing/Visualization software for bioinformatics research. "
    ))]
    #[openapi(
        paths(
// DNA Endpoints
super::endpoints::dna::dna_to_protein,
super::endpoints::dna::dna_to_circular_svg,
super::endpoints::dna::dna_to_circular_png,
super::endpoints::dna::dna_to_circular_png_bw,
super::endpoints::dna::dna_to_amino_acids,
super::endpoints::dna::kmer_substring_from,
super::endpoints::dna::compute_dna_ndiffs,
super::endpoints::dna::compute_dna_hamming_distance,
super::endpoints::dna::compute_dna_levenshtein_distance,
super::endpoints::dna::calculate_sparse_alignments,
super::endpoints::dna::align_needleman_wunsch,
super::endpoints::dna::align_smith_waterman,
// FastA Endpoints
super::endpoints::fasta::lorf_from_fasta,
// Sequence Endpoints
super::endpoints::sequence::nucleotide_at_index,
super::endpoints::sequence::codon_frames,
super::endpoints::sequence::seq_lorf,
super::endpoints::sequence::seq_random,
        ),
        components(schemas(
// DNA Endpoints
super::endpoints::dna::DnaString,
super::endpoints::dna::DnaAlign,
super::endpoints::dna::DnaNdiffs,
super::endpoints::dna::HammingDistance,
super::endpoints::dna::LevenshteinDistance,
super::endpoints::dna::ProteinString,
super::endpoints::dna::AminoAcids,
// FastA Endpoints
super::endpoints::fasta::FastaFile,
super::endpoints::fasta::Lorf,
// Sequence Endpoints
super::endpoints::sequence::Sequence,
super::endpoints::sequence::NucleotideIndex,
super::endpoints::sequence::SingleLorf,
super::endpoints::sequence::MultiLorf,
super::endpoints::sequence::GenomicSequence,
        ))
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 1337))?
    .run()
    .await
}
