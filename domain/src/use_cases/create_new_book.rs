use std::sync::Arc;

use serde::Deserialize;

use crate::{
    entities::Book,
    repositories::{BookRepository, BookRepositoryError},
};

pub struct CreateNewBookUseCase {
    repository: Arc<dyn BookRepository>,
}

#[derive(Deserialize)]
pub struct CreateNewUserRequest {
    pub name: String,
    pub author: String,
    pub page_count: u32,
}

pub enum CreateNewBookUseCaseError {
    Conflict,
    Unknown,
}

impl CreateNewBookUseCase {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, req: CreateNewUserRequest) -> Result<Book, CreateNewBookUseCaseError> {
        match self
            .repository
            .create(Book::new(req.name, req.author, req.page_count))
        {
            Ok(book) => Ok(book),
            Err(error) => Err(error.into()),
        }
    }
}

impl From<BookRepositoryError> for CreateNewBookUseCaseError {
    fn from(error: BookRepositoryError) -> Self {
        match error {
            BookRepositoryError::Conflict => CreateNewBookUseCaseError::Conflict,
            BookRepositoryError::Unknown => CreateNewBookUseCaseError::Unknown,
        }
    }
}
