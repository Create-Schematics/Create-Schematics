use std::borrow::Cow;
use std::fmt::{Formatter, Debug};
use std::ops::Deref;

use poem::web::Field;
use poem_openapi::types::{ParseError, ParseFromMultipartField, ParseResult, Type};
use poem_openapi::registry::{MetaSchema, MetaSchemaRef};

pub struct FileUpload {
    pub file_name: String,
    pub content_type: Option<String>,
    pub contents: Vec<u8>,
}

impl Debug for FileUpload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Upload");
        
        d.field("filename", &self.file_name);
        
        if let Some(content_type) = &self.content_type {
            d.field("content_type", &content_type);
        }
        
        d.finish()
    }
}

impl FileUpload {
    pub fn file_name(&self) -> &str {
        &self.file_name

    }
}

impl Deref for FileUpload {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl Type for FileUpload {
    const IS_REQUIRED: bool = true;

    type RawValueType = Self;

    type RawElementValueType = Self;

    fn name() -> Cow<'static, str> {
        "string(binary)".into()
    }

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("string", "binary")))
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}

#[poem::async_trait]
impl ParseFromMultipartField for FileUpload {
    async fn parse_from_multipart(field: Option<Field>) -> ParseResult<Self> {
        match field {
            Some(field) => {
                let content_type = field.content_type().map(ToString::to_string);
                
                let file_name = field.file_name()
                    .map(sanitize_filename::sanitize)
                    .ok_or(ParseError::custom("files must be named"))?;

                Ok(Self {
                    content_type,
                    file_name,
                    contents: field.bytes().await.map_err(ParseError::custom)?
                })
            }
            None => Err(ParseError::expected_input()),
        }
    }
}
