use actix_web::{test, App, http::header::ContentType};

use crate::routes::home;

#[actix_web::test]
async fn test_index_get() {
    let app = test::init_service(App::new().service(home)).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}