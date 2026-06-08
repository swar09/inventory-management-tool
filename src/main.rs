use axum::{Json, Router, routing::{Route, get}};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    check: String,
}

#[derive(Serialize)]
struct ResponseHomePage {
    message: String,
}


// All stock adjustments are logged

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .route("/api/health", get(health_check));
        /* 
        
        POST /login  user-login
        POST /vendor create-new-vendor
        GET /vendor/{id} get-vendor
        DELETE /vendor/{id} suspend-vendor
        PUT /vendor/{id} update-vendor-info

        

        
        
        
        
        */
        

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Health> {
    let heal = Health {
        check: String::from("ok"),
    };
    Json(heal)
}

async fn home() -> Json<ResponseHomePage> {
    let response = ResponseHomePage {
        message: String::from("Welcome to Inventory Management Tool"),
    };
    Json(response)
}

