//! Validate a compact snapshot file against wxj schemas.

use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationSummary {
    pub line_count: usize,
    pub valid_count: usize,
    pub schema_errors: Vec<SchemaError>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SchemaError {
    pub line_num: usize,
    pub error: String,
}

impl ValidationSummary {
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.schema_errors.is_empty()
    }
}

fn validate_data_schema(value: &Value) -> bool {
    if let Some(obj) = value.as_object() {
        matches!(obj.get("data"), Some(Value::Object(_)))
            && matches!(obj.get("includes"), Some(Value::Object(_)))
    } else {
        false
    }
}

fn validate_flat_schema(value: &Value) -> bool {
    if let Some(obj) = value.as_object() {
        matches!(obj.get("created_at"), Some(Value::String(_)))
            && matches!(obj.get("text"), Some(Value::String(_)))
            && matches!(
                obj.get("id"),
                Some(Value::Number(_)) | Some(Value::String(_))
            )
    } else {
        false
    }
}

pub fn validate<P: AsRef<Path>>(input: &P, flat: bool) -> Result<ValidationSummary, Error> {
    let file = File::open(input)?;
    let decoder = zstd::Decoder::new(file)?;
    let reader = BufReader::new(decoder);

    let mut summary = ValidationSummary {
        line_count: 0,
        valid_count: 0,
        schema_errors: Vec::new(),
    };

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        summary.line_count += 1;

        let value: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                summary.schema_errors.push(SchemaError {
                    line_num: line_num + 1,
                    error: format!("Invalid JSON: {}", e),
                });
                continue;
            }
        };

        let is_valid = if flat {
            validate_flat_schema(&value)
        } else {
            validate_data_schema(&value)
        };

        if is_valid {
            summary.valid_count += 1;
        } else {
            let schema_name = if flat { "wxj/flat" } else { "wxj/data" };
            summary.schema_errors.push(SchemaError {
                line_num: line_num + 1,
                error: format!("Does not match {} schema", schema_name),
            });
        }
    }

    Ok(summary)
}
