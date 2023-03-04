use serde::{Serialize, Deserialize};
use actix_web::{post, web::{Json, Either}, Responder};

use crate::core::sequence;

#[derive(Deserialize)]
struct Sequence {
    sequence: String,
}

#[derive(Deserialize)]
struct NucleotideIndex {
    sequence: String,
    index: usize,
}

#[derive(Serialize)]
struct SingleLorf {
    lorf: String,
    length: usize,
}

#[derive(Serialize)]
struct MultiLorf {
    lorfs: Vec<String>,
    length: usize,
}

#[derive(Deserialize)]
struct GenomicSequence {
    length: i64,
}

#[post("/sequence/nucleotide_at_index")]
async fn nucleotide_at_index(form: Json<NucleotideIndex>) -> impl Responder {
    sequence::utils::nucleotide_index(form.sequence.to_owned(), form.index)
}

#[post("/sequence/codon_frames")]
async fn codon_frames(form: Json<Sequence>) -> impl Responder {
    sequence::utils::codon_frames(form.sequence.to_owned())
}

#[post("/sequence/lorf")]
async fn seq_lorf(form: Json<Sequence>) -> Either<Json<SingleLorf>, Json<MultiLorf>> {
    let lorf = sequence::utils::seq_lorf(form.sequence.to_owned());

    match lorf {
        Either::Left(lorf) => {
            let length = lorf.len();
            Either::Left(Json(SingleLorf {
                lorf,
                length
            }))
        },
        Either::Right(lorfs) => {
            let length = lorfs.len();
            Either::Right(Json(MultiLorf {
                lorfs,
                length
            }))
        },
    }
}

#[post("/sequence/random")]
async fn seq_random(form: Json<GenomicSequence>) -> impl Responder {
    sequence::utils::seq_random(form.length)
}