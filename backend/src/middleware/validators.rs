use validator::ValidationError;
use rustrict::{CensorStr, Type};

pub fn profanity(value: &str) -> Result<(), ValidationError> {
    if !value.is(Type::INAPPROPRIATE) {
        Ok(())
    } else {
        Err(ValidationError::new("Schematics cannot contain profanity"))
    }
}