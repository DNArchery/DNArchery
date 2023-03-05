use serde::{Serialize, Deserialize};
use actix_web::{post, web::{Json, Either}};
use utoipa::ToSchema;

use crate::core::schema::{Error};
use crate::core::fasta;

#[derive(Deserialize, ToSchema)]
pub struct FastaFile {
    #[schema()]
    path: String,
}

#[derive(Serialize, ToSchema)]
pub struct Lorf {
    #[schema()]
    lorfs_indexes: String,
    #[schema()]
    length: usize,
}

#[utoipa::path(
    tag="FastA Parsing",
    responses(
        (status = 200, description = "Lorf", body = Lorf),
    ),
    params(
        ("FastAFile" = FastAFile, description = "FastAFile"),
    )
)]
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
