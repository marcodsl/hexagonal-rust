use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct Book {
    pub id: Uuid,
    pub name: String,
    pub author: String,
    pub page_count: u32,
}

impl Book {
    pub fn new(name: String, author: String, page_count: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            author,
            page_count,
        }
    }
}
