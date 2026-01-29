use ulid::Ulid;
use uuid::Uuid;

use crate::error::{Error, Result};

pub fn generate_id() -> Uuid {
    Uuid::from(Ulid::new())
}

pub fn parse_id(value: &str) -> Result<Uuid> {
    value
        .parse::<Ulid>()
        .map(Uuid::from)
        .or_else(|_| value.parse::<Uuid>())
        .map_err(|_| Error::InvalidIdFormat(value.to_string()))
}
