use crate::entities::Book;

pub enum BookRepositoryError {
    Conflict,
    Unknown,
}

pub trait BookRepository: Send + Sync {
    fn create(&self, book: Book) -> Result<Book, BookRepositoryError>;
    fn read_all(&self) -> Result<Vec<Book>, BookRepositoryError>;
}
