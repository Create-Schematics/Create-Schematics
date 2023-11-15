use crate::error::ApiError;

pub type ApiResult<T> = std::result::Result<T, ApiError>;
