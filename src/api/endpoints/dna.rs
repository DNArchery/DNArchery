use serde::{Serialize, Deserialize};
use actix_web::{post, web::Json};

use crate::core::dna;

#[derive(Deserialize)]
struct DnaString {
    dna: String,
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