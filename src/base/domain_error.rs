#[derive(Debug)]
pub enum DomainError {
    AlreadyExists,
    Empty,
    Unauthorized,
    NotFound,
    Invalid(String),
    NonPositive
}

impl PartialEq for DomainError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DomainError::AlreadyExists, DomainError::AlreadyExists)
            | (DomainError::Empty, DomainError::Empty)
            | (DomainError::Unauthorized, DomainError::Unauthorized)
            | (DomainError::NotFound, DomainError::NotFound)
            | (DomainError::NonPositive, DomainError::NonPositive) => true,
            (DomainError::Invalid(msg1), DomainError::Invalid(msg2)) => msg1 == msg2,
            _ => false,
        }
    }
}