use std::path::Path;

use crate::error::AppError;

pub fn validate_path_segment<'a>(value: &'a str, field: &str) -> Result<&'a str, AppError> {
    if value.trim().is_empty() {
        return Err(AppError::bad_request(format!("{field} must not be empty")));
    }

    let path = Path::new(value);
    let is_single_segment = path.components().count() == 1;
    let exact_name_match = path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name == value);

    if is_single_segment && exact_name_match {
        Ok(value)
    } else {
        Err(AppError::bad_request(format!("invalid {field}")))
    }
}
