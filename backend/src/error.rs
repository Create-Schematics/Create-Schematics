use poem_openapi::payload::Json;
use poem_openapi::ApiResponse;
use poem_openapi_derive::Object;
use sqlx::error::DatabaseError;
use std::collections::HashMap;
use time::OffsetDateTime;

/// A common error type that can be used throughout the API.
///
/// Can be returned in a `Result` from an API handler function.
///
/// For convenience, this represents both API errors as well as internal recoverable errors,
/// and maps them to appropriate status codes along with at least a minimally useful error
/// message in a plain text body, or a JSON body in the case of `UnprocessableEntity`.
///
/// This error handling solution should probably be replaced at some point, this is a
/// re-implemenation of the existing errors to work with poem
///
/// https://github.com/Create-Schematics/Create-Schematics/blob/50adde233e45a66ede0bf1d3013bf8e2c0a81623/backend/src/error.rs
///
#[derive(Debug, ApiResponse)]
pub enum ApiError {
    // Return '400 Bad Request'
    #[oai(status = 400)]
    BadRequest,

    // Return '401' Unauthorized, this is typically only
    // raised by middleware when a token is missing, expired,
    // malformed or otherwise invalid
    #[oai(status = 401)]
    Unauthorized,

    // Return '403 Forbidden, for when the user's identity
    // is known but they are either not member of the project
    // they are attempting to access or do not have permissions
    // within the project to perform the action within the
    // project
    #[oai(status = 403)]
    Forbidden,

    /// Return `403 Forbidden`, for when the user may have a valid
    /// session and permissions but has an active timeout, returning
    /// how long it is for and the reason why. If the duration is
    /// not given then the timeout is permanent
    #[oai(status = 403)]
    Banned(Json<Punishment>),

    // Return '404' Not Found
    #[oai(status = 404)]
    NotFound,

    /// Return `422 Unprocessable Entity`
    ///
    /// This also serializes the `errors` map provided to JSON
    ///
    #[oai(status = 422)]
    UnprocessableEntity(Json<EntityErrors>),

    /// Return `500 Internal Server Error`
    ///
    /// This should generally be called implicity by another
    /// error see implementation bellow
    ///
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(Debug, Object, Serialize)]
pub struct EntityErrors {
    /// Structure to return unprocessable data in with the erroneous field
    /// as the key and the reason it cannot be handled in the value.
    ///
    /// Ideally we would use a Cow<'static, str> here to avoid unnessasery
    /// cloning of the strings but does not implement ParseFromJSON
    ///
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Debug, Object, Serialize, Deserialize, Default)]
pub struct Punishment {
    pub until: Option<OffsetDateTime>,
    pub reason: Option<String>,
}

impl ApiError {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key.
    ///
    pub fn unprocessable_entity<K, V>(values: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let mut errors: HashMap<String, Vec<String>> = HashMap::new();

        for (key, val) in values {
            errors
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity(Json(EntityErrors { errors }))
    }
}

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
///
impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        tracing::error!("Generic error: {:?}", error);

        ApiError::InternalServerError
    }
}

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
impl From<sqlx::error::Error> for ApiError {
    fn from(error: sqlx::error::Error) -> Self {
        tracing::error!("Database error: {:?}", error);

        ApiError::InternalServerError
    }
}

/// Automatically return `500 Internal Server Error` on a `redis::RedisError`.
///
/// Via the generated `From<redis::RedisError> for Error` impl,
/// this allows using `?` on database calls in handler functions without a manual mapping step.
///
/// The actual error message isn't returned to the client for security reasons.
/// It should be logged instead.
///
impl From<redis::RedisError> for ApiError {
    fn from(error: redis::RedisError) -> Self {
        tracing::error!("Redis error: {:?}", error);

        ApiError::InternalServerError
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

impl<T> ResultExt<T> for Result<T, sqlx::Error> {
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> ApiError,
    ) -> Result<T, ApiError> {
        self.map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some(name) => map_err(dbe),
            e => e.into(),
        })
    }
}
