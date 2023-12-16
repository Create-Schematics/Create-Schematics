use poem::web::Json;
use poem::{IntoResponse, Response};
use poem::http::{StatusCode, HeaderValue};
use poem::http::header::WWW_AUTHENTICATE;
use poem::http::HeaderMap;
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;

/// A common error type that can be used throughout the API.
///
/// Can be returned in a `Result` from an API handler function.
///
/// For convenience, this represents both API errors as well as internal recoverable errors,
/// and maps them to appropriate status codes along with at least a minimally useful error
/// message in a plain text body, or a JSON body in the case of `UnprocessableEntity`.
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    // Return '400 Bad Request'
    #[error("bad request")]
    BadRequest,

    // Return '401' Unauthorized, this is typically only
    // raised by middleware when a token is missing, expired,
    // malformed or otherwise invalid
    #[error("authentication is required, please provide a valid bearer token")]
    Unauthorized,

    // Return '403 Forbidden, for when the user's identity
    // is known but they are either not member of the project
    // they are attempting to access or do not have permissions
    // within the project to perform the action within the
    // project
    #[error("insufficient permissions to perform this action")]
    Forbidden,

    // Return '404' Not Found
    #[error("resource not found")]
    NotFound,
    
    /// Return '409 Conflict'
    #[error("this resource already exists")]
    Conflict,

    /// Return `422 Unprocessable Entity`
    ///
    /// This also serializes the `errors` map provided to JSON
    ///
    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    /// Automatically return `500 Internal Server Error` on a `sqlx::Error`.
    ///
    /// Via the generated `From<sqlx::Error> for Error` impl,
    /// this allows using `?` on database calls in handler functions without a manual mapping step.
    ///
    /// The actual error message isn't returned to the client for security reasons.
    /// It should be logged instead.
    ///
    /// Note that this could also contain database constraint errors, which should usually
    /// be transformed into client errors (e.g. `422 Unprocessable Entity` or `409 Conflict`).
    /// See `ResultExt` below for a convenient way to do this.
    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    /// Automatically return `500 Internal Server Error` on a `redis::RedisError`.
    ///
    /// Via the generated `From<redis::RedisError> for Error` impl,
    /// this allows using `?` on database calls in handler functions without a manual mapping step.
    ///
    /// The actual error message isn't returned to the client for security reasons.
    /// It should be logged instead.
    /// 
    #[error("an error occurred with the database")]
    Redis(#[from] redis::RedisError),

    /// Return `500 Internal Server Error` on a `anyhow::Error`.
    ///
    /// `anyhow::Error` is used in a few places to capture context and backtraces
    /// on unrecoverable (but technically non-fatal) errors which could be highly useful for
    /// debugging. We use it a lot in our code for background tasks or making API calls
    /// to external services so we can use `.context()` to refine the logged error.
    ///
    /// Via the generated `From<anyhow::Error> for Error` impl, this allows the
    /// use of `?` in handler functions to automatically convert `anyhow::Error` into a response.
    ///
    /// Like with `Error::Sqlx`, the actual error message is not returned to the client
    /// for security reasons.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error)
}

impl ApiError {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key.
    ///
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Conflict => StatusCode::CONFLICT,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR, 
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
           Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }
                    
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            },

            Self::Unauthorized => {
                let mut headers = HeaderMap::new();
                
                // Include the `WWW-Authenticate` challenge required in the specification
                // for the `401 Unauthorized` response code:
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401
                //
                headers.append(WWW_AUTHENTICATE, HeaderValue::from_static("session"));

                return (headers, self.status_code()).into_response();
            }

            Self::Sqlx(ref e) => {
                tracing::error!("SQLx error: {:?}", e);
            }

            Self::Redis(ref e) => {
                tracing::error!("Redis error: {:?}", e);
            }

            Self::Anyhow(ref e) => {
                tracing::error!("Generic error: {:?}", e);
            }

            _ => ()
        };

        (self.status_code(), self.to_string()).into_response()
    }
}

/// A little helper trait for more easily converting database constraint errors into API errors.
///
/// ```rust,ignore
/// let user_id = sqlx::query_scalar!(
///     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
///     username,
///     email,
///     password_hash
/// )
///     .fetch_one(&ctxt.db)
///     .await
///     .on_constraint("user_username_key", |_| Error::unprocessable_entity([("username", "already taken")]))?;
/// ```
///
/// Something like this would ideally live in a crate if it made sense to author one,
/// however its definition is tied pretty intimately to the `ResponseError` type, which is itself
/// tied directly to application semantics.
///
/// To actually make this work in a generic context would make it quite a bit more complex,
/// as you'd need an intermediate error type to represent either a mapped or an unmapped error,
/// and even then it's not clear how to handle `?` in the unmapped case without more boilerplate.
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> ApiError,
    ) -> Result<T, ApiError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<ApiError>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> ApiError,
    ) -> Result<T, ApiError> {
        self.map_err(|e| match e.into() {
            ApiError::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}