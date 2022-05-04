use std::sync::{Arc, Mutex};

use actix_web::{web::Data, App, HttpServer};
use domain::use_cases::{CreateNewBookUseCase, GetAllBooksUseCase};
use infrastructure::repositories::InMemoryBookRepository;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repository = Arc::new(Mutex::new(InMemoryBookRepository::new()));

    let create_new_book_use_case = Arc::new(CreateNewBookUseCase::new(repository.clone()));
    let get_all_books_use_case = Arc::new(GetAllBooksUseCase::new(repository.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(create_new_book_use_case.clone()))
            .app_data(Data::from(get_all_books_use_case.clone()))
            .configure(routes::routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
