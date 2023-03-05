use actix_web::{post, web::Json, Either, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::dna::algos::DNAAlignment;

use crate::core::dna;
use crate::core::schema::*;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DnaString {
    #[schema()]
    dna: String,
}

#[derive(Deserialize, ToSchema)]
pub struct DnaAlign {
    #[schema()]
    dna_a: String,
    #[schema()]
    dna_b: String,
}

#[derive(Serialize, ToSchema)]
pub struct DnaNdiffs {
    #[schema()]
    ndiff: usize,
}

#[derive(Serialize, ToSchema)]
pub struct HammingDistance {
    #[schema()]
    distance: u64,
}

#[derive(Serialize, ToSchema)]
pub struct LevenshteinDistance {
    #[schema()]
    distance: u32,
}

#[derive(Serialize)]
pub struct SparseAlignment {
    score: u32,
    match_path: Vec<(u32, u32)>,
}

#[derive(Serialize, ToSchema)]
pub struct ProteinString {
    #[schema()]
    protein: String,
}

#[derive(Serialize, ToSchema)]
pub struct AminoAcids {
    #[schema()]
    amino_acids: Vec<String>,
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "ProteinString", body = ProteinString),
    ),
    params(
        ("DnaString" = DnaString, description = "DNA Strings"),
    )
)]
#[post("/dna/to_protein")]
async fn dna_to_protein(form: Json<DnaString>) -> Json<ProteinString> {
    Json(ProteinString {
        protein: dna::utils::dna_to_protein(form.dna.to_owned()),
    })
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "HttpResponse", body = HttpResponse),
    ),
    params(
        ("DnaString" = DnaString, description = "DNA Strings"),
    )
)]
#[post("/dna/circular_svg")]
async fn dna_to_circular_svg(form: Json<DnaString>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(dna::utils::gen_dna_circular_svg(form.dna.to_owned()))
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "HttpResponse"),
    ),
    params(
        ("DnaString" = DnaString, description = "DNA Strings"),
    )
)]
#[post("/dna/circular_png")]
async fn dna_to_circular_png(form: Json<DnaString>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(dna::utils::gen_dna_circular_png(form.dna.to_owned()))
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "HttpResponse", body = HttpResponse),
    ),
    params(
        ("DnaString" = DnaString, description = "DNA Strings"),
    )
)]
#[post("/dna/circular_png_bw")]
async fn dna_to_circular_png_bw(form: Json<DnaString>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(dna::utils::gen_dna_circular_png_bw(form.dna.to_owned()))
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "Amino Acids", body = AminoAcids),
    ),
    params(
        ("DnaString" = DnaString, description = "DNA Strings"),
    )
)]
#[post("/dna/to_amino_acids")]
async fn dna_to_amino_acids(form: Json<DnaString>) -> Json<AminoAcids> {
    Json(AminoAcids {
        amino_acids: dna::utils::amino_acids_from_dna(form.dna.to_owned()),
    })
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "DNA String", body = DnaString),
    ),
    params(
        ("DnaString" = DnaString, description = "DNA String"),
    )
)]
#[post("/dna/kmer_substring")]
async fn kmer_substring_from(form: Json<DnaString>) -> Json<DnaString> {
    Json(DnaString {
        dna: dna::utils::derive_kmer_substring_from_dna(form.dna.to_owned()),
    })
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "DNANDiffs", body = DnaNdiffs),
    ),
    params(
        ("DnaAlign" = DnaAlign, description = "DNA Alignment"),
    )
)]
#[post("/dna/ndiffs")]
async fn compute_dna_ndiffs(form: Json<DnaAlign>) -> Either<Json<DnaNdiffs>, Json<Error>> {
    let ndiff_compute =
        dna::utils::compute_dna_ndiffs(form.dna_a.to_owned(), form.dna_b.to_owned());

    let response = match ndiff_compute {
        Some(ndiff) => Either::Left(Json(DnaNdiffs { ndiff })),
        None => Either::Right(Json(Error {
            error: "DNA strings are not of equal length".into(),
        })),
    };

    response
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "Hamming Distance", body = HammingDistance),
    ),
    params(
        ("DnaAlign" = DnaAlign, description = "DNA Strings"),
    )
)]
#[post("/dna/hamming_distance")]
async fn compute_dna_hamming_distance(form: Json<DnaAlign>) -> Json<HammingDistance> {
    Json(HammingDistance {
        distance: dna::utils::compute_dna_hamming_distance(
            form.dna_a.to_owned(),
            form.dna_b.to_owned(),
        ),
    })
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "DNAAlignment", body = LevenshteinDistance),
    ),
    params(
        ("DnaAlign" = DnaAlign, description = "DNA Alignment"),
    )
)]
#[post("/dna/levenshtein_distance")]
async fn compute_dna_levenshtein_distance(form: Json<DnaAlign>) -> Json<LevenshteinDistance> {
    Json(LevenshteinDistance {
        distance: dna::utils::compute_dna_levenshtein_distance(
            form.dna_a.to_owned(),
            form.dna_b.to_owned(),
        ),
    })
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "SparseAlignment"),
    ),
    params(
        ("DnaAlign" = DnaAlign, description = "DNA Strings"),
    )
)]
#[post("/dna/sparse_alignment")]
async fn calculate_sparse_alignments(form: Json<DnaAlign>) -> Json<SparseAlignment> {
    let (score, match_path) =
        dna::utils::calculate_sparse_alignments(form.dna_a.to_owned(), form.dna_b.to_owned());

    Json(SparseAlignment { score, match_path })
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "DNAAlignment", body = DNAAlignment),
    ),
    params(
        ("DnaAlign" = DnaAlign, description = "DNA Strings"),
    )
)]
#[post("/dna/needleman_wunsch")]
async fn align_needleman_wunsch(form: Json<DnaAlign>) -> Json<DNAAlignment> {
    Json(dna::algos::align_needleman_wunsch(
        form.dna_a.to_owned(),
        form.dna_b.to_owned(),
    ))
}

#[utoipa::path(
    tag="DNA Algorithms",
    responses(
        (status = 200, description = "DNAAlignment", body = DNAAlignment),
    ),
    params(
        ("DnaAlign" = DnaAlign, description = "DNA Strings"),
    )
)]
#[post("/dna/smith_waterman")]
async fn align_smith_waterman(form: Json<DnaAlign>) -> Json<DNAAlignment> {
    Json(dna::algos::align_smith_waterman(
        form.dna_a.to_owned(),
        form.dna_b.to_owned(),
    ))
}
