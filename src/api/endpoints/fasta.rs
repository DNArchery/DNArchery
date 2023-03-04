use serde::{Serialize, Deserialize};
use actix_web::{post, web::{Json, Either}};

use crate::core::schema::{Error};
use crate::core::fasta;

#[derive(Deserialize)]
struct FastaFile {
    path: String,
}

#[derive(Serialize)]
struct Lorf {
    lorfs_indexes: String,
    length: usize,
}

#[post("/fasta/lorf")]
async fn lorf_from_fasta(form: Json<FastaFile>) -> Either<Json<Lorf>, Json<Error>> {
    match fasta::utils::lorf_from_fasta(&form.path) {
        Ok((lorf, len)) => {
            Either::Left(Json(Lorf {
                lorfs_indexes: lorf,
                length: len,
            }))
        },
        Err(e) => {
            Either::Right(Json(Error {
                error: e.to_string(),
            }))
        }
    }
}
