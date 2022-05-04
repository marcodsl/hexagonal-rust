use std::sync::Mutex;

use domain::{
    entities::Book,
    repositories::{BookRepository, BookRepositoryError},
};

pub struct InMemoryBookRepository {
    books: Mutex<Vec<Book>>,
}

impl InMemoryBookRepository {
    pub fn new() -> Self {
        let books = Mutex::new(Vec::new());

        Self { books }
    }
}

impl BookRepository for InMemoryBookRepository {
    fn create(&self, book: Book) -> Result<Book, BookRepositoryError> {
        let mut books = match self.books.lock() {
            Ok(books) => books,
            _ => return Err(BookRepositoryError::Unknown),
        };

        if books.iter().any(|b| b.id == book.id) {
            return Err(BookRepositoryError::Conflict);
        }

        books.push(book.clone());

        Ok(book)
    }

    fn read_all(&self) -> Result<Vec<Book>, BookRepositoryError> {
        let books = match self.books.lock() {
            Ok(books) => books,
            _ => return Err(BookRepositoryError::Unknown),
        };

        Ok(books.clone())
    }
}
