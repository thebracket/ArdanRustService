use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{async_trait, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use service_framework::{load_config, ConfigFileType, Service};
use crate::librarian::db;
use crate::librarian::db::{Book, LibrarianDb, setup_database};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LibrarianConfig {
    #[serde(default)]
    pub database_url: String,
}

#[derive(Default)]
pub struct LibrarianService {}

#[async_trait]
impl Service for LibrarianService {
    async fn build_web_service(&self) -> anyhow::Result<Router> {
        let settings = load_config::<LibrarianConfig>(
            "LIBRARY",
            ConfigFileType::File {
                filename: String::from("library-config.json"),
                required: false,
            },
        )?;

        let db = setup_database(&settings).await?;

        let app = Router::new()
            .route("/", get(LibrarianService::get_all_books))
            .route("/:id", get(LibrarianService::get_book))
            .route("/add", post(LibrarianService::add_book))
            .route("/edit", put(LibrarianService::update_book))
            .route("/delete/:id", delete(LibrarianService::delete_book))
            .with_state(settings)
            .layer(Extension(db));

        Ok(app)
    }

    fn prefix(&self) -> String {
        String::from("/library/v1/")
    }
}

impl LibrarianService {
    pub async fn get_all_books(Extension(db): Extension<LibrarianDb>) -> Json<Vec<Book>> {
        let rows = db::all_books(&db).await.unwrap();
        Json(rows)
    }

    pub async fn get_book(
        Extension(db): Extension<LibrarianDb>,
        Path(id): Path<i32>,
    ) -> Json<Book> {
        let book = db::book_by_id(&db, id).await.unwrap();
        Json(book)
    }

    pub async fn add_book(
        Extension(db): Extension<LibrarianDb>,
        Json(book): Json<Book>,
    ) -> Result<Json<i32>, StatusCode> {
        if let Ok(id) = db::add_book(&db, book).await {
            Ok(Json(id))
        } else {
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }

    pub async fn update_book(
        Extension(db): Extension<LibrarianDb>,
        Json(book): Json<Book>,
    ) -> StatusCode {
        if db::update_book(&db, &book).await.is_ok() {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        }
    }

    pub async fn delete_book(
        Extension(db): Extension<LibrarianDb>,
        Path(id): Path<i32>,
    ) -> StatusCode {
        if db::delete_book(&db, id).await.is_ok() {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}

#[cfg(test)]
mod test {
    use axum::body::Body;
    use axum::extract::Request;
    use super::*;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
    use http_body_util::BodyExt; // for `collect`

    #[tokio::test]
    async fn get_all_books() {
        let app = LibrarianService::default().build_web_service().await.unwrap();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Vec<Book> = serde_json::from_slice(&body).unwrap();
        assert!(!body.is_empty());
    }
}