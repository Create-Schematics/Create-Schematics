use poem_openapi::payload::Json;
use sqlx::error::DatabaseError;

macro_rules! create_response_error {
    (
        $error_enum:ident,
        $(($error_status:literal, $error_name:ident)),*
    ) => {
        #[derive(Debug, poem_openapi::ApiResponse)]
        pub enum $error_enum {
            $(
            #[oai(status = $error_status)]
            $error_name,
            )*
        }

        impl From<sqlx::error::Error> for $error_enum {
            fn from(e: sqlx::error::Error) -> Self {
                tracing::error!("Database error: {}", e);
                Self::InternalServerError
            }
        }

        impl From<anyhow::Error> for $error_enum {
            fn from(e: anyhow::Error) -> Self {
                tracing::error!("Generic error: {}", e);
                Self::InternalServerError
            }
        }
    };
}

pub trait ResultExt<T, E> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> E,
    ) -> Result<T, E>;
}

impl <T> ResultExt<T, sqlx::error::Error> for Result<T, sqlx::error::Error> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> sqlx::error::Error,
    ) -> Result<T, sqlx::error::Error> {
        self.map_err(|e| match e.into() {
            sqlx::error::Error::Database(dbe) if dbe.constraint() == Some(name) => {
                f(dbe)
            }
            e => e
        })
    }
}

create_response_error!(
    FetchError,
    (404, NotFound),
    (500, InternalServerError)
);

pub type FetchResult<T> = Result<Json<T>, FetchError>;

create_response_error!(
    UploadError,
    (400, BadRequest),
    (401, Unauthorized),
    (403, Forbidden),
    (404, NotFound),
    (409, Conflict),
    (429, UnprocessableEntity),
    (500, InternalServerError)
);

pub type UploadResult<T> = Result<Json<T>, UploadError>;

create_response_error!(
    UpdateError,
    (400, BadRequest),
    (401, Unauthorized),
    (403, Forbidden),
    (404, NotFound),
    (429, UnprocessableEntity),
    (500, InternalServerError)
);

pub type UpdateResult<T> = Result<Json<T>, UpdateError>;

create_response_error!(
    DeleteError,
    (400, BadRequest),
    (401, Unauthorized),
    (403, Forbidden),
    (404, NotFound),
    (500, InternalServerError)
);

pub type DeleteResult = Result<(), DeleteError>;

