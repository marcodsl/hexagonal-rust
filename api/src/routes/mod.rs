use actix_web::web::{self, ServiceConfig};

mod books;

pub fn routes(config: &mut ServiceConfig) {
    let books = web::scope("/books")
        .route("", web::get().to(books::get_all_books))
        .route("", web::post().to(books::create_new_book));

    config.service(books);
}
