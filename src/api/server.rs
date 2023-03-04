use actix_web::{get, App, HttpServer, Responder};
use portpicker::pick_unused_port;

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
        dna_to_protein
    }
};

#[get("/")]
async fn index() -> impl Responder {
    "DNArchery API Server v0.1.0"
}

#[actix_web::main]
pub async fn spin() -> std::io::Result<()> {
    let port = pick_unused_port().expect("Failed to find an unused port");

    info!("DNArchery API server listening on port {}", port);

    HttpServer::new(||
        App::new()
            .service(index)
            .service(lorf_from_fasta)
            .service(nucleotide_at_index)
            .service(codon_frames)
            .service(seq_lorf)
            .service(seq_random)
            .service(dna_to_protein)
        )
        .bind(("127.0.0.1", 1337))?
        .run()
        .await
}