use actix_web::{
    post,
    web::{Either, Json},
    Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::sequence;

#[derive(Deserialize, ToSchema)]
pub struct Sequence {
    #[schema()]
    sequence: String,
}

#[derive(Deserialize, ToSchema)]
pub struct NucleotideIndex {
    #[schema()]
    sequence: String,
    #[schema()]
    index: usize,
}

#[derive(Serialize, ToSchema)]
pub struct SingleLorf {
    #[schema()]
    lorf: String,
    #[schema()]
    length: usize,
}

#[derive(Serialize, ToSchema)]
pub struct MultiLorf {
    #[schema()]
    lorfs: Vec<String>,
    #[schema()]
    length: usize,
}

#[derive(Deserialize, ToSchema)]
pub struct GenomicSequence {
    #[schema()]
    length: i64,
}

#[utoipa::path(
    tag="DNA Sequencing",
    responses(
        (status = 200, description = "Responder", body = Responder),
    ),
    params(
        ("NucleotideIndex" = NucleotideIndex, description = "DnaAlign"),
    )
)]
#[post("/sequence/nucleotide_at_index")]
async fn nucleotide_at_index(form: Json<NucleotideIndex>) -> impl Responder {
    sequence::utils::nucleotide_index(form.sequence.to_owned(), form.index)
}

#[utoipa::path(
    tag="DNA Sequencing",
    responses(
        (status = 200, description = "Responder", body = Responder),
    ),
    params(
        ("Sequence" = Sequence, description = "Sequence"),
    )
)]
#[post("/sequence/codon_frames")]
async fn codon_frames(form: Json<Sequence>) -> impl Responder {
    sequence::utils::codon_frames(form.sequence.to_owned())
}

#[utoipa::path(
    tag="DNA Sequencing",
    responses(
        (status = 200, description = "Lorf", body = Lorf),
    ),
    params(
        ("Sequence" = Sequence, description = "Surface"),
    )
)]
#[post("/sequence/lorf")]
async fn seq_lorf(form: Json<Sequence>) -> Either<Json<SingleLorf>, Json<MultiLorf>> {
    let lorf = sequence::utils::seq_lorf(form.sequence.to_owned());

    match lorf {
        Either::Left(lorf) => {
            let length = lorf.len();
            Either::Left(Json(SingleLorf { lorf, length }))
        }
        Either::Right(lorfs) => {
            let length = lorfs.len();
            Either::Right(Json(MultiLorf { lorfs, length }))
        }
    }
}

#[utoipa::path(
    tag="DNA Sequencing",
    responses(
        (status = 200, description = "Responder", body = Responder),
    ),
    params(
        ("GenomicSequence" = GenomicSequence, description = "GenomicSequence"),
    )
)]
#[post("/sequence/random")]
async fn seq_random(form: Json<GenomicSequence>) -> impl Responder {
    sequence::utils::seq_random(form.length)
}
