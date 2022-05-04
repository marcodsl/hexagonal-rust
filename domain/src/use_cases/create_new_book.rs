use std::sync::{Arc, Mutex};

use serde::Deserialize;

use crate::{
    entities::Book,
    repositories::{BookRepository, BookRepositoryError},
};

pub struct CreateNewBookUseCase {
    repository: Arc<Mutex<dyn BookRepository>>,
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
    pub fn new(repository: Arc<Mutex<dyn BookRepository>>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, req: CreateNewUserRequest) -> Result<Book, CreateNewBookUseCaseError> {
        let repository = match self.repository.lock() {
            Ok(repository) => repository,
            _ => return Err(CreateNewBookUseCaseError::Unknown),
        };

        match repository.create(Book::new(req.name, req.author, req.page_count)) {
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
