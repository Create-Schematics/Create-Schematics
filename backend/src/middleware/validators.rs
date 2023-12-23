use std::fmt::Display;

use poem_openapi::Validator;
use rustrict::{CensorStr, Type};

pub struct Profanity;

impl Display for Profanity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Profanity")
    }
}

impl<T> Validator<T> for Profanity 
where
    T: AsRef<str>
{
    #[inline]
    fn check(&self, value: &T) -> bool {
        !value.as_ref().is(Type::INAPPROPRIATE)
    }
}
