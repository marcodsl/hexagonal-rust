use std::sync::{Arc, Mutex};

use crate::{entities::Book, repositories::BookRepository};

pub struct GetAllBooksUseCase {
    repository: Arc<Mutex<dyn BookRepository>>,
}

pub enum GetAllBooksUseCaseError {
    Unknown,
}

impl GetAllBooksUseCase {
    pub fn new(repository: Arc<Mutex<dyn BookRepository>>) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> Result<Vec<Book>, GetAllBooksUseCaseError> {
        let repository = match self.repository.lock() {
            Ok(repository) => repository,
            _ => return Err(GetAllBooksUseCaseError::Unknown),
        };

        repository
            .read_all()
            .map_err(|_| GetAllBooksUseCaseError::Unknown)
    }
}
