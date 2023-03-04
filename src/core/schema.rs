use serde::{Serialize};

/// Endpoint Schema Specification
/// Every endpoint response have a specification schema
#[derive(Serialize)]
pub struct Schema {
    /// Tooltip describing the function of the endpoint
    pub tooltip: String,

    /// Description explaining the terminologies/parameters used in the endpoint
    pub description: String,
}

/// Message response schema
#[derive(Serialize)]
pub struct Message {
    message: String,
}

/// Error response schema
#[derive(Serialize)]
pub struct Error {
    pub error: String,
}