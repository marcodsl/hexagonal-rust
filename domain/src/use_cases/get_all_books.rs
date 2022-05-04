use std::sync::Arc;

use crate::{entities::Book, repositories::BookRepository};

pub struct GetAllBooksUseCase {
    repository: Arc<dyn BookRepository>,
}

pub enum GetAllBooksUseCaseError {
    Unknown,
}

impl GetAllBooksUseCase {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> Result<Vec<Book>, GetAllBooksUseCaseError> {
        self.repository
            .read_all()
            .map_err(|_| GetAllBooksUseCaseError::Unknown)
    }
}
