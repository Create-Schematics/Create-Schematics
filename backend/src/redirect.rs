use poem_openapi::ApiResponse;

#[derive(ApiResponse)]
pub enum RedirectResponse {
    #[oai(status = 302)]
    Found(#[oai(header = "location")] String)
}

impl RedirectResponse {
    pub fn to<T>(location: T) -> Self 
    where
        T: Into<String>
    {
        Self::Found(location.into())
    }
}