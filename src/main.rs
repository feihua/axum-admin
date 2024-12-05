#[macro_use]
extern crate rbatis;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod middleware;

use axum::{middleware as md, routing::{get, post}, Router};

use std::sync::Arc;
use rbatis::RBatis;
use handler::system::{menu_handler, role_handler, user_handler};
use crate::model::db::init_db;
use middleware::auth::auth;

pub struct AppState {
    pub batis: RBatis,
}

#[tokio::main]
async fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    let rb = init_db().await;
    let shared_state = Arc::new(AppState { /* ... */ batis: rb.clone() });

    let app = Router::new()
        .nest("/api", Router::new()
            .route("/login", post(user_handler::login))
            .route("/query_user_role", post(user_handler::query_user_role))
            .route("/update_user_role", post(user_handler::update_user_role))
            .route("/query_user_menu", get(user_handler::query_user_menu))
            .route("/user_list", post(user_handler::user_list))
            .route("/user_save", post(user_handler::user_save))
            .route("/user_delete", post(user_handler::user_delete))
            .route("/user_update", post(user_handler::user_update))
            .route("/update_user_password", post(user_handler::update_user_password))
            .route("/query_role_menu", post(role_handler::query_role_menu))
            .route("/update_role_menu", post(role_handler::update_role_menu))
            .route("/role_list", post(role_handler::role_list))
            .route("/role_save", post(role_handler::role_save))
            .route("/role_delete", post(role_handler::role_delete))
            .route("/role_update", post(role_handler::role_update))
            .route("/menu_list", post(menu_handler::menu_list))
            .route("/menu_save", post(menu_handler::menu_save))
            .route("/menu_delete", post(menu_handler::menu_delete))
            .route("/menu_update", post(menu_handler::menu_update))
            .route_layer(md::from_fn(auth))
            .with_state(shared_state));


    // axum 0.6.x
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // log::info!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // axum 0.7.x
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

