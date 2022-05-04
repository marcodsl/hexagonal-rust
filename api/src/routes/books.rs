use actix_web::{
    error::ErrorBadRequest,
    web::{Data, Json},
    HttpResponse, Responder, Result,
};
use domain::use_cases::{CreateNewBookUseCase, CreateNewUserRequest, GetAllBooksUseCase};

pub async fn get_all_books(use_case: Data<GetAllBooksUseCase>) -> Result<impl Responder> {
    let books = use_case
        .execute()
        .map_err(|_| ErrorBadRequest("Bad Request"))?;

    Ok(HttpResponse::Ok().json(books))
}

pub async fn create_new_book(
    body: Json<CreateNewUserRequest>,
    use_case: Data<CreateNewBookUseCase>,
) -> Result<impl Responder> {
    let request = body.into_inner();

    let book = use_case
        .execute(request)
        .map_err(|_| ErrorBadRequest("Bad Request"))?;

    Ok(HttpResponse::Created().json(book))
}
