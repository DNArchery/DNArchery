use serde::{Serialize, Deserialize};
use actix_web::{post, web::Json, Either, HttpResponse};

use crate::core::dna::algos::NeedlemanWunschAlignment;

use crate::core::dna;
use crate::core::schema::*;

#[derive(Serialize, Deserialize)]
struct DnaString {
    dna: String,
}

#[derive(Deserialize)]
struct DnaAlign {
    dna_a: String,
    dna_b: String,
}

#[derive(Serialize)]
struct DnaNdiffs {
    ndiff: usize,
}

#[derive(Serialize)]
struct HammingDistance {
    distance: u64,
}

#[derive(Serialize)]
struct LevenshteinDistance {
    distance: u32,
}

#[derive(Serialize)]
struct SparseAlignment {
    score: u32,
    match_path: Vec<(u32, u32)>
}

#[derive(Serialize)]
struct ProteinString {
    protein: String,
}

#[derive(Serialize)]
struct AminoAcids {
    amino_acids: Vec<String>,
}

#[post("/dna/to_protein")]
async fn dna_to_protein(form: Json<DnaString>) -> Json<ProteinString> {
    Json(ProteinString {
        protein: dna::utils::dna_to_protein(form.dna.to_owned()),
    })
}

#[post("/dna/circular_svg")]
async fn dna_to_circular_svg(form: Json<DnaString>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(dna::utils::gen_dna_circular_svg(form.dna.to_owned()))
}

#[post("/dna/circular_png")]
async fn dna_to_circular_png(form: Json<DnaString>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(dna::utils::gen_dna_circular_png(form.dna.to_owned()))
}

#[post("/dna/circular_png_bw")]
async fn dna_to_circular_png_bw(form: Json<DnaString>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(dna::utils::gen_dna_circular_png_bw(form.dna.to_owned()))
}

#[post("/dna/to_amino_acids")]
async fn dna_to_amino_acids(form: Json<DnaString>) -> Json<AminoAcids> {
    Json(
        AminoAcids {
            amino_acids: dna::utils::amino_acids_from_dna(form.dna.to_owned())
        }
    )
}

#[post("/dna/kmer_substring")]
async fn kmer_substring_from(form: Json<DnaString>) -> Json<DnaString> {
    Json(
        DnaString {
            dna: dna::utils::derive_kmer_substring_from_dna(form.dna.to_owned())
        }
    )
}

#[post("/dna/ndiffs")]
async fn compute_dna_ndiffs(form: Json<DnaAlign>) -> Either<Json<DnaNdiffs>, Json<Error>> {
    let ndiff_compute = dna::utils::compute_dna_ndiffs(
        form.dna_a.to_owned(),
        form.dna_b.to_owned()
    );

    let response = match ndiff_compute {
        Some(ndiff) => Either::Left(Json(DnaNdiffs { ndiff })),
        None => Either::Right(Json(Error { error: "DNA strings are not of equal length".into() }))
    };

    response
}

#[post("/dna/hamming_distance")]
async fn compute_dna_hamming_distance(form: Json<DnaAlign>) -> Json<HammingDistance> {
    Json(
        HammingDistance {
            distance: dna::utils::compute_dna_hamming_distance(
                form.dna_a.to_owned(),
                form.dna_b.to_owned()
            )
        }
    )
}

#[post("/dna/levenshtein_distance")]
async fn compute_dna_levenshtein_distance(form: Json<DnaAlign>) -> Json<LevenshteinDistance> {
    Json(
        LevenshteinDistance {
            distance: dna::utils::compute_dna_levenshtein_distance(
                form.dna_a.to_owned(),
                form.dna_b.to_owned()
            )
        }
    )
}

#[post("/dna/sparse_alignment")]
async fn calculate_sparse_alignments(form: Json<DnaAlign>) -> Json<SparseAlignment> {
    let (score, match_path) = dna::utils::calculate_sparse_alignments(
        form.dna_a.to_owned(),
        form.dna_b.to_owned()
    );

    Json(
        SparseAlignment {
            score,
            match_path
        }
    )
}

#[post("/dna/needleman_wunsch")]
async fn align_needleman_wunsch(form: Json<DnaAlign>) -> Json<NeedlemanWunschAlignment> {
    Json(
        dna::algos::align_needleman_wunsch(
            form.dna_a.to_owned(),
            form.dna_b.to_owned()
        )
    )
}
